#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Core data types and provider interfaces for OpenCryptCommons.
//!
//! This crate intentionally contains no production cryptographic implementation.
//! Cryptographic algorithms belong in separately reviewed provider crates.

mod credential;
mod policy;
mod traits;

pub use credential::{CredentialEnvelope, CredentialValidationError};
pub use policy::{
    evaluate_policy, Approval, PolicyError, PolicyEvaluation, ThresholdPolicy,
};
pub use traits::{
    CryptoBackendError, SelectiveDisclosureSuite, SignatureSuite, ThresholdSignatureSuite,
};
