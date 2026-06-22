# Contributing to OpenCryptCommons

Thank you for your interest in OpenCryptCommons.

## Current status

## Current status

OpenCryptCommons is in its early implementation phase. The repository currently contains the Rust workspace foundation, core data structures, policy validation, command-line tooling, tests, and initial design documentation.

Cryptographic provider implementations are not yet production-ready. Contributions should clearly distinguish between non-cryptographic architecture work and security-sensitive cryptographic implementation.

## Helpful contributions at this stage

Useful contributions in the current phase include:

- feedback on project scope and architecture
- references to relevant standards and open implementations
- review of threat-model assumptions
- suggestions for real-world integration targets
- identification of adjacent projects or prior art
- documentation and wording improvements

## Before opening an issue

Please check whether your suggestion is already reflected in the README or documentation files.

## Expected contribution areas in the first implementation phase

- documentation
- roadmap review
- interoperability notes
- example use cases
- threat model feedback

## Code contributions

## Code contributions

Code contributions are welcome through pull requests.

Before submitting code:

1. open or reference a GitHub Issue;
2. explain the purpose and security impact of the change;
3. add or update tests;
4. run `cargo fmt --all`;
5. run `cargo clippy --workspace --all-targets --all-features`;
6. run `cargo test --workspace --all-features`;
7. do not include passwords, tokens, private keys, or real personal credentials.

Changes involving cryptographic algorithms, key handling, wire formats, or verification rules require additional technical review.

## Communication

Please use GitHub Issues for technical feedback, questions, and suggestions.
