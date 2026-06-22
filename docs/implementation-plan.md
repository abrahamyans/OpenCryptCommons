# Practical Implementation Plan

## Work package 1 — Foundation

Status: included in this starter package.

Deliverables:

- Rust workspace;
- `occ-core`;
- CLI;
- sample files;
- tests;
- CI;
- architecture and threat-model documents.

## Work package 2 — RFC 9901 adapter

Tasks:

1. Compare actively maintained Rust SD-JWT libraries.
2. Record the selection in an Architecture Decision Record.
3. Implement issuer, holder, and verifier adapters.
4. Require issuer-signature verification.
5. Add holder key binding and verifier nonce support.
6. Add RFC examples and negative tests.
7. Add CLI commands:

```text
occ credential issue
occ credential present
occ credential verify
```

## Work package 3 — FROST adapter

Tasks:

1. Select an RFC 9591-compatible implementation.
2. Begin with a local simulation; do not pretend it is a distributed deployment.
3. Add participant identifiers, signing-package serialization, and replay protection.
4. Add CLI commands:

```text
occ threshold dealer-keygen
occ threshold round1
occ threshold round2
occ threshold aggregate
occ threshold verify
```

5. Document the trusted-dealer limitation before adding distributed key generation.

## Work package 4 — ML-DSA adapter

Tasks:

1. Select a FIPS 204 implementation after review.
2. Support one parameter set first.
3. Add deterministic test vectors and explicit binary encodings.
4. Add CLI commands:

```text
occ pq keygen
occ pq sign
occ pq verify
```

5. Benchmark and document key and signature sizes.

## Work package 5 — Integration example

Recommended first example:

A small local web service requests proof that a holder has a role such as `researcher`, without requesting unrelated claims. A sensitive issuer-administration operation requires threshold approval. ML-DSA is demonstrated separately as a migration-capable signing provider.

## Work package 6 — Security review and release

- freeze the intended release scope;
- prepare review documentation;
- commission external review;
- correct findings;
- publish signed source and binary releases;
- clearly label the security and maturity level.
