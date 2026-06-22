use serde_json::Value;
use thiserror::Error;

/// Common error type for future cryptographic provider adapters.
#[derive(Debug, Error)]
pub enum CryptoBackendError {
    /// The requested algorithm is not supported.
    #[error("unsupported algorithm: {0}")]
    UnsupportedAlgorithm(String),

    /// Input data is malformed.
    #[error("invalid input: {0}")]
    InvalidInput(String),

    /// A signature or proof failed verification.
    #[error("verification failed")]
    VerificationFailed,

    /// The provider reported an internal failure.
    #[error("provider failure: {0}")]
    Provider(String),
}

/// Provider interface for an ordinary digital-signature suite.
pub trait SignatureSuite {
    /// Returns the explicit algorithm identifier.
    fn algorithm(&self) -> &'static str;

    /// Signs a message using provider-owned key material.
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, CryptoBackendError>;

    /// Verifies a signature.
    fn verify(
        &self,
        message: &[u8],
        signature: &[u8],
    ) -> Result<(), CryptoBackendError>;
}

/// Provider interface for standards-based selective disclosure.
pub trait SelectiveDisclosureSuite {
    /// Issues a credential with selected disclosable claim paths.
    fn issue(
        &self,
        claims: &Value,
        disclosable_paths: &[String],
    ) -> Result<String, CryptoBackendError>;

    /// Creates a presentation revealing only requested claim paths.
    fn present(
        &self,
        issued_credential: &str,
        reveal_paths: &[String],
        audience: &str,
        nonce: &str,
    ) -> Result<String, CryptoBackendError>;

    /// Verifies a presentation and returns the disclosed claim object.
    fn verify_presentation(
        &self,
        presentation: &str,
        expected_audience: &str,
        expected_nonce: &str,
    ) -> Result<Value, CryptoBackendError>;
}

/// Provider interface for a threshold-signature workflow.
///
/// Implementations must document message transport, participant
/// authentication, replay protection, and key-generation assumptions.
pub trait ThresholdSignatureSuite {
    /// Creates round-one material for one participant.
    fn round_one(
        &self,
        participant: &str,
        message: &[u8],
    ) -> Result<Vec<u8>, CryptoBackendError>;

    /// Creates round-two material after receiving an authenticated signing package.
    fn round_two(
        &self,
        participant: &str,
        signing_package: &[u8],
    ) -> Result<Vec<u8>, CryptoBackendError>;

    /// Aggregates authenticated signature shares.
    fn aggregate(
        &self,
        signing_package: &[u8],
        signature_shares: &[Vec<u8>],
    ) -> Result<Vec<u8>, CryptoBackendError>;
}
