use occ_core::{evaluate_policy, Approval, ThresholdPolicy};

#[test]
fn distinct_valid_approvals_reach_threshold() {
    let policy = ThresholdPolicy {
        operation: "rotate-issuer-key".to_owned(),
        threshold: 2,
        participants: vec![
            "alice".to_owned(),
            "bob".to_owned(),
            "carol".to_owned(),
        ],
    };

    let approvals = vec![
        Approval {
            operation: "rotate-issuer-key".to_owned(),
            participant: "alice".to_owned(),
            approved: true,
            timestamp: "2026-06-22T10:00:00Z".to_owned(),
        },
        Approval {
            operation: "rotate-issuer-key".to_owned(),
            participant: "bob".to_owned(),
            approved: true,
            timestamp: "2026-06-22T10:01:00Z".to_owned(),
        },
        Approval {
            operation: "rotate-issuer-key".to_owned(),
            participant: "mallory".to_owned(),
            approved: true,
            timestamp: "2026-06-22T10:02:00Z".to_owned(),
        },
    ];

    let result = evaluate_policy(&policy, &approvals).expect("valid policy");

    assert!(result.authorized);
    assert_eq!(result.valid_approvals, 2);
    assert_eq!(result.ignored_approvals, 1);
}

#[test]
fn duplicate_approval_is_counted_once() {
    let policy = ThresholdPolicy {
        operation: "publish-release".to_owned(),
        threshold: 2,
        participants: vec!["alice".to_owned(), "bob".to_owned()],
    };

    let approvals = vec![
        Approval {
            operation: "publish-release".to_owned(),
            participant: "alice".to_owned(),
            approved: true,
            timestamp: "2026-06-22T10:00:00Z".to_owned(),
        },
        Approval {
            operation: "publish-release".to_owned(),
            participant: "alice".to_owned(),
            approved: true,
            timestamp: "2026-06-22T10:01:00Z".to_owned(),
        },
    ];

    let result = evaluate_policy(&policy, &approvals).expect("valid policy");

    assert!(!result.authorized);
    assert_eq!(result.valid_approvals, 1);
    assert_eq!(result.ignored_approvals, 1);
}
