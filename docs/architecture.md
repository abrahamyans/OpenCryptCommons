# Architecture

## Purpose

The first architectural goal is to define stable boundaries around cryptographic functionality. Application code must not depend directly on one signing algorithm or credential implementation.

## Planned workspace

```text
crates/
  occ-core/        Shared data types, policies, errors, and backend traits
  occ-sd-jwt/      Future RFC 9901 issuance, presentation, and verification adapter
  occ-threshold/   Future RFC 9591 FROST adapter
  occ-pq/          Future NIST FIPS 204 ML-DSA adapter
cli/               Human- and script-friendly command-line interface
examples/          Example data and future service integration
tests/             Interoperability and regression tests
docs/              Architecture, threat model, standards, and operational guidance
```

Only `occ-core` and the CLI foundation are included in the Phase 1 starter.

## Layering

1. **Domain layer**  
   Credential metadata, trust policies, approvals, and validation rules.

2. **Cryptographic provider layer**  
   Narrow traits for signing, selective disclosure, and threshold operations.

3. **Workflow layer**  
   Issuer, holder, verifier, and threshold-approval workflows.

4. **Interface layer**  
   CLI, future HTTP service adapter, and integration examples.

## Mandatory design rules

- Do not invent cryptographic primitives.
- Keep secret keys out of logs, errors, test fixtures, and Git.
- Use explicit algorithm identifiers and versioned formats.
- Treat parsing, verification, and policy authorization as separate steps.
- Fail closed when an algorithm, version, issuer, or policy is unsupported.
- Keep FROST and ML-DSA security claims separate.
- Require external review before recommending production use.
- Forbid unsafe Rust in project-owned crates unless a documented exception is approved.

## Crypto-agility

Crypto-agility means that callers use a stable OpenCryptCommons interface while approved backends can change. It does not mean silently accepting any algorithm. Every credential and signature must carry an explicit, allow-listed algorithm and version.
