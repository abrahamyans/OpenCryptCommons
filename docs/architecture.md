# Architecture Notes

## Intended repository structure

- `crates/` — reusable Rust crates
- `cli/` — command-line tools and utilities
- `examples/` — example integrations and demos
- `tests/` — interoperability and regression tests
- `docs/` — design notes, roadmap, threat model, and project documentation

## Architectural direction

The project is expected to follow a layered approach:

1. a small memory-safe core for trust flows and credential handling
2. thin interfaces for command-line and service integration
3. documentation and examples designed for non-specialist developers

## Design constraints

- standards-aware implementation choices
- modular and auditable structure
- minimal unnecessary complexity
- developer usability as a first-order requirement
