use crate::error::LogicAskResponseError;
use crate::ContractError;
use axone_logic_bindings::{AskResponse, TermValue};
use axone_objectarium_client::ObjectRef;
use axone_wasm::error::CosmwasmUriError;
use axone_wasm::uri::CosmwasmUri;
use cosmwasm_std::{
    from_json, to_json_vec, ContractResult, Event, QuerierWrapper, QueryRequest, StdError,
    StdResult, SystemResult,
};
use itertools::Itertools;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::any::type_name;

pub fn object_ref_to_uri(object: ObjectRef) -> StdResult<CosmwasmUri> {
    CosmwasmUri::try_from(object).map_err(|e: CosmwasmUriError| {
        StdError::parse_err(type_name::<CosmwasmUri>(), e.to_string())
    })
}

pub fn get_reply_event_attribute(events: &[Event], key: &str) -> Option<String> {
    events
        .iter()
        .flat_map(|e| e.attributes.iter())
        .find(|a| a.key == key)
        .map(|a| a.value.clone())
}

fn term_as_vec(term: TermValue) -> Result<Vec<String>, ContractError> {
    match term {
        TermValue::Array(values) => values
            .iter()
            .map(|v| -> Result<String, ContractError> {
                match v {
                    TermValue::Value(str) => Ok(str.clone()),
                    _ => Err(ContractError::LogicAskResponse(
                        LogicAskResponseError::UnexpectedTerm,
                    )),
                }
            })
            .collect(),
        _ => Err(ContractError::LogicAskResponse(
            LogicAskResponseError::UnexpectedTerm,
        )),
    }
}

/// Extract the substitution of a specified variable, assuming a single result containing a single substitution in the
/// response. The substitution is then parsed as an array of [CosmwasmUri], returning their [ObjectRef] representation.
pub fn ask_response_to_objects(
    res: AskResponse,
    variable: String,
) -> Result<Vec<ObjectRef>, ContractError> {
    let result = res
        .answer
        .map(|a| a.results)
        .unwrap_or_default()
        .into_iter()
        .exactly_one()
        .map_err(|_| {
            ContractError::LogicAskResponse(LogicAskResponseError::Unexpected(
                "expected exactly one result".to_string(),
            ))
        })?;

    if let Some(e) = result.error {
        return Err(ContractError::LogicAskResponse(
            LogicAskResponseError::Substitution(e),
        ));
    }

    let substitution = result
        .substitutions
        .into_iter()
        .filter(|s| s.variable == variable)
        .exactly_one()
        .map_err(|_| {
            ContractError::LogicAskResponse(LogicAskResponseError::Unexpected(
                "expected exactly one substitution".to_string(),
            ))
        })?;

    substitution
        .parse_expression()
        .map_err(|e| ContractError::LogicAskResponse(LogicAskResponseError::Parse(e)))
        .and_then(term_as_vec)?
        .into_iter()
        .map(|raw| {
            CosmwasmUri::try_from(raw)
                .and_then(ObjectRef::try_from)
                .map_err(ContractError::ParseCosmwasmUri)
        })
        .collect()
}

pub fn query<U: DeserializeOwned, C: Serialize>(
    q: &QuerierWrapper<'_>,
    request: &QueryRequest<C>,
) -> StdResult<U> {
    let raw = to_json_vec(request).map_err(|serialize_err| {
        StdError::generic_err(format!("Serializing QueryRequest: {serialize_err}"))
    })?;
    match q.raw_query(&raw) {
        SystemResult::Err(system_err) => Err(StdError::generic_err(format!(
            "Querier system error: {system_err}"
        ))),
        SystemResult::Ok(ContractResult::Err(contract_err)) => Err(StdError::generic_err(format!(
            "Querier contract error: {contract_err}"
        ))),
        SystemResult::Ok(ContractResult::Ok(value)) => from_json(value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axone_logic_bindings::error::TermParseError;
    use axone_logic_bindings::{Answer, Substitution};

    #[test]
    fn logic_to_objects() {
        let cases = vec![
            (
                vec![axone_logic_bindings::Result {
                    error: None,
                    substitutions: vec![Substitution {
                        variable: "X".to_string(),
                        expression: "[]".to_string(),
                    }]
                }],
                Ok(vec![])
            ),
            (
                vec![axone_logic_bindings::Result {
                    error: None,
                    substitutions: vec![Substitution {
                        variable: "X".to_string(),
                        expression: "['cosmwasm:axone-objectarium:axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv?query=%7B%22object_data%22%3A%7B%22id%22%3A%224cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05%22%7D%7D']".to_string(),
                    }]
                }],
                Ok(vec![ObjectRef{
                    object_id: "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05".to_string(),
                    storage_address: "axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv".to_string(),
                }])
            ),
            (
                vec![axone_logic_bindings::Result {
                    error: None,
                    substitutions: vec![Substitution {
                        variable: "X".to_string(),
                        expression: "['cosmwasm:axone-objectarium:axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv?query=%7B%22object_data%22%3A%7B%22id%22%3A%224cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05%22%7D%7D','cosmwasm:axone-objectarium:axone1cxmx7su8h5pvqca85cxdylz86uj9x9gu5xuqv34kw87q5x0hexdsr3g4x4?query=%7B%22object_data%22%3A%7B%22id%22%3A%221485133dd3ab4b1c4b8085e7265585f91ae3cca0996a39e0377a1059296f6aa7%22%7D%7D']".to_string(),
                    }]
                }],
                Ok(vec![ObjectRef{
                    object_id: "4cbe36399aabfcc7158ee7a66cbfffa525bb0ceab33d1ff2cff08759fe0a9b05".to_string(),
                    storage_address: "axone1ffzp0xmjhwkltuxcvccl0z9tyfuu7txp5ke0tpkcjpzuq9fcj3pq85yqlv".to_string(),
                },ObjectRef{
                    object_id: "1485133dd3ab4b1c4b8085e7265585f91ae3cca0996a39e0377a1059296f6aa7".to_string(),
                    storage_address: "axone1cxmx7su8h5pvqca85cxdylz86uj9x9gu5xuqv34kw87q5x0hexdsr3g4x4".to_string(),
                }])
            ),
            (
                vec![axone_logic_bindings::Result {
                    error: None,
                    substitutions: vec![Substitution {
                        variable: "X".to_string(),
                        expression: "[,]".to_string(),
                    }]
                }],
                Err(ContractError::LogicAskResponse(LogicAskResponseError::Parse(TermParseError::EmptyValue)))
            ),
            (
                vec![axone_logic_bindings::Result {
                    error: None,
                    substitutions: vec![Substitution {
                        variable: "X".to_string(),
                        expression: "(1,2)".to_string(),
                    }]
                }],
                Err(ContractError::LogicAskResponse(LogicAskResponseError::UnexpectedTerm))
            ),
            (
                vec![axone_logic_bindings::Result {
                    error: None,
                    substitutions: vec![Substitution {
                        variable: "X".to_string(),
                        expression: "[[]]".to_string(),
                    }]
                }],
                Err(ContractError::LogicAskResponse(LogicAskResponseError::UnexpectedTerm))
            ),
            (
                vec![axone_logic_bindings::Result {
                    error: None,
                    substitutions: vec![Substitution {
                        variable: "X".to_string(),
                        expression: "[[]]".to_string(),
                    }]
                }],
                Err(ContractError::LogicAskResponse(LogicAskResponseError::UnexpectedTerm))
            ),
            (
                vec![axone_logic_bindings::Result {
                    error: None,
                    substitutions: vec![Substitution {
                        variable: "X".to_string(),
                        expression: "['nawak']".to_string(),
                    }]
                }],
                Err(ContractError::ParseCosmwasmUri(CosmwasmUriError::ParseURI(url::ParseError::RelativeUrlWithoutBase)))
            ),
            (
                vec![axone_logic_bindings::Result {
                    error: None,
                    substitutions: vec![Substitution {
                        variable: "X".to_string(),
                        expression: "['cosmwasm:addr?query=%7B%22object%22%3A%7B%22id%22%3A%221485133dd3ab4b1c4b8085e7265585f91ae3cca0996a39e0377a1059296f6aa7%22%7D%7D']".to_string(),
                    }]
                }],
                Err(ContractError::ParseCosmwasmUri(CosmwasmUriError::Malformed("wrong query content".to_string())))
            ),
            (
                vec![
                    axone_logic_bindings::Result {
                        error: None,
                        substitutions: vec![Substitution {
                            variable: "X".to_string(),
                            expression: "[]".to_string(),
                        }]
                    },
                    axone_logic_bindings::Result {
                        error: None,
                        substitutions: vec![Substitution {
                            variable: "X".to_string(),
                            expression: "[]".to_string(),
                        }]
                    },
                ],
                Err(ContractError::LogicAskResponse(LogicAskResponseError::Unexpected("expected exactly one result".to_string())))
            ),
            (
                vec![
                    axone_logic_bindings::Result {
                        error: None,
                        substitutions: vec![
                            Substitution {
                                variable: "X".to_string(),
                                expression: "[]".to_string(),
                            },
                            Substitution {
                                variable: "X".to_string(),
                                expression: "[]".to_string(),
                            },
                        ]
                    },
                ],
                Err(ContractError::LogicAskResponse(LogicAskResponseError::Unexpected("expected exactly one substitution".to_string())))
            ),
            (
                vec![axone_logic_bindings::Result {
                    error: Some("error".to_string()),
                    substitutions: vec![Substitution {
                        variable: "X".to_string(),
                        expression: "[]".to_string(),
                    }]
                }],
                Err(ContractError::LogicAskResponse(LogicAskResponseError::Substitution("error".to_string())))
            ),
        ];

        for case in cases {
            assert_eq!(
                ask_response_to_objects(
                    AskResponse {
                        answer: Some(Answer {
                            results: case.0,
                            has_more: false,
                            variables: vec!["X".to_string()],
                        }),
                        height: 1,
                        gas_used: 1,
                        user_output: None,
                    },
                    "X".to_string()
                ),
                case.1
            );
        }
    }
}
