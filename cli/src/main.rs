#![forbid(unsafe_code)]

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use occ_core::{evaluate_policy, Approval, CredentialEnvelope, ThresholdPolicy};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(
    name = "occ",
    version,
    about = "OpenCryptCommons Phase 1 command-line foundation"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Generate non-secret sample JSON files.
    Sample {
        /// Directory in which the sample files will be created.
        #[arg(long, default_value = "examples/generated")]
        output_dir: PathBuf,
    },

    /// Perform basic structural validation of a credential envelope.
    CredentialCheck {
        /// Path to a credential JSON file.
        #[arg(long)]
        credential: PathBuf,
    },

    /// Evaluate approvals against a threshold policy.
    PolicyCheck {
        /// Path to a threshold policy JSON file.
        #[arg(long)]
        policy: PathBuf,

        /// Path to a JSON array of approvals.
        #[arg(long)]
        approvals: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Sample { output_dir } => create_samples(&output_dir),
        Commands::CredentialCheck { credential } => {
            let credential: CredentialEnvelope = read_json(&credential)?;
            credential
                .validate()
                .context("credential envelope is invalid")?;

            println!("Credential envelope is structurally valid.");
            println!(
                "{}",
                serde_json::to_string_pretty(&credential)
                    .context("failed to print credential")?
            );
            Ok(())
        }
        Commands::PolicyCheck { policy, approvals } => {
            let policy: ThresholdPolicy = read_json(&policy)?;
            let approvals: Vec<Approval> = read_json(&approvals)?;
            let evaluation =
                evaluate_policy(&policy, &approvals).context("policy evaluation failed")?;

            println!(
                "{}",
                serde_json::to_string_pretty(&evaluation)
                    .context("failed to print policy evaluation")?
            );
            Ok(())
        }
    }
}

fn create_samples(output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir).with_context(|| {
        format!(
            "failed to create sample directory {}",
            output_dir.display()
        )
    })?;

    let credential = CredentialEnvelope {
        schema: "occ:credential-envelope:v1".to_owned(),
        issuer: "https://issuer.example".to_owned(),
        subject: "did:example:holder".to_owned(),
        issued_at: "2026-06-22T10:00:00Z".to_owned(),
        expires_at: Some("2027-06-22T10:00:00Z".to_owned()),
        claims: serde_json::from_value(json!({
            "role": "researcher",
            "organization": "Example Institute",
            "country": "AM"
        }))
        .context("sample claims must be an object")?,
        signature_suite: "planned:rfc9901-adapter".to_owned(),
    };

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

    write_json(&output_dir.join("sample-credential.json"), &credential)?;
    write_json(&output_dir.join("sample-policy.json"), &policy)?;
    write_json(&output_dir.join("sample-approvals.json"), &approvals)?;

    println!("Sample files created in {}", output_dir.display());
    Ok(())
}

fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let data = fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;

    serde_json::from_str(&data)
        .with_context(|| format!("invalid JSON in {}", path.display()))
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let data =
        serde_json::to_string_pretty(value).context("failed to encode JSON")?;

    fs::write(path, format!("{data}\n"))
        .with_context(|| format!("failed to write {}", path.display()))
}
