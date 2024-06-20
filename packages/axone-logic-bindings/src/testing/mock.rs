use std::marker::PhantomData;

use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{CustomQuery, OwnedDeps, QuerierResult};
use serde::de::DeserializeOwned;
use serde::Serialize;

/// A drop-in replacement for cosmwasm_std::testing::mock_dependencies
/// that allows to provide a custom querier handler.
pub fn mock_dependencies_with_custom_handler<Q, F>(
    custom_handler: F,
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<Q>, Q>
where
    Q: CustomQuery + DeserializeOwned + Serialize + 'static,
    F: Fn(&Q) -> QuerierResult + 'static,
{
    let custom_querier: MockQuerier<Q> =
        MockQuerier::new(&[]).with_custom_handler(move |query| custom_handler(query));

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
        custom_query_type: PhantomData,
    }
}
