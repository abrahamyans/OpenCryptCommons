use occ_core::{CredentialEnvelope, CredentialValidationError};
use serde_json::json;

#[test]
fn valid_credential_envelope_passes() {
    let credential = CredentialEnvelope {
        schema: "occ:credential-envelope:v1".to_owned(),
        issuer: "https://issuer.example".to_owned(),
        subject: "did:example:holder".to_owned(),
        issued_at: "2026-06-22T10:00:00Z".to_owned(),
        expires_at: None,
        claims: serde_json::from_value(json!({
            "role": "researcher"
        }))
        .expect("object"),
        signature_suite: "planned:sd-jwt".to_owned(),
    };

    assert_eq!(credential.validate(), Ok(()));
}

#[test]
fn empty_claims_are_rejected() {
    let credential = CredentialEnvelope {
        schema: "occ:credential-envelope:v1".to_owned(),
        issuer: "https://issuer.example".to_owned(),
        subject: "did:example:holder".to_owned(),
        issued_at: "2026-06-22T10:00:00Z".to_owned(),
        expires_at: None,
        claims: Default::default(),
        signature_suite: "planned:sd-jwt".to_owned(),
    };

    assert_eq!(
        credential.validate(),
        Err(CredentialValidationError::EmptyClaims)
    );
}
