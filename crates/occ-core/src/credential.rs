use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use thiserror::Error;

/// A neutral credential envelope used by the Phase 1 architecture.
///
/// This structure is not itself a W3C Verifiable Credential or SD-JWT.
/// A future standards adapter will convert between its domain representation
/// and a standards-defined wire format.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CredentialEnvelope {
    /// Versioned schema or profile identifier.
    pub schema: String,
    /// Identifier of the credential issuer.
    pub issuer: String,
    /// Identifier of the credential subject.
    pub subject: String,
    /// Issuance time in an application-selected interoperable format.
    pub issued_at: String,
    /// Optional expiration time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Claims carried by the credential.
    pub claims: Map<String, Value>,
    /// Requested or applied signature-suite identifier.
    pub signature_suite: String,
}

impl CredentialEnvelope {
    /// Performs basic structural validation.
    ///
    /// Cryptographic verification, issuer trust, status, and time validation
    /// are intentionally outside this method.
    pub fn validate(&self) -> Result<(), CredentialValidationError> {
        validate_non_empty("schema", &self.schema)?;
        validate_non_empty("issuer", &self.issuer)?;
        validate_non_empty("subject", &self.subject)?;
        validate_non_empty("issued_at", &self.issued_at)?;
        validate_non_empty("signature_suite", &self.signature_suite)?;

        if self.claims.is_empty() {
            return Err(CredentialValidationError::EmptyClaims);
        }

        Ok(())
    }
}

fn validate_non_empty(
    field: &'static str,
    value: &str,
) -> Result<(), CredentialValidationError> {
    if value.trim().is_empty() {
        Err(CredentialValidationError::EmptyField(field))
    } else {
        Ok(())
    }
}

/// Errors produced by basic credential-envelope validation.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CredentialValidationError {
    /// A required string field was empty.
    #[error("field `{0}` must not be empty")]
    EmptyField(&'static str),

    /// No claims were supplied.
    #[error("credential must contain at least one claim")]
    EmptyClaims,
}
