use crate::credential::rdf_marker::IRI_VC_TYPE;
use crate::credential::vc::VerifiableCredential;
use crate::ContractError;
use cosmwasm_std::Addr;
use itertools::Itertools;
use okp4_rdf::dataset::Dataset;

#[derive(Debug, PartialEq)]
pub struct DataverseCredential<'a> {
    pub submitter_addr: Addr,
    pub id: &'a str,
    pub issuer: &'a str,
    pub r#type: &'a str,
    pub valid_from: &'a str,
    pub valid_until: Option<&'a str>,
    pub subject: &'a str,
    pub claim: &'a Dataset<'a>,
}

impl<'a> DataverseCredential<'a> {
    fn extract_vc_type(vc: &'a VerifiableCredential<'a>) -> Result<&'a str, ContractError> {
        vc.types
            .iter()
            .filter(|t| *t != &IRI_VC_TYPE)
            .exactly_one()
            .map_err(|_| {
                ContractError::UnsupportedCredential(
                    "credential is expected to have exactly one type".to_string(),
                )
            })
            .map(|t| *t)
    }

    fn extract_vc_claim(
        vc: &'a VerifiableCredential<'a>,
    ) -> Result<(&'a str, &'a Dataset<'a>), ContractError> {
        vc.claims
            .iter()
            .exactly_one()
            .map(|claim| (claim.id, &(claim.content)))
            .map_err(|_| {
                ContractError::UnsupportedCredential(
                    "credential is expected to contain exactly one claim".to_string(),
                )
            })
    }
}

impl<'a> TryFrom<(Addr, &'a VerifiableCredential<'a>)> for DataverseCredential<'a> {
    type Error = ContractError;

    fn try_from(
        (submitter_addr, vc): (Addr, &'a VerifiableCredential<'a>),
    ) -> Result<Self, Self::Error> {
        let (subject, claim) = DataverseCredential::extract_vc_claim(vc)?;
        Ok(DataverseCredential {
            submitter_addr,
            id: vc.id,
            issuer: vc.issuer,
            r#type: DataverseCredential::extract_vc_type(vc)?,
            valid_from: vc.issuance_date,
            valid_until: vc.expiration_date,
            subject,
            claim,
        })
    }
}