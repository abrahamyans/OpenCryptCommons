# Minimum Viable Product Scope

## User story

A developer can run a local example in which:

1. an issuer creates a standards-based selective-disclosure credential;
2. a holder receives it;
3. a verifier requests only selected claims;
4. the holder creates a presentation;
5. the verifier validates the issuer signature, disclosures, timing, and holder binding;
6. a separate threshold policy controls one sensitive administrative operation;
7. the project demonstrates a post-quantum signature provider as an independent crypto-agility option.

## Included in the MVP

- Rust library APIs;
- CLI commands;
- RFC 9901 selective-disclosure flow;
- one FROST threshold-signing demonstration;
- one ML-DSA signing and verification demonstration;
- test vectors and negative tests;
- explicit algorithm and version metadata;
- one reference integration;
- documentation and threat model.

## Excluded from the MVP

- a national identity platform;
- biometric processing;
- blockchain requirements;
- custom cryptographic algorithms;
- anonymous credentials beyond the selected standard;
- production hardware-security-module integration;
- a claim that FROST is post-quantum;
- a claim of production security before external review.
