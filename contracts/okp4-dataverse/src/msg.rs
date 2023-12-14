use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Binary, Uint128, Uint64};

/// `InstantiateMsg` is used to initialize a new instance of the dataverse.
#[cw_serde]
pub struct InstantiateMsg {
    /// A unique name to identify the dataverse instance.
    pub name: String,

    /// The configuration used to instantiate the triple store.
    pub triplestore_config: TripleStoreConfig,
}

/// `ExecuteMsg` defines the set of possible actions that can be performed on the dataverse.
///
/// This enum provides variants for registering services, datasets, and other operations related to the dataverse.
#[cw_serde]
pub enum ExecuteMsg {
    /// # SubmitClaims
    /// Submits new claims about a resource to the dataverse.
    ///
    /// The SubmitClaims message is a pivotal component in the dataverse, enabling entities to contribute new claims about various
    /// resources. A claim represents a statement made by an entity, referred to as the issuer, which could be a person, organization,
    /// or service. These claims pertain to a diverse range of resources, including digital resources, services, zones, or individuals,
    /// and are asserted as factual by the issuer.
    ///
    /// #### Format
    ///
    /// Claims are injected into the dataverse through Verifiable Presentations (VPs). These presentations effectively amalgamate and
    /// showcase multiple credentials, thus providing a cohesive and comprehensive view of the assertions being made.
    ///
    /// While the data in a VP typically revolves around a common subject, it accommodates an unlimited number of subjects and issuers.
    /// This flexibility allows for a broad spectrum of claims to be represented.
    ///
    /// Primarily, the claims leverage the OKP4 ontology, which facilitates articulating assertions about widely acknowledged resources
    /// in the dataverse, including digital services, digital resources, zones, governance, and more.
    ///
    /// Additionally, other schemas may also be employed to supplement and enhance the validated knowledge contributed to these resources.
    ///
    /// #### Preconditions
    ///
    /// To maintain integrity and coherence in the dataverse, several preconditions are set for the submission of claims:
    ///
    ///   1. **Format Requirement**: Claims must be encapsulated within Verifiable Presentations (VPs).
    ///
    ///   2. **Unique Identifier Mandate**: Each Verifiable Credential within the dataverse must possess a unique identifier.
    ///
    ///   3. **Issuer Signature**: Claims must bear the issuer's signature. This signature must be verifiable, ensuring authenticity and credibility.
    SubmitClaims {
        /// The serialized metadata intended for attachment.
        /// This metadata should adhere to the format specified in the `format` field.
        metadata: Binary,
        /// RDF format in which the metadata is represented.
        /// If not provided, the default format is [Turtle](https://www.w3.org/TR/turtle/) format.
        format: Option<RdfFormat>,
    },

    /// # RevokeClaims
    /// Revoke or withdraw a previously submitted claims.
    ///
    /// #### Preconditions:
    ///
    ///  1. **Identifier Existance**: The identifier of the claims must exist in the dataverse.
    RevokeClaims {
        /// The unique identifier of the claims to be revoked.
        identifier: Uri,
    },
}

/// # TripleStoreConfig
/// `TripleStoreConfig` represents the configuration related to the management of the triple store.
#[cw_serde]
pub struct TripleStoreConfig {
    /// The code id that will be used to instantiate the triple store contract in which
    /// to store dataverse semantic data. It must implement the cognitarium interface.
    pub code_id: Uint64,

    /// Limitations regarding triple store usage.
    pub limits: TripleStoreLimitsInput,
}

/// # TripleStoreLimitsInput
/// Contains requested limitations regarding store usages.
#[cw_serde]
#[derive(Default)]
pub struct TripleStoreLimitsInput {
    /// The maximum number of triples the store can contain.
    /// Default to [Uint128::MAX] if not set, which can be considered as no limit.
    pub max_triple_count: Option<Uint128>,
    /// The maximum number of bytes the store can contain.
    /// The size of a triple is counted as the sum of the size of its subject, predicate and object,
    /// including the size of data types and language tags if any.
    /// Default to [Uint128::MAX] if not set, which can be considered as no limit.
    pub max_byte_size: Option<Uint128>,
    /// The maximum number of bytes the store can contain for a single triple.
    /// The size of a triple is counted as the sum of the size of its subject, predicate and object,
    /// including the size of data types and language tags if any. The limit is used to prevent
    /// storing very large triples, especially literals.
    /// Default to [Uint128::MAX] if not set, which can be considered as no limit.
    pub max_triple_byte_size: Option<Uint128>,
    /// The maximum limit of a query, i.e. the maximum number of triples returned by a select query.
    /// Default to 30 if not set.
    pub max_query_limit: Option<u32>,
    /// The maximum number of variables a query can select.
    /// Default to 30 if not set.
    pub max_query_variable_count: Option<u32>,
    /// The maximum number of bytes an insert data query can contain.
    /// Default to [Uint128::MAX] if not set, which can be considered as no limit.
    pub max_insert_data_byte_size: Option<Uint128>,
    /// The maximum number of triples an insert data query can contain (after parsing).
    /// Default to [Uint128::MAX] if not set, which can be considered as no limit.
    pub max_insert_data_triple_count: Option<Uint128>,
}

impl From<TripleStoreLimitsInput> for okp4_cognitarium::msg::StoreLimitsInput {
    fn from(value: TripleStoreLimitsInput) -> Self {
        let mut limits = okp4_cognitarium::msg::StoreLimitsInput::default();
        if let Some(max_triple_count) = value.max_triple_count {
            limits.max_triple_count = max_triple_count;
        }
        if let Some(max_byte_size) = value.max_byte_size {
            limits.max_byte_size = max_byte_size;
        }
        if let Some(max_triple_byte_size) = value.max_triple_byte_size {
            limits.max_triple_byte_size = max_triple_byte_size;
        }
        if let Some(max_query_limit) = value.max_query_limit {
            limits.max_query_limit = max_query_limit;
        }
        if let Some(max_query_variable_count) = value.max_query_variable_count {
            limits.max_query_variable_count = max_query_variable_count;
        }
        if let Some(max_insert_data_byte_size) = value.max_insert_data_byte_size {
            limits.max_insert_data_byte_size = max_insert_data_byte_size;
        }
        if let Some(max_insert_data_triple_count) = value.max_insert_data_triple_count {
            limits.max_insert_data_triple_count = max_insert_data_triple_count;
        }

        limits
    }
}

/// # RdfFormat
/// `RdfFormat` represents the various serialization formats for RDF (Resource Description Framework) data.
#[cw_serde]
#[derive(Default)]
pub enum RdfFormat {
    /// # RdfXml
    /// RDF/XML Format
    ///
    /// RDF/XML is a syntax to express RDF information in XML.
    /// See the [official RDF/XML specification](https://www.w3.org/TR/rdf-syntax-grammar/).
    #[serde(rename = "rdf_xml")]
    RdfXml,

    /// # Turtle
    /// Turtle (Terse RDF Triple Language) Format
    ///
    /// Turtle is a textual format for representing RDF triples in a more compact and human-readable way compared to RDF/XML.
    /// See the [official Turtle specification](https://www.w3.org/TR/turtle/).
    #[serde(rename = "turtle")]
    #[default]
    Turtle,

    /// # NTriples
    /// N-Triples Format
    ///
    /// N-Triples is a line-based, plain text format for encoding an RDF graph. Each line corresponds to a single RDF triple.
    /// See the [official N-Triples specification](https://www.w3.org/TR/n-triples/).
    #[serde(rename = "n_triples")]
    NTriples,

    /// # NQuads
    /// N-Quads Format
    ///
    /// N-Quads is an extension of N-Triples to support RDF datasets by adding an optional fourth element to represent the graph name.
    /// See the [official N-Quads specification](https://www.w3.org/TR/n-quads/).
    #[serde(rename = "n_quads")]
    NQuads,
}

/// # Uri
/// `Uri` represents a Uniform Resource Identifier (URI), a string of characters that provides a simple way
/// to identify a resource.
/// see https://en.wikipedia.org/wiki/Uniform_Resource_Identifier.
type Uri = String;

/// `QueryMsg` defines the set of possible queries that can be made to retrieve information about the dataverse.
///
/// This enum provides variants for querying the dataverse's details and other related information.
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// # Dataverse
    /// Retrieves information about the current dataverse instance.
    #[returns(DataverseResponse)]
    Dataverse {},
}

/// # DataverseResponse
/// DataverseResponse is the response of the Dataverse query.
#[cw_serde]
pub struct DataverseResponse {
    /// The name of the dataverse.
    pub name: String,
}