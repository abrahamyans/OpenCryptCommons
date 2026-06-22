# Roadmap

## Phase 0 — Public project framing

- Publish repository scope, governance, licence, and security policy.
- Collect early review comments.
- Identify one realistic integration example.

## Phase 1 — Engineering foundation

- Create the Rust workspace.
- Define core credential and policy types.
- Define cryptographic provider traits.
- Add a CLI skeleton, samples, tests, and continuous integration.
- Publish the initial threat model and standards policy.

**Exit condition:** `cargo test --workspace` passes and the non-cryptographic CLI demonstration works.

## Phase 2 — Selective-disclosure credential flow

- Select an RFC 9901-compatible Rust implementation after review.
- Add issuer, holder, presentation, and verifier adapters.
- Add holder key binding.
- Add positive and negative test vectors.
- Clearly document supported and unsupported SD-JWT features.

**Exit condition:** an end-to-end local issuance, selective presentation, and verification demonstration passes published test vectors.

## Phase 3 — Threshold trust

- Integrate an RFC 9591 FROST implementation for an approved classical ciphersuite.
- Separate key generation, signing rounds, transport, and aggregation.
- Add replay protection, participant identity checks, and failure handling.
- Document trusted-dealer and distributed-key-generation assumptions.

**Exit condition:** a test demonstration performs a threshold signature and verifies it with the group public key.

## Phase 4 — Post-quantum provider

- Integrate an audited or appropriately reviewed FIPS 204 ML-DSA implementation.
- Add explicit key and signature encodings.
- Add algorithm allow-lists and migration metadata.
- Benchmark key size, signature size, memory, and runtime.
- Do not describe ML-DSA as threshold signing unless a separately reviewed threshold construction is adopted.

**Exit condition:** ML-DSA test vectors pass and the provider is independently reviewed.

## Phase 5 — Reference service integration

- Build one internet-facing demonstration.
- Add deployment documentation and interoperability notes.
- Add misuse-resistant defaults and operational guidance.
- Conduct an independent security review.
- Resolve findings and publish a versioned release.
