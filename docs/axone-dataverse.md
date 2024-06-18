# Dataverse

## Overview

The `dataverse` smart contract is responsible for overseeing and managing the Dataverse.

## Dataverse

The Dataverse is an ever-expanding universe that encompasses a wide range of digital resources. These include datasets, data processing algorithms, ML algorithm, storage resources, computational resources, identity management solutions, orchestration engines, oracles, and many other resources recorded on the blockchain.

When the smart contract is instantiated, it creates a Dataverse instance. This instance is separated and isolated from any pre-existing ones, and as many dataverse instances as required can be created.

## Zones

Zones within the Dataverse represent distinct areas or domains where specific governance rules and policies are applied. These Zones are conceptual frameworks created to manage and organize resources under a unified set of regulations and permissions.

Each Zone is defined by its unique identity and set of governing rules, which dictate how resources within it can be accessed, used, and shared. This approach allows for granular control over different segments of the Dataverse, catering to various requirements and use cases. By managing these Zones, the dataverse smart contract ensures that resources are utilized in compliance with the defined policies and consents, thereby maintaining order and integrity within the Dataverse.

## Resources

In the context of the Dataverse, Resources refer to a broad category of digital entities, which include Services and Digital Resources.

- **Digital Resources**: This category extends to various digital entities such as datasets, algorithms, machine learning models, and other digital assets. Like Services, Digital Resources are identified by a URI in conjunction with the Service responsible for their provision.

- **Services**: These are network-accessible functionalities like REST APIs, gRPC services, and other similar offerings. Each Service in the Dataverse is uniquely identified by its Uniform Resource Identifier (URI) and is associated with a specific Registrar responsible for its registration and management.

## Decentralized Identifiers (DID)

Decentralized Identifiers (DID) are a foundational element in the Dataverse, serving as unique, persistent, and globally resolvable identifiers that are fully under the control of the DID subject, which could be an individual, organization, or a any kind of resource (dataset,
algorithm, nft, ML algorithm).

DIDs play a crucial role in the Dataverse by facilitating a trustable and interoperable identity mechanism. They enable the establishment of a verifiable and self-sovereign identity for resources, services, and entities within the ecosystem.

## Claims

Claims in the Dataverse context are assertions or statements made about a Resource identified by a DID.

Claims play a pivotal role in the governance framework of the Dataverse. By leveraging knowledge derived from verifiable credentials, the governances established by Zones can evaluate the fulfilment of specific rules and compliance. This evaluation is critical in ensuring that the resources within the Dataverse adhere to the established norms, policies, and requirements.

Claims are submitted in the form of [Verifiable Presentations (VPs)](https://www.w3.org/TR/vc-data-model/#presentations), which are aggregations of one or more [Verifiable Credentials (VCs)](https://www.w3.org/TR/vc-data-model/#what-is-a-verifiable-credential).

## Dependencies

Given its role and status, this smart contract serves as the primary access point for the AXONE protocol to manage all on-chain stored resources. To fulfill its tasks, the smart contract relies on other smart contracts within the AXONE ecosystem. Notably, it uses the `Cognitarium` smart contract for persisting the Dataverse representation in an ontological form and the `Law Stone` smart contract to establish governance rules.

## InstantiateMsg

`InstantiateMsg` is used to initialize a new instance of the dataverse.

| parameter                    | description                                                                                                                                                                           |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `name`                       | _(Required.) _ **string**. A unique name to identify the dataverse instance.                                                                                                          |
| `triplestore_config`         | _(Required.) _ **[TripleStoreConfig](#triplestoreconfig)**. The configuration used to instantiate the triple store.                                                                   |
| `triplestore_config.code_id` | **[Uint64](#uint64)**. The code id that will be used to instantiate the triple store contract in which to store dataverse semantic data. It must implement the cognitarium interface. |
| `triplestore_config.limits`  | **[TripleStoreLimitsInput](#triplestorelimitsinput)**. Limitations regarding triple store usage.                                                                                      |

## ExecuteMsg

`ExecuteMsg` defines the set of possible actions that can be performed on the dataverse.

This enum provides variants for registering services, datasets, and other operations related to the dataverse.

### ExecuteMsg::SubmitClaims

Submits new claims about a resource to the dataverse.

The SubmitClaims message is a pivotal component in the dataverse, enabling entities to contribute new claims about various resources. A claim represents a statement made by an entity, referred to as the issuer, which could be a person, organization, or service. These claims pertain to a diverse range of resources, including digital resources, services, zones, or individuals, and are asserted as factual by the issuer.

#### Format

Claims are injected into the dataverse through Verifiable Credentials (VCs).

Primarily, the claims leverage the AXONE ontology, which facilitates articulating assertions about widely acknowledged resources in the dataverse, including digital services, digital resources, zones, governance, and more.

Additionally, other schemas may also be employed to supplement and enhance the validated knowledge contributed to these resources.

#### Preconditions

To maintain integrity and coherence in the dataverse, several preconditions are set for the submission of claims:

1. **Format Requirement**: Claims must be encapsulated within Verifiable Credentials (VCs).

2. **Unique Identifier Mandate**: Each Verifiable Credential within the dataverse must possess a unique identifier.

3. **Issuer Signature**: Claims must bear the issuer's signature. This signature must be verifiable, ensuring authenticity and credibility.

4. **Content**: The actual implementation supports the submission of a single Verifiable Credential, containing a single claim.

#### Supported cryptographic proofs

- `Ed25519Signature2018`

- `Ed25519Signature2020`

- `EcdsaSecp256k1Signature2019`

- `DataIntegrity` with the following cryptosuites: `eddsa-2022`, `eddsa-rdfc-2022`.

| parameter              | description                                                                                                                                                                                |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `submit_claims`        | _(Required.) _ **object**.                                                                                                                                                                 |
| `submit_claims.claims` | _(Required.) _ **[Binary](#binary)**. The Verifiable Credential containing the claims. The claims must be serialized in the format specified by the `format` field.                        |
| `submit_claims.format` | **[RdfDatasetFormat](#rdfdatasetformat)\|null**. RDF dataset serialization format for the claims. If not provided, the default format is [N-Quads](https://www.w3.org/TR/n-quads/) format. |

### ExecuteMsg::RevokeClaims

Revoke or withdraw a previously submitted claims.

#### Preconditions:

1. **Identifier Existance**: The identifier of the claims must exist in the dataverse.

| parameter                  | description                                                                   |
| -------------------------- | ----------------------------------------------------------------------------- |
| `revoke_claims`            | _(Required.) _ **object**.                                                    |
| `revoke_claims.identifier` | _(Required.) _ **string**. The unique identifier of the claims to be revoked. |

## QueryMsg

`QueryMsg` defines the set of possible queries that can be made to retrieve information about the dataverse.

This enum provides variants for querying the dataverse's details and other related information.

### QueryMsg::Dataverse

Retrieves information about the current dataverse instance.

| parameter   | description                |
| ----------- | -------------------------- |
| `dataverse` | _(Required.) _ **object**. |

## Responses

### dataverse

DataverseResponse is the response of the Dataverse query.

| property              | description                                                         |
| --------------------- | ------------------------------------------------------------------- |
| `name`                | _(Required.) _ **string**. The name of the dataverse.               |
| `triplestore_address` | _(Required.) _ **[Addr](#addr)**. The cognitarium contract address. |

## Definitions

### Addr

A human readable address.

In Cosmos, this is typically bech32 encoded. But for multi-chain smart contracts no assumptions should be made other than being UTF-8 encoded and of reasonable length.

This type represents a validated address. It can be created in the following ways 1. Use `Addr::unchecked(input)` 2. Use `let checked: Addr = deps.api.addr_validate(input)?` 3. Use `let checked: Addr = deps.api.addr_humanize(canonical_addr)?` 4. Deserialize from JSON. This must only be done from JSON that was validated before such as a contract's state. `Addr` must not be used in messages sent by the user because this would result in unvalidated instances.

This type is immutable. If you really need to mutate it (Really? Are you sure?), create a mutable copy using `let mut mutable = Addr::to_string()` and operate on that `String` instance.

| type        |
| ----------- |
| **string**. |

### Binary

A string containing Base64-encoded data.

| type        |
| ----------- |
| **string**. |

### NQuads

N-Quads Format

N-Quads is an extension of N-Triples to support RDF datasets by adding an optional fourth element to represent the graph name. See the [official N-Quads specification](https://www.w3.org/TR/n-quads/).

| literal     |
| ----------- |
| `"n_quads"` |

### RdfDatasetFormat

Represents the various serialization formats for an RDF dataset, i.e. a collection of RDF graphs ([RDF Dataset](https://www.w3.org/TR/rdf11-concepts/#section-dataset)).

| variant           | description                                                                                                                                                                                                                                               |
| ----------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [NQuads](#nquads) | **string**: `n_quads`. N-Quads Format<br /><br />N-Quads is an extension of N-Triples to support RDF datasets by adding an optional fourth element to represent the graph name. See the [official N-Quads specification](https://www.w3.org/TR/n-quads/). |

### TripleStoreConfig

`TripleStoreConfig` represents the configuration related to the management of the triple store.

| property                              | description                                                                                                                                                                                                                                                                                                                                                                                                            |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `code_id`                             | _(Required.) _ **[Uint64](#uint64)**. The code id that will be used to instantiate the triple store contract in which to store dataverse semantic data. It must implement the cognitarium interface.                                                                                                                                                                                                                   |
| `limits`                              | _(Required.) _ **[TripleStoreLimitsInput](#triplestorelimitsinput)**. Limitations regarding triple store usage.                                                                                                                                                                                                                                                                                                        |
| `limits.max_byte_size`                | **[Uint128](#uint128)\|null**. The maximum number of bytes the store can contain. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. Default to [Uint128::MAX] if not set, which can be considered as no limit.                                                                                                   |
| `limits.max_insert_data_byte_size`    | **[Uint128](#uint128)\|null**. The maximum number of bytes an insert data query can contain. Default to [Uint128::MAX] if not set, which can be considered as no limit.                                                                                                                                                                                                                                                |
| `limits.max_insert_data_triple_count` | **[Uint128](#uint128)\|null**. The maximum number of triples an insert data query can contain (after parsing). Default to [Uint128::MAX] if not set, which can be considered as no limit.                                                                                                                                                                                                                              |
| `limits.max_query_limit`              | **integer\|null**. The maximum limit of a query, i.e. the maximum number of triples returned by a select query. Default to 30 if not set.                                                                                                                                                                                                                                                                              |
| `limits.max_query_variable_count`     | **integer\|null**. The maximum number of variables a query can select. Default to 30 if not set.                                                                                                                                                                                                                                                                                                                       |
| `limits.max_triple_byte_size`         | **[Uint128](#uint128)\|null**. The maximum number of bytes the store can contain for a single triple. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. The limit is used to prevent storing very large triples, especially literals. Default to [Uint128::MAX] if not set, which can be considered as no limit. |
| `limits.max_triple_count`             | **[Uint128](#uint128)\|null**. The maximum number of triples the store can contain. Default to [Uint128::MAX] if not set, which can be considered as no limit.                                                                                                                                                                                                                                                         |

### TripleStoreLimitsInput

Contains requested limitations regarding store usages.

| property                       | description                                                                                                                                                                                                                                                                                                                                                                                                            |
| ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `max_byte_size`                | **[Uint128](#uint128)\|null**. The maximum number of bytes the store can contain. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. Default to [Uint128::MAX] if not set, which can be considered as no limit.                                                                                                   |
| `max_insert_data_byte_size`    | **[Uint128](#uint128)\|null**. The maximum number of bytes an insert data query can contain. Default to [Uint128::MAX] if not set, which can be considered as no limit.                                                                                                                                                                                                                                                |
| `max_insert_data_triple_count` | **[Uint128](#uint128)\|null**. The maximum number of triples an insert data query can contain (after parsing). Default to [Uint128::MAX] if not set, which can be considered as no limit.                                                                                                                                                                                                                              |
| `max_query_limit`              | **integer\|null**. The maximum limit of a query, i.e. the maximum number of triples returned by a select query. Default to 30 if not set.                                                                                                                                                                                                                                                                              |
| `max_query_variable_count`     | **integer\|null**. The maximum number of variables a query can select. Default to 30 if not set.                                                                                                                                                                                                                                                                                                                       |
| `max_triple_byte_size`         | **[Uint128](#uint128)\|null**. The maximum number of bytes the store can contain for a single triple. The size of a triple is counted as the sum of the size of its subject, predicate and object, including the size of data types and language tags if any. The limit is used to prevent storing very large triples, especially literals. Default to [Uint128::MAX] if not set, which can be considered as no limit. |
| `max_triple_count`             | **[Uint128](#uint128)\|null**. The maximum number of triples the store can contain. Default to [Uint128::MAX] if not set, which can be considered as no limit.                                                                                                                                                                                                                                                         |

### Uint128

A string containing a 128-bit integer in decimal representation.

| type        |
| ----------- |
| **string**. |

### Uint64

A thin wrapper around u64 that is using strings for JSON encoding/decoding, such that the full u64 range can be used for clients that convert JSON numbers to floats, like JavaScript and jq.

# Examples

Use `from` to create instances of this and `u64` to get the value out:

````# use cosmwasm_std::Uint64; let a = Uint64::from(42u64); assert_eq!(a.u64(), 42);

let b = Uint64::from(70u32); assert_eq!(b.u64(), 70); ```

|type|
|----|
|**string**.|

---

*Rendered by [Fadroma](https://fadroma.tech) ([@fadroma/schema 1.1.0](https://www.npmjs.com/package/@fadroma/schema)) from `axone-dataverse.json` (`1c3a4ebcda16a08f`)*
````
