#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, SubMsg,
    WasmMsg,
};
use cw2::set_contract_version;
use cw_utils::nonpayable;

use axone_logic_bindings::LogicCustomQuery;
use axone_objectarium::msg::{
    ExecuteMsg as StorageMsg, ObjectPinsResponse, QueryMsg as StorageQuery,
};
use axone_objectarium_client::ObjectRef;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::INSTANTIATE_CONTEXT;

// version info for migration info
const CONTRACT_NAME: &str = concat!("crates.io:", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const STORE_PROGRAM_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<'_>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let store_msg = StorageMsg::StoreObject {
        data: msg.program.clone(),
        pin: true,
        compression_algorithm: None,
    };

    let store_program_msg = WasmMsg::Execute {
        contract_addr: msg.storage_address.clone(),
        msg: to_json_binary(&store_msg)?,
        funds: vec![],
    };

    INSTANTIATE_CONTEXT.save(deps.storage, &msg.storage_address)?;

    Ok(Response::new().add_submessage(SubMsg::reply_on_success(
        store_program_msg,
        STORE_PROGRAM_REPLY_ID,
    )))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<'_>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    nonpayable(&info)?;
    match msg {
        ExecuteMsg::BreakStone {} => execute::break_stone(deps, env, info),
    }
}

pub mod execute {
    use cosmwasm_std::{ensure_eq, Order};

    use crate::state::{DEPENDENCIES, PROGRAM};

    use super::*;

    pub fn break_stone(
        deps: DepsMut<'_>,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        ensure_eq!(
            deps.querier
                .query_wasm_contract_info(env.contract.address)?
                .creator,
            info.sender,
            ContractError::Unauthorized
        );

        let resp = Response::new().add_attribute("action", "break_stone");

        let mut stone = PROGRAM.load(deps.storage)?;
        if stone.broken {
            return Ok(resp);
        }
        stone.broken = true;
        PROGRAM.save(deps.storage, &stone)?;

        let law_release_msg = match deps
            .querier
            .query_wasm_smart::<ObjectPinsResponse>(
                stone.law.storage_address.clone(),
                &StorageQuery::ObjectPins {
                    id: stone.law.object_id.clone(),
                    first: Some(1u32),
                    after: None,
                },
            )?
            .page_info
            .has_next_page
        {
            true => stone.law.to_exec_unpin_msg(vec![]),
            _ => stone.law.to_exec_forget_msg(vec![]),
        }?;

        Ok(resp.add_message(law_release_msg).add_messages(
            DEPENDENCIES
                .range(deps.storage, None, None, Order::Ascending)
                .map(|res: StdResult<(String, ObjectRef)>| {
                    res.and_then(|(_, obj)| obj.to_exec_unpin_msg(vec![]))
                })
                .collect::<StdResult<Vec<WasmMsg>>>()?,
        ))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<'_>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Ask { query } => to_json_binary(&query::ask(deps, env, query)?),
        QueryMsg::Program {} => to_json_binary(&query::program(deps)?),
        QueryMsg::ProgramCode {} => to_json_binary(&query::program_code(deps)?),
    }
}

pub mod query {
    use axone_logic_bindings::{Answer, AskResponse};

    use crate::helper;
    use crate::helper::object_ref_to_uri;
    use crate::msg::ProgramResponse;
    use crate::state::PROGRAM;

    use super::*;

    const ERR_STONE_BROKEN: &str = "system_error(broken_law_stone)";

    pub fn program(deps: Deps<'_>) -> StdResult<ProgramResponse> {
        let program = PROGRAM.load(deps.storage)?.into();
        Ok(program)
    }

    pub fn program_code(deps: Deps<'_>) -> StdResult<Binary> {
        let ObjectRef {
            storage_address,
            object_id,
        } = PROGRAM.load(deps.storage)?.law;

        deps.querier.query_wasm_smart::<Binary>(
            storage_address,
            &StorageQuery::ObjectData { id: object_id },
        )
    }

    pub fn ask(deps: Deps<'_>, env: Env, query: String) -> StdResult<AskResponse> {
        let stone = PROGRAM.load(deps.storage)?;
        if stone.broken {
            return Ok(AskResponse {
                height: env.block.height,
                answer: Some(Answer::from_error(format!(
                    "error({},root)",
                    ERR_STONE_BROKEN
                ))),
                ..Default::default()
            });
        }

        let req = build_ask_query(stone.law, query)?.into();
        helper::query(&deps.querier, &req)
    }

    pub fn build_ask_query(program: ObjectRef, query: String) -> StdResult<LogicCustomQuery> {
        let program_uri = object_ref_to_uri(program)?;

        Ok(LogicCustomQuery::Ask {
            program: format!(":- consult('{}').", program_uri),
            query,
        })
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut<'_>, env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        STORE_PROGRAM_REPLY_ID => reply::store_program_reply(deps, env, msg),
        _ => Err(ContractError::UnknownReplyID),
    }
}

pub mod reply {
    use cw_utils::ParseReplyError;

    use crate::helper;
    use crate::helper::{ask_response_to_objects, get_reply_event_attribute, object_ref_to_uri};
    use crate::state::{LawStone, DEPENDENCIES, PROGRAM};

    use super::*;

    pub fn store_program_reply(
        deps: DepsMut<'_>,
        _env: Env,
        msg: Reply,
    ) -> Result<Response, ContractError> {
        let context = INSTANTIATE_CONTEXT.load(deps.storage)?;

        msg.result
            .into_result()
            .map_err(ParseReplyError::SubMsgFailure)
            .map_err(Into::into)
            .and_then(|e| {
                get_reply_event_attribute(&e.events, "id").ok_or_else(|| {
                    ParseReplyError::SubMsgFailure(
                        "reply event doesn't contains object id".to_string(),
                    )
                    .into()
                })
            })
            .map(|obj_id| LawStone {
                broken: false,
                law: ObjectRef {
                    object_id: obj_id,
                    storage_address: context.clone(),
                },
            })
            .and_then(|stone| -> Result<Vec<SubMsg>, ContractError> {
                PROGRAM
                    .save(deps.storage, &stone)
                    .map_err(ContractError::from)?;

                // Clean instantiate context
                INSTANTIATE_CONTEXT.remove(deps.storage);

                let req = build_source_files_query(stone.law.clone())?.into();
                let res = helper::query(&deps.querier, &req).map_err(ContractError::from)?;

                let objects = ask_response_to_objects(res, "Files".to_string())?;
                let mut msgs = Vec::with_capacity(objects.len());
                for obj in objects {
                    if obj.object_id == stone.law.object_id {
                        continue;
                    }
                    DEPENDENCIES.save(deps.storage, obj.object_id.as_str(), &obj)?;

                    msgs.push(SubMsg::new(obj.to_exec_pin_msg(vec![])?));
                }

                Ok(msgs)
            })
            .map(|msg| Response::new().add_submessages(msg))
    }

    pub fn build_source_files_query(program: ObjectRef) -> StdResult<LogicCustomQuery> {
        let program_uri = object_ref_to_uri(program)?.to_string();

        Ok(LogicCustomQuery::Ask {
            program: "source_files(Files) :- bagof(File, source_file(File), Files).".to_string(),
            query: [
                "consult('",
                program_uri.as_str(),
                "'), source_files(Files).",
            ]
            .join(""),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::marker::PhantomData;

    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier,
        MockQuerierCustomHandlerResult, MockStorage,
    };
    use cosmwasm_std::{
        coins, from_json, to_json_binary, ContractInfoResponse, ContractResult, CosmosMsg, Event,
        Order, OwnedDeps, SubMsgResponse, SubMsgResult, SystemError, SystemResult, WasmQuery,
    };
    use cw_utils::ParseReplyError::SubMsgFailure;
    use cw_utils::PaymentError;
    use cw_utils::PaymentError::NonPayable;

    use axone_logic_bindings::testing::mock::mock_dependencies_with_custom_handler;
    use axone_logic_bindings::{
        Answer, AskResponse, LogicCustomQuery, Result as LogicResult, Substitution,
    };
    use axone_objectarium::msg::PageInfo;
    use axone_wasm::uri::CosmwasmUri;

    use crate::msg::ProgramResponse;
    use crate::state::{LawStone, DEPENDENCIES, PROGRAM};

    use super::*;

    fn custom_logic_handler_with_dependencies(
        dependencies: Vec<String>,
        program: ObjectRef,
        request: &LogicCustomQuery,
    ) -> MockQuerierCustomHandlerResult {
        let mut updated_deps = dependencies;
        updated_deps.push(CosmwasmUri::try_from(program.clone()).unwrap().to_string());
        let deps_name = format!("[{}]", &updated_deps.join(","));
        let LogicCustomQuery::Ask {
            program: exp_program,
            query: exp_query,
            ..
        } = reply::build_source_files_query(program).unwrap();
        match request {
            LogicCustomQuery::Ask { program, query }
                if *query == exp_query && *program == exp_program =>
            {
                SystemResult::Ok(
                    to_json_binary(&AskResponse {
                        height: 1,
                        gas_used: 1000,
                        answer: Some(Answer {
                            has_more: false,
                            variables: vec!["Files".to_string()],
                            results: vec![LogicResult {
                                error: None,
                                substitutions: vec![Substitution {
                                    variable: "Files".to_string(),
                                    expression: deps_name,
                                }],
                            }],
                        }),
                        user_output: None,
                    })
                    .into(),
                )
            }
            _ => SystemResult::Err(SystemError::InvalidRequest {
                error: "Ask `souces_files(Files).` predicate not called".to_string(),
                request: Default::default(),
            }),
        }
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_custom_handler(|_: &LogicCustomQuery| {
            SystemResult::Err(SystemError::Unknown {})
        });

        let program = to_json_binary("foo(_) :- true.").unwrap();

        let msg = InstantiateMsg {
            program: program.clone(),
            storage_address: "axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv"
                .to_string(),
        };
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut().into_empty(), mock_env(), info, msg).unwrap();

        // Check if a message is send to the axone-objectarium to store the logic program.
        assert_eq!(1, res.messages.len());
        let sub_msg = res.messages.first().unwrap();
        assert_eq!(STORE_PROGRAM_REPLY_ID, sub_msg.id);
        match &sub_msg.msg {
            CosmosMsg::Wasm(wasm_msg) => match wasm_msg {
                WasmMsg::Execute { msg, .. } => {
                    let result: StorageMsg = from_json(msg).unwrap();
                    match result {
                        StorageMsg::StoreObject {
                            data,
                            pin,
                            compression_algorithm,
                        } => {
                            assert_eq!(data, program);
                            assert!(pin, "the main program should be pinned");
                            assert_eq!(compression_algorithm, None);
                        }
                        _ => panic!("storage message should be a StoreObject message"),
                    }
                }
                _ => panic!("wasm message should be a Storage message"),
            },
            _ => panic!("cosmos sub message should be a Wasm message execute"),
        }
        assert_eq!(
            "axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv".to_string(),
            INSTANTIATE_CONTEXT.load(&deps.storage).unwrap()
        );
    }

    #[test]
    fn funds_initialization() {
        let mut deps = mock_dependencies_with_custom_handler(|_: &LogicCustomQuery| {
            SystemResult::Err(SystemError::Unknown {})
        });
        let env = mock_env();
        let info = mock_info("sender", &coins(10, "uaxone"));

        let msg = InstantiateMsg {
            program: to_json_binary("foo(_) :- true.").unwrap(),
            storage_address: "axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv"
                .to_string(),
        };

        let result = instantiate(deps.as_mut().into_empty(), env, info, msg);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ContractError::Payment(NonPayable {}));
    }

    #[test]
    fn program() {
        let mut deps = mock_dependencies_with_custom_handler(|_: &LogicCustomQuery| {
            SystemResult::Err(SystemError::Unknown {})
        });

        let object_id =
            "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05".to_string();
        let storage_addr =
            "axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv".to_string();
        PROGRAM
            .save(
                deps.as_mut().storage,
                &LawStone {
                    broken: false,
                    law: ObjectRef {
                        object_id: object_id.clone(),
                        storage_address: storage_addr.clone(),
                    },
                },
            )
            .unwrap();

        let res = query(deps.as_ref().into_empty(), mock_env(), QueryMsg::Program {}).unwrap();
        let result: ProgramResponse = from_json(&res).unwrap();

        assert_eq!(object_id, result.object_id);
        assert_eq!(storage_addr, result.storage_address);
    }

    #[test]
    fn program_code() {
        const CONTRACT_ID: &str =
            "axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv";
        const OBJECT_ID: &str = "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05";
        const A_PROGRAM: &str = "foo(_) :- true.";

        let mut deps = mock_dependencies_with_custom_handler(|_: &LogicCustomQuery| {
            SystemResult::Err(SystemError::Unknown {})
        });
        deps.querier.update_wasm(move |query| match query {
            WasmQuery::Smart { contract_addr, msg } if contract_addr == CONTRACT_ID => {
                let data = to_json_binary(&A_PROGRAM).unwrap();
                let storage_query: StorageQuery = from_json(msg).unwrap();

                assert!(
                    matches!(storage_query, StorageQuery::ObjectData { id } if id == OBJECT_ID)
                );

                SystemResult::Ok(ContractResult::Ok(to_json_binary(&data).unwrap()))
            }
            _ => {
                panic!("UnsupportedRequest: query_wasm");
            }
        });

        PROGRAM
            .save(
                deps.as_mut().storage,
                &LawStone {
                    broken: false,
                    law: ObjectRef {
                        object_id: OBJECT_ID.to_string(),
                        storage_address: CONTRACT_ID.to_string(),
                    },
                },
            )
            .unwrap();

        let result = query(
            deps.as_ref().into_empty(),
            mock_env(),
            QueryMsg::ProgramCode {},
        )
        .unwrap();
        let data: Binary = from_json(&result).unwrap();
        let program: String = from_json(&data).unwrap();

        assert_eq!(A_PROGRAM, program);
    }

    fn custom_logic_handler_with_query(
        env: &Env,
        query: String,
        program: ObjectRef,
        request: &LogicCustomQuery,
    ) -> MockQuerierCustomHandlerResult {
        let LogicCustomQuery::Ask {
            program: exp_program,
            query: exp_query,
            ..
        } = query::build_ask_query(program, query.to_string()).unwrap();
        match request {
            LogicCustomQuery::Ask {
                program,
                query: queryy,
            } if *queryy == exp_query && *program == exp_program => SystemResult::Ok(
                to_json_binary(&AskResponse {
                    height: env.block.height,
                    gas_used: 1000,
                    answer: Some(Answer {
                        has_more: false,
                        variables: vec!["Foo".to_string()],
                        results: vec![LogicResult {
                            error: None,
                            substitutions: vec![Substitution {
                                variable: "Foo".to_string(),
                                expression: "bar".to_string(),
                            }],
                        }],
                    }),
                    user_output: None,
                })
                .into(),
            ),
            _ => SystemResult::Err(SystemError::InvalidRequest {
                error: format!("Ask `{query}` predicate not called"),
                request: Default::default(),
            }),
        }
    }

    #[test]
    fn ask() {
        let cases = vec![
            (
                false,                    // broken
                "test(Foo).".to_string(), // query
                ObjectRef {
                    object_id: "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05"
                        .to_string(),
                    storage_address:
                        "axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv"
                            .to_string(),
                },
                Some(AskResponse {
                    height: 12345,
                    gas_used: 1000,
                    answer: Some(Answer {
                        variables: vec!["Foo".to_string()],
                        results: vec![axone_logic_bindings::Result {
                            substitutions: vec![Substitution {
                                variable: "Foo".to_string(),
                                expression: "bar".to_string(),
                            }],
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                None, // Expected error
            ),
            (
                true,                     // broken
                "test(Foo).".to_string(), // query
                ObjectRef {
                    object_id: "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05"
                        .to_string(),
                    storage_address:
                        "axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv"
                            .to_string(),
                },
                Some(AskResponse {
                    height: 12345,
                    answer: Some(Answer {
                        results: vec![axone_logic_bindings::Result {
                            error: Some("error(system_error(broken_law_stone),root)".to_string()),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                None, // Expected error
            ),
        ];

        for case in cases {
            let p = Box::new((
                case.1.clone(),
                case.2.object_id.to_string(),
                case.2.storage_address.to_string(),
            ));
            let env = mock_env();
            let env_4_closure = env.clone();
            let mut deps = mock_dependencies_with_custom_handler(move |request| {
                let (query, o, s) = p.as_ref();
                custom_logic_handler_with_query(
                    &env_4_closure,
                    query.to_string(),
                    ObjectRef {
                        object_id: o.to_string(),
                        storage_address: s.to_string(),
                    },
                    request,
                )
            });

            PROGRAM
                .save(
                    deps.as_mut().storage,
                    &LawStone {
                        broken: case.0,
                        law: case.2.clone(),
                    },
                )
                .unwrap();

            let res = query(
                deps.as_ref().into_empty(),
                env,
                QueryMsg::Ask { query: case.1 },
            );

            match res {
                Ok(result) => {
                    let result: AskResponse = from_json(&result).unwrap();

                    assert!(case.3.is_some());
                    assert!(result.answer.is_some());
                    assert_eq!(result, case.3.unwrap());
                    assert!(case.4.is_none(), "query doesn't return error")
                }
                Err(e) => {
                    assert!(case.4.is_some(), "query return error");
                    assert_eq!(e, case.4.unwrap())
                }
            }
        }
    }

    #[derive(Clone)]
    struct StoreTestCase {
        dependencies: Vec<(String, String, String)>, // URI, contract address, object id
        object_id: String,
    }

    #[test]
    fn store_program_reply() {
        let cases = vec![
            StoreTestCase {
                dependencies: vec![
                    (
                        "cosmwasm:axone1dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqen9apd?query=%7B%22object_data%22%3A%7B%22id%22%3A%20%224cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05%22%7D%7D".to_string(),
                        "axone1dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqen9apd".to_string(),
                        "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05".to_string()
                    ),
                ],
                object_id: "0689c526187c6785dfcce28f8df19138da292598dc19548a852de1792062f271"
                    .to_string(),
            },
            StoreTestCase {
                dependencies: vec![],
                object_id: "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05"
                    .to_string(),
            },
            StoreTestCase {
                dependencies: vec![
                    (
                        "cosmwasm:axone1dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqen9apd?query=%7B%22object_data%22%3A%7B%22id%22%3A%20%224cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05%22%7D%7D".to_string(),
                        "axone1dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqen9apd".to_string(), // contract addr
                        "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05".to_string() // object id
                    ),
                    (
                        "cosmwasm:axone1dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqen9apd?query=%7B%22object_data%22%3A%7B%22id%22%3A%20%220689c526187c6785dfcce28f8df19138da292598dc19548a852de1792062f271%22%7D%7D".to_string(),
                        "axone1dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqen9apd".to_string(), // contract addr
                        "0689c526187c6785dfcce28f8df19138da292598dc19548a852de1792062f271".to_string() // object id
                    ),
                ],
                object_id: "1cc6de7672c97db145a3940df2264140ea893c6688fa5ca55b73cb8b68e0574d"
                    .to_string(),
            },
        ];

        for case in cases {
            let uris = Box::new(
                case.dependencies
                    .clone()
                    .into_iter()
                    .map(|(uri, _, _)| uri)
                    .collect::<Vec<String>>(),
            );
            let program_object_id = case.clone().object_id;
            let mut deps = mock_dependencies_with_custom_handler(move |request| {
                custom_logic_handler_with_dependencies(
                    uris.to_vec(),
                    ObjectRef {
                        object_id: program_object_id.clone(),
                        storage_address:
                            "axone1dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqen9apd"
                                .to_string(),
                    },
                    request,
                )
            });

            let reply = Reply {
                id: STORE_PROGRAM_REPLY_ID,
                result: SubMsgResult::Ok(SubMsgResponse {
                    events: vec![Event::new("e".to_string())
                        .add_attribute("id".to_string(), case.clone().object_id)],
                    data: None,
                }),
            };

            // Configure the instantiate context
            INSTANTIATE_CONTEXT
                .save(
                    deps.as_mut().storage,
                    &"axone1dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqen9apd".to_string(),
                )
                .unwrap();

            let response =
                reply::store_program_reply(deps.as_mut().into_empty(), mock_env(), reply);
            let res = response.unwrap();

            let program = PROGRAM.load(&deps.storage).unwrap();
            assert!(!program.broken);
            assert_eq!(case.clone().object_id, program.law.object_id);

            let deps_len_requirement = case.clone().dependencies.len();

            if deps_len_requirement > 0 {
                assert_eq!(
                    deps_len_requirement,
                    DEPENDENCIES
                        .keys_raw(&deps.storage, None, None, Order::Ascending)
                        .count()
                );
                for (_, contract_addr, object_id) in case.clone().dependencies {
                    let o = DEPENDENCIES.load(&deps.storage, object_id.as_str());
                    assert!(
                        o.is_ok(),
                        "dependencies should contains each object id dependencies as key"
                    );
                    let o = o.unwrap();
                    assert_eq!(
                        o.object_id, object_id,
                        "dependencies should contains each object id dependencies as key"
                    );
                    assert_eq!(
                        o.storage_address, contract_addr,
                        "dependencies should contains each object id dependencies as key"
                    );
                }
            }

            assert_eq!(
                deps_len_requirement,
                res.messages.len(),
                "response should contains any sub message as dependencies"
            );

            let objects_pinned: Vec<String> = res
                .messages
                .into_iter()
                .flat_map(|sub_msg| -> Option<String> {
                    match &sub_msg.msg {
                        CosmosMsg::Wasm(wasm_msg) => match wasm_msg {
                            WasmMsg::Execute { msg, .. } => {
                                let result: StorageMsg = from_json(msg).unwrap();
                                match result {
                                    StorageMsg::PinObject { id } => Some(id),
                                    _ => panic!("should contains only PinObject message(s)"),
                                }
                            }
                            _ => panic!("wasm message should be a Storage message"),
                        },
                        _ => panic!("cosmos sub message should be a Wasm message execute"),
                    }
                })
                .collect();

            for object in objects_pinned {
                assert!(
                    DEPENDENCIES.has(&deps.storage, object.as_str()),
                    "each dependencies should be pinned by a PinObject message"
                )
            }

            assert!(
                INSTANTIATE_CONTEXT.load(&deps.storage).is_err(),
                "the instantiate context should be cleaned at the end"
            )
        }
    }

    #[test]
    fn program_reply_errors() {
        let object_id = "axone1dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqen9apd";
        let cases = vec![
            (
                Reply {
                    id: 404,
                    result: SubMsgResult::Ok(SubMsgResponse {
                        events: vec![Event::new("e".to_string())
                            .add_attribute("id".to_string(), object_id.to_string())],
                        data: None,
                    }),
                },
                Err(ContractError::UnknownReplyID),
            ),
            (
                Reply {
                    id: 1,
                    result: SubMsgResult::Ok(SubMsgResponse {
                        events: vec![Event::new("e".to_string())],
                        data: None,
                    }),
                },
                Err(ContractError::ParseReplyError(SubMsgFailure(
                    "reply event doesn't contains object id".to_string(),
                ))),
            ),
        ];

        for case in cases {
            let mut deps = OwnedDeps {
                storage: MockStorage::default(),
                api: MockApi::default(),
                querier: MockQuerier::default(),
                custom_query_type: PhantomData,
            };

            INSTANTIATE_CONTEXT
                .save(
                    deps.as_mut().storage,
                    &"axone1dclchlcttf2uektxyryg0c6yau63eml5q9uq03myg44ml8cxpxnqen9apd".to_string(),
                )
                .unwrap();

            let response = reply(deps.as_mut(), mock_env(), case.0);

            assert_eq!(response, case.1);
        }
    }

    #[test]
    fn build_source_files_query() {
        let result = reply::build_source_files_query(ObjectRef {
            object_id: "1cc6de7672c97db145a3940df2264140ea893c6688fa5ca55b73cb8b68e0574d"
                .to_string(),
            storage_address: "axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv"
                .to_string(),
        });

        match result {
            Ok(LogicCustomQuery::Ask { program, query }) => {
                assert_eq!(
                    program,
                    "source_files(Files) :- bagof(File, source_file(File), Files)."
                );
                assert_eq!(query, "consult('cosmwasm:axone-objectarium:axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv?query=%7B%22object_data%22%3A%7B%22id%22%3A%221cc6de7672c97db145a3940df2264140ea893c6688fa5ca55b73cb8b68e0574d%22%7D%7D'), source_files(Files).")
            }
            _ => panic!("Expected Ok(LogicCustomQuery)."),
        }
    }

    #[test]
    fn build_ask_query() {
        let result = query::build_ask_query(
            ObjectRef {
                object_id: "1cc6de7672c97db145a3940df2264140ea893c6688fa5ca55b73cb8b68e0574d"
                    .to_string(),
                storage_address: "axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv"
                    .to_string(),
            },
            "test(X).".to_string(),
        );

        match result {
            Ok(LogicCustomQuery::Ask { program, query }) => {
                assert_eq!(program, ":- consult('cosmwasm:axone-objectarium:axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv?query=%7B%22object_data%22%3A%7B%22id%22%3A%221cc6de7672c97db145a3940df2264140ea893c6688fa5ca55b73cb8b68e0574d%22%7D%7D').");
                assert_eq!(query, "test(X).")
            }
            _ => panic!("Expected Ok(LogicCustomQuery)."),
        }
    }

    #[test]
    fn execute_fail_with_funds() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &coins(10, "uaxone"));

        let result = execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::BreakStone {},
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ContractError::Payment(PaymentError::NonPayable {})
        );
    }

    #[test]
    fn break_stone() {
        let cases = vec![
            (2, vec![]),
            (1, vec![]),
            (
                1,
                vec![ObjectRef {
                    storage_address: "addr1".to_string(),
                    object_id: "object1".to_string(),
                }],
            ),
            (
                3,
                vec![
                    ObjectRef {
                        storage_address: "addr1".to_string(),
                        object_id: "object1".to_string(),
                    },
                    ObjectRef {
                        storage_address: "addr2".to_string(),
                        object_id: "object2".to_string(),
                    },
                ],
            ),
        ];

        for case in cases {
            let mut deps = mock_dependencies();
            deps.querier.update_wasm(move |req| match req {
                WasmQuery::ContractInfo { .. } => {
                    let mut contract_info = ContractInfoResponse::default();
                    contract_info.creator = "creator".to_string();
                    SystemResult::Ok(ContractResult::Ok(to_json_binary(&contract_info).unwrap()))
                }
                WasmQuery::Smart { contract_addr, msg }
                    if contract_addr == "axone-objectarium1" =>
                {
                    match from_json(msg) {
                        Ok(StorageQuery::ObjectPins {
                            id,
                            first: Some(1u32),
                            after: None,
                        }) if id == "program-id" => SystemResult::Ok(ContractResult::Ok(
                            to_json_binary(&ObjectPinsResponse {
                                data: vec!["creator".to_string()],
                                page_info: PageInfo {
                                    has_next_page: case.0 > 1,
                                    cursor: "".to_string(),
                                },
                            })
                            .unwrap(),
                        )),
                        _ => SystemResult::Err(SystemError::Unknown {}),
                    }
                }
                _ => SystemResult::Err(SystemError::Unknown {}),
            });

            PROGRAM
                .save(
                    &mut deps.storage,
                    &LawStone {
                        broken: false,
                        law: ObjectRef {
                            object_id: "program-id".to_string(),
                            storage_address: "axone-objectarium1".to_string(),
                        },
                    },
                )
                .unwrap();
            for dep in case.1.clone() {
                let mut id = dep.storage_address.to_owned();
                id.push_str(dep.object_id.as_str());
                DEPENDENCIES
                    .save(&mut deps.storage, id.as_str(), &dep.clone())
                    .unwrap();
            }

            let info = mock_info("creator", &[]);
            let res = execute(
                deps.as_mut(),
                mock_env(),
                info.clone(),
                ExecuteMsg::BreakStone {},
            )
            .unwrap();

            assert!(PROGRAM.load(&deps.storage).unwrap().broken);

            let mut sub_msgs: VecDeque<SubMsg> = res.messages.into();
            match sub_msgs.pop_front() {
                Some(SubMsg {
                    msg: cosmos_msg, ..
                }) => match cosmos_msg {
                    CosmosMsg::Wasm(WasmMsg::Execute {
                        contract_addr, msg, ..
                    }) => {
                        assert_eq!(contract_addr, "axone-objectarium1".to_string());
                        if case.0 > 1 {
                            match from_json(&msg) {
                                Ok(StorageMsg::UnpinObject { id }) => {
                                    assert_eq!(id, "program-id".to_string());
                                }
                                _ => panic!("storage message should be a UnpinObject message"),
                            }
                        } else {
                            match from_json(&msg) {
                                Ok(StorageMsg::ForgetObject { id }) => {
                                    assert_eq!(id, "program-id".to_string());
                                }
                                _ => panic!("storage message should be a ForgetObject message"),
                            }
                        }
                    }
                    _ => panic!("sub message should be a WasmMsg message"),
                },
                _ => panic!("result should contains sub messages"),
            }

            for dep in case.1 {
                match sub_msgs.pop_front() {
                    Some(SubMsg {
                        msg: cosmos_msg, ..
                    }) => match cosmos_msg {
                        CosmosMsg::Wasm(WasmMsg::Execute {
                            contract_addr, msg, ..
                        }) => {
                            assert_eq!(contract_addr, dep.storage_address);
                            match from_json(&msg) {
                                Ok(StorageMsg::UnpinObject { id }) => {
                                    assert_eq!(id, dep.object_id);
                                }
                                _ => panic!("storage message should be a UnpinObject message"),
                            }
                        }
                        _ => panic!("sub message should be a WasmMsg message"),
                    },
                    _ => panic!("result should contains sub messages"),
                }
            }
        }
    }

    #[test]
    fn break_stone_creator() {
        let cases = vec![
            // creator, sender, broken, Error
            (
                "creator",
                "sender",
                false,
                Some(ContractError::Unauthorized),
            ),
            ("creator", "sender", true, Some(ContractError::Unauthorized)),
            ("creator", "creator", false, None),
            ("creator", "creator", true, None),
        ];

        for case in cases {
            let mut deps = mock_dependencies();

            deps.querier.update_wasm(move |req| match req {
                WasmQuery::ContractInfo { .. } => {
                    let mut contract_info = ContractInfoResponse::default();
                    contract_info.creator = case.0.to_string();

                    SystemResult::Ok(ContractResult::Ok(to_json_binary(&contract_info).unwrap()))
                }
                WasmQuery::Smart { .. } => SystemResult::Ok(ContractResult::Ok(
                    to_json_binary(&ObjectPinsResponse {
                        data: vec![case.1.to_string()],
                        page_info: PageInfo {
                            has_next_page: false,
                            cursor: "".to_string(),
                        },
                    })
                    .unwrap(),
                )),
                _ => SystemResult::Err(SystemError::Unknown {}),
            });

            PROGRAM
                .save(
                    &mut deps.storage,
                    &LawStone {
                        broken: case.2,
                        law: ObjectRef {
                            object_id: "id".to_string(),
                            storage_address: "addr".to_string(),
                        },
                    },
                )
                .unwrap();

            let res = execute(
                deps.as_mut(),
                mock_env(),
                mock_info(case.1, &[]),
                ExecuteMsg::BreakStone {},
            );

            match case.3 {
                Some(err) => {
                    assert!(res.is_err());
                    assert_eq!(res.err().unwrap(), err);
                }
                None => assert!(res.is_ok()),
            };
        }
    }

    #[test]
    fn break_broken_stone() {
        let mut deps = mock_dependencies();
        deps.querier.update_wasm(|req| match req {
            WasmQuery::ContractInfo { .. } => {
                let mut contract_info = ContractInfoResponse::default();
                contract_info.creator = "creator".to_string();
                SystemResult::Ok(ContractResult::Ok(to_json_binary(&contract_info).unwrap()))
            }
            _ => SystemResult::Err(SystemError::Unknown {}),
        });

        PROGRAM
            .save(
                &mut deps.storage,
                &LawStone {
                    broken: true,
                    law: ObjectRef {
                        object_id: "id".to_string(),
                        storage_address: "addr".to_string(),
                    },
                },
            )
            .unwrap();
        DEPENDENCIES
            .save(
                &mut deps.storage,
                "id",
                &ObjectRef {
                    object_id: "id2".to_string(),
                    storage_address: "addr2".to_string(),
                },
            )
            .unwrap();

        let res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("creator", &[]),
            ExecuteMsg::BreakStone {},
        );
        assert!(res.is_ok());
        assert_eq!(res.ok().unwrap().messages.len(), 0);
    }
}
