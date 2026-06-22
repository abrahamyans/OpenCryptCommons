# OpenCryptCommons

**Post-Quantum-Ready Selective Disclosure and Threshold Trust Toolkit for Internet Services**

> **Current status:** Phase 1 engineering foundation. This repository does not yet provide production-ready cryptographic credentials.

OpenCryptCommons is an open-source project intended to provide reusable, developer-friendly building blocks for:

- selective-disclosure credentials;
- threshold-based authorization and shared control;
- crypto-agile migration toward post-quantum signatures;
- integration into self-hosted, federated, and public-interest internet services.

## What this starter version contains

This first implementation package provides:

- a Rust workspace;
- a reusable `occ-core` crate;
- credential data validation;
- threshold-policy validation and approval counting;
- a command-line application;
- sample JSON files;
- automated tests;
- GitHub Actions continuous integration;
- an initial threat model and standards plan.

## What it deliberately does not contain yet

The current version does **not**:

- issue or verify real SD-JWT credentials;
- create FROST threshold signatures;
- create ML-DSA post-quantum signatures;
- manage production keys;
- claim security certification or external audit.

These capabilities must be added using established standards and reviewed libraries. OpenCryptCommons must not invent new cryptographic primitives.

## Planned standards direction

The project currently targets:

- W3C Verifiable Credentials Data Model 2.0;
- RFC 9901 Selective Disclosure for JSON Web Tokens;
- RFC 9591 FROST for classical threshold signatures;
- NIST FIPS 204 ML-DSA for post-quantum digital signatures.

FROST and ML-DSA are separate mechanisms. FROST is not post-quantum. A future OpenCryptCommons design may use threshold authorization to control a post-quantum key operation, but it must not describe ordinary FROST signatures as post-quantum secure.

## Repository structure

```text
.
├── Cargo.toml
├── cli/
│   └── src/main.rs
├── crates/
│   └── occ-core/
├── docs/
├── examples/
└── .github/workflows/ci.yml
```

## Quick start

Install Rust, open a terminal in the repository, and run:

```bash
cargo fmt --all
cargo test --workspace
cargo run -p occ-cli -- sample --output-dir examples/generated
cargo run -p occ-cli -- credential-check --credential examples/sample-credential.json
cargo run -p occ-cli -- policy-check \
  --policy examples/sample-policy.json \
  --approvals examples/sample-approvals.json
```

On Windows PowerShell, the final command can be written on one line:

```powershell
cargo run -p occ-cli -- policy-check --policy examples/sample-policy.json --approvals examples/sample-approvals.json
```

## Expected demonstration result

The sample policy requires two approvals from three authorized participants. The included approvals from Alice and Bob satisfy the policy, so the CLI should report:

```json
{
  "authorized": true,
  "valid_approvals": 2,
  "required_approvals": 2,
  "missing_approvals": 0,
  "approved_participants": [
    "alice",
    "bob"
  ],
  "ignored_approvals": 1
}
```

## Security notice

This project is under active development. Do not use this starter version to protect real identities, money, production credentials, government systems, or sensitive infrastructure.

Please report security concerns according to `SECURITY.md`. Do not publish suspected vulnerabilities in a public issue.

## Licence

Apache License 2.0.
