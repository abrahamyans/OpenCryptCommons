# Standards and Implementation Policy

## Target standards

### W3C Verifiable Credentials Data Model 2.0

Used for the high-level issuer, holder, verifier, credential, and presentation model.

### RFC 9901 — Selective Disclosure for JSON Web Tokens

Used for selective disclosure of JSON claims and optional holder key binding.

### RFC 9591 — FROST

Used for a classical two-round threshold Schnorr signature demonstration.

### NIST FIPS 204 — ML-DSA

Used for a post-quantum digital-signature provider.

## Important separation

RFC 9591 FROST ciphersuites use classical groups. ML-DSA is post-quantum but is not automatically threshold. OpenCryptCommons must represent these as separate capabilities.

A safe first architecture can support:

- selective-disclosure credentials signed by an approved issuer suite;
- threshold authorization for sensitive administrative decisions;
- an independent ML-DSA provider for post-quantum signing experiments and migration testing.

## Library-selection checklist

Before adding a cryptographic dependency, record:

- specification and exact version implemented;
- licence compatibility;
- maintenance activity;
- test-vector coverage;
- unsafe-code use;
- external audit or review status;
- supported platforms;
- key and signature encoding;
- randomness requirements;
- known limitations and security advisories.

## Dependency rule

Cryptographic dependency versions should be pinned in `Cargo.lock` for application and demonstration builds. Updates require test-vector and interoperability reruns.
