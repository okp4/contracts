use crate::credential::rdf_marker::RDF_DATE_TYPE;
use crate::registrar::credential::DataverseCredential;
use crate::ContractError;
use cosmwasm_std::{Binary, StdError};
use okp4_rdf::serde::{DataFormat, TripleWriter};
use rio_api::model::{Literal, NamedNode, Subject, Term, Triple};

pub const VC_SUBMITTER_ADDRESS: NamedNode<'_> = NamedNode {
    iri: "dataverse:credential#submitterAddress",
};
pub const VC_TYPE: NamedNode<'_> = NamedNode {
    iri: "dataverse:credential#type",
};
pub const VC_ISSUER: NamedNode<'_> = NamedNode {
    iri: "dataverse:credential#issuer",
};
pub const VC_VALID_FROM: NamedNode<'_> = NamedNode {
    iri: "dataverse:credential#validFrom",
};
pub const VC_VALID_UNTIL: NamedNode<'_> = NamedNode {
    iri: "dataverse:credential#validUntil",
};
pub const VC_SUBJECT: NamedNode<'_> = NamedNode {
    iri: "dataverse:credential#subject",
};
pub const VC_CLAIM: NamedNode<'_> = NamedNode {
    iri: "dataverse:credential#claim",
};

impl<'a> From<&'a DataverseCredential<'a>> for Vec<Triple<'a>> {
    fn from(credential: &'a DataverseCredential<'a>) -> Self {
        let c_subject = Subject::NamedNode(NamedNode { iri: credential.id });

        let mut triples = vec![
            Triple {
                subject: c_subject,
                predicate: VC_SUBMITTER_ADDRESS,
                object: Term::NamedNode(NamedNode {
                    iri: credential.submitter_addr.as_str(),
                }),
            },
            Triple {
                subject: c_subject,
                predicate: VC_ISSUER,
                object: Term::NamedNode(NamedNode {
                    iri: credential.issuer,
                }),
            },
            Triple {
                subject: c_subject,
                predicate: VC_TYPE,
                object: Term::NamedNode(NamedNode {
                    iri: credential.r#type,
                }),
            },
            Triple {
                subject: c_subject,
                predicate: VC_VALID_FROM,
                object: Term::Literal(Literal::Typed {
                    value: credential.valid_from,
                    datatype: RDF_DATE_TYPE,
                }),
            },
            Triple {
                subject: c_subject,
                predicate: VC_SUBJECT,
                object: Term::NamedNode(NamedNode {
                    iri: credential.subject,
                }),
            },
        ];

        triples.extend(credential.claim.as_slice());

        if let Some(valid_until) = credential.valid_until {
            triples.push(Triple {
                subject: c_subject,
                predicate: VC_VALID_UNTIL,
                object: Term::Literal(Literal::Typed {
                    value: valid_until,
                    datatype: RDF_DATE_TYPE,
                }),
            });
        }

        triples
    }
}

pub fn serialize(
    credential: &DataverseCredential<'_>,
    format: DataFormat,
) -> Result<Binary, ContractError> {
    let triples: Vec<Triple<'_>> = credential.into();
    let out: Vec<u8> = Vec::default();
    let mut writer = TripleWriter::new(&format, out);
    for triple in triples {
        writer
            .write(&triple)
            .map_err(|e| StdError::serialize_err("triple", format!("Error writing triple: {e}")))?;
    }

    Ok(Binary::from(writer.finish().map_err(|e| {
        StdError::serialize_err("triple", format!("Error writing triple: {e}"))
    })?))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::credential::vc::VerifiableCredential;
    use crate::testutil::testutil;
    use cosmwasm_std::Addr;
    use okp4_rdf::dataset::Dataset;

    #[test]
    fn proper_serialization() {
        let owned_quads = testutil::read_test_quads("vc-valid.nq");
        let dataset = Dataset::from(owned_quads.as_slice());
        let vc = VerifiableCredential::try_from(&dataset).unwrap();
        let dc = DataverseCredential::try_from((
            Addr::unchecked("okp41072nc6egexqr2v6vpp7yxwm68plvqnkf6xsytf"),
            &vc,
        ))
        .unwrap();

        let expected = "<https://w3id.org/okp4/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#submitterAddress> <okp41072nc6egexqr2v6vpp7yxwm68plvqnkf6xsytf> .
<https://w3id.org/okp4/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#issuer> <did:key:zQ3shs7auhJSmVJpiUbQWco6bxxEhSqWnVEPvaBHBRvBKw6Q3> .
<https://w3id.org/okp4/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#type> <https://w3id.org/okp4/ontology/vnext/schema/credential/digital-service/description/DigitalServiceDescriptionCredential> .
<https://w3id.org/okp4/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#validFrom> \"2024-01-22T00:00:00\"^^<http://www.w3.org/2001/XMLSchema#dateTime> .
<https://w3id.org/okp4/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#subject> <did:key:zQ3shhb4SvzBRLbBonsvKb3WX6WoDeKWHpsXXXMhAJETqXAfB> .
_:c0 <https://w3id.org/okp4/ontology/vnext/schema/credential/digital-service/description/hasCategory> <https://w3id.org/okp4/ontology/vnext/thesaurus/digital-service-category/Storage> .
_:c0 <https://w3id.org/okp4/ontology/vnext/schema/credential/digital-service/description/hasTag> \"Cloud\" .
<https://w3id.org/okp4/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#claim> _:c0 .
<https://w3id.org/okp4/ontology/vnext/schema/credential/digital-service/description/72cab400-5bd6-4eb4-8605-a5ee8c1a45c9> <dataverse:credential#validUntil> \"2025-01-22T00:00:00\"^^<http://www.w3.org/2001/XMLSchema#dateTime> .\n";

        let serialization_res = serialize(&dc, DataFormat::NQuads);
        assert!(serialization_res.is_ok());

        assert_eq!(
            String::from_utf8(serialization_res.unwrap().0).unwrap(),
            expected
        );
    }
}
