use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

/// A simple threshold authorization policy.
///
/// This Phase 1 type counts authenticated approvals. It does not implement
/// threshold cryptography or prove that an approval was cryptographically signed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThresholdPolicy {
    /// Identifier of the sensitive operation.
    pub operation: String,
    /// Number of distinct authorized approvals required.
    pub threshold: usize,
    /// Authorized participant identifiers.
    pub participants: Vec<String>,
}

/// One participant's decision for an operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Approval {
    /// Identifier of the operation being approved.
    pub operation: String,
    /// Participant identifier.
    pub participant: String,
    /// Whether the participant approved the operation.
    pub approved: bool,
    /// Informational timestamp. Cryptographic freshness is not checked in Phase 1.
    pub timestamp: String,
}

/// Result of evaluating a set of approvals against a threshold policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyEvaluation {
    /// Whether the threshold has been reached.
    pub authorized: bool,
    /// Number of distinct valid approvals.
    pub valid_approvals: usize,
    /// Threshold required by the policy.
    pub required_approvals: usize,
    /// Number of additional approvals still required.
    pub missing_approvals: usize,
    /// Accepted participant identifiers.
    pub approved_participants: Vec<String>,
    /// Entries ignored because they were rejected, duplicated, unknown, or for another operation.
    pub ignored_approvals: usize,
}

/// Evaluates approvals using distinct authorized participant identifiers.
pub fn evaluate_policy(
    policy: &ThresholdPolicy,
    approvals: &[Approval],
) -> Result<PolicyEvaluation, PolicyError> {
    validate_policy(policy)?;

    let authorized: BTreeSet<&str> = policy.participants.iter().map(String::as_str).collect();
    let mut accepted = BTreeSet::new();
    let mut ignored = 0usize;

    for approval in approvals {
        let is_acceptable = approval.operation == policy.operation
            && approval.approved
            && authorized.contains(approval.participant.as_str())
            && !approval.participant.trim().is_empty();

        if is_acceptable && accepted.insert(approval.participant.clone()) {
            continue;
        }

        ignored += 1;
    }

    let count = accepted.len();
    let missing = policy.threshold.saturating_sub(count);

    Ok(PolicyEvaluation {
        authorized: count >= policy.threshold,
        valid_approvals: count,
        required_approvals: policy.threshold,
        missing_approvals: missing,
        approved_participants: accepted.into_iter().collect(),
        ignored_approvals: ignored,
    })
}

fn validate_policy(policy: &ThresholdPolicy) -> Result<(), PolicyError> {
    if policy.operation.trim().is_empty() {
        return Err(PolicyError::EmptyOperation);
    }

    if policy.participants.is_empty() {
        return Err(PolicyError::NoParticipants);
    }

    if policy.threshold == 0 {
        return Err(PolicyError::ZeroThreshold);
    }

    let normalized: BTreeSet<&str> = policy
        .participants
        .iter()
        .map(String::as_str)
        .filter(|value| !value.trim().is_empty())
        .collect();

    if normalized.len() != policy.participants.len() {
        return Err(PolicyError::DuplicateOrEmptyParticipant);
    }

    if policy.threshold > policy.participants.len() {
        return Err(PolicyError::ThresholdExceedsParticipants {
            threshold: policy.threshold,
            participants: policy.participants.len(),
        });
    }

    Ok(())
}

/// Threshold-policy validation errors.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum PolicyError {
    /// Operation identifier is empty.
    #[error("policy operation must not be empty")]
    EmptyOperation,

    /// Participant list is empty.
    #[error("policy must contain at least one participant")]
    NoParticipants,

    /// Threshold is zero.
    #[error("policy threshold must be at least one")]
    ZeroThreshold,

    /// A participant identifier was empty or duplicated.
    #[error("participant identifiers must be non-empty and unique")]
    DuplicateOrEmptyParticipant,

    /// Threshold is larger than the participant count.
    #[error(
        "threshold {threshold} exceeds participant count {participants}"
    )]
    ThresholdExceedsParticipants {
        /// Requested threshold.
        threshold: usize,
        /// Available participant count.
        participants: usize,
    },
}
