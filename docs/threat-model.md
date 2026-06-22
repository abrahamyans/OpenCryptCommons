# Initial Threat Model

This is an initial engineering document, not a security audit.

## Assets

- issuer signing keys;
- holder binding keys;
- threshold participant key shares;
- post-quantum signing keys;
- credential claims and disclosures;
- trust-policy configuration;
- audit and revocation information;
- software supply-chain integrity.

## Main actors

- issuer;
- holder;
- verifier;
- threshold participant;
- service operator;
- software maintainer;
- external attacker;
- malicious or compromised insider.

## Main threats

### Credential threats

- forged issuer signatures;
- altered claims or disclosures;
- presentation of another person's credential;
- replay of a valid presentation;
- acceptance of an expired or revoked credential;
- excessive disclosure of personal data;
- algorithm downgrade or confusion;
- maliciously structured JSON causing parser differences.

### Threshold threats

- unauthorized participant;
- duplicate approval counted more than once;
- replayed signing-round messages;
- malicious coordinator;
- missing or unavailable participants;
- leaked key share;
- incorrect trusted-dealer assumptions;
- mixing approvals for different operations.

### Post-quantum migration threats

- claiming post-quantum protection while still relying on a classical-only path;
- accepting experimental encodings without explicit versioning;
- very large keys or signatures causing denial of service;
- use of an unreviewed implementation;
- downgrade from ML-DSA to an unintended algorithm.

### Operational threats

- secrets committed to Git;
- secrets printed in logs;
- dependency compromise;
- unsigned or unverified releases;
- weak randomness;
- insecure backup or key recovery;
- unsafe default configuration.

## Initial mitigations

- use standards and reviewed libraries;
- explicit allow-lists for algorithms and issuers;
- holder binding and verifier-provided nonce;
- expiry and status checks;
- operation identifiers in threshold approval;
- unique participant counting;
- dependency lock files and CI checks;
- secret-scanning and code review;
- external security review before production guidance.

## Non-goals in Phase 1

Phase 1 does not protect real credentials or keys. It validates data structures and threshold policies only.
