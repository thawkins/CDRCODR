<!--
SYNC IMPACT REPORT
Version change: none -> 1.0.0
Modified principles:
- template placeholder 1 -> Library-First (Rust crates first)
- template placeholder 2 -> CLI & Library Boundary
- template placeholder 3 -> Test-First (TDD) (NON-NEGOTIABLE)
- template placeholder 4 -> Integration & Contract Testing
- template placeholder 5 -> Observability, Versioning & Simplicity
Added sections:
- Constraints & Requirements
- Development Workflow
Removed sections:
- none
Templates requiring updates:
- .specify/templates/plan-template.md ✅ updated
- .specify/templates/spec-template.md ✅ updated
- .specify/templates/tasks-template.md ✅ updated
Follow-up TODOs:
- RATIFICATION_DATE: TODO(RATIFICATION_DATE): set original adoption date
-->

# cprcodr Constitution

## Core Principles

### Library-First

Every new capability MUST begin as a well-scoped Rust crate (library) within the workspace
where practical. Libraries MUST be independently testable, documented, and publishable.
Binary/CLI projects may depend on crates in the workspace but MUST not contain core logic
only expressed in binaries. Rationale: crate-first promotes reuse, small public APIs,
and makes testing and benchmarking straightforward in Rust.

### CLI & Library Boundary

Public functionality MUST be exposed via library APIs first and, where a CLI is
required, a thin binary wrapper MUST be provided that calls into library code.
Command-line interfaces SHOULD support both human-readable and machine-readable
outputs (e.g., JSON). Rationale: clear separation of concerns simplifies testing
and enables programmatic use of the project.

### Test-First (NON-NEGOTIABLE)

Tests are REQUIRED before significant behavior changes. Feature development
MUST follow a red-green-refactor cycle: write failing automated tests (unit and
integration), implement until they pass, then refactor. All code merged to main
MUST pass the full test suite. Rationale: Rust's strong type system complements
TDD by catching many classes of error early; TDD ensures regressions are avoided.

### Integration & Contract Testing

Integration tests and contract tests are REQUIRED for cross-crate boundaries,
external interfaces, and any IO/serialization formats. Contracts (public types,
JSON schemas, and CLI protocols) MUST be explicitly documented and have tests
verifying backward compatibility where applicable. Rationale: prevents silent
breaks for downstream consumers.

### Observability, Versioning & Simplicity

Structured logging (e.g., JSON or key=value pairs) and explicit error handling
MUST be used to aid debugging. Versioning MUST follow semantic versioning for
public crates (MAJOR.MINOR.PATCH). Dependency policy: the project will track and
adopt the latest stable releases of cargo crates; dependency updates MUST be
reviewed and CI-verified (see Governance). Keep designs simple: prefer
composability over cleverness. Rationale: predictable versions and clear
observability make production usage and debugging feasible.

## Constraints & Requirements

- Language / Tooling: Rust (stable, latest). Use Cargo for packaging and
  dependency management. Toolchains should be recorded in plans (rust-toolchain).
- Testing: `cargo test` for unit/integration tests; `cargo bench` (Criterion)
  for performance benchmarks where needed.
- Linting & Formatting: `cargo fmt` and `cargo clippy` are REQUIRED in CI.
- Security: run `cargo audit` in CI; address high/critical advisories before
  merging releases.

## Development Workflow

- PRs: Every change MUST have a linked issue or feature spec. PRs MUST pass
  CI (tests, clippy, fmt, audit) before merging. Large changes SHOULD include
  a migration plan for consumers.
- Commits: Use conventional, clear messages. Version bumps for public crates
  MUST follow semver and include a CHANGELOG entry.
- Releases: Tag releases and publish as needed. For published crates, ensure
  the Cargo.toml has correct metadata and the public API has compatibility tests.

## Governance

Amendments: This constitution MAY be amended by a documented proposal (PR)
that describes the change, migration steps, and tests. A MINOR or MAJOR
governance change requires review by at least two maintainers and adoption
documented in the PR. Emergency fixes (security or legal) can be merged with
maintainer consensus and recorded in the follow-up PR.

Versioning policy: Governance changes follow semantic versioning per the
Constitution: MAJOR for incompatible governance/principle removals or
redefinitions; MINOR for added principles or materially expanded guidance;
PATCH for wording/typo fixes.

Compliance: All PRs MUST include a Constitution Check section (see
`.specify/templates/plan-template.md`) and document how the change complies
or provide an explicit justification where a principle is intentionally
deviated from.

**Version**: 1.0.0 | **Ratified**: TODO(RATIFICATION_DATE) | **Last Amended**: 2025-09-28
