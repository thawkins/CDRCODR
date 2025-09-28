````markdown
# Tasks: Create a coding CLI (cprcodr)

This tasks list was generated from the feature plan, data model, research, and quickstart for `specs/001-create-a-coding`.

Paths referenced are relative to the repository root. Tasks are numbered, dependency-ordered, and include parallelization hints `[P]` where safe.

Feature directory: `/home/thawkins/projects/rust/cprcodr/specs/001-create-a-coding`

---

T001 - Setup workspace and CI (setup)

- Purpose: Ensure the repo builds and tests run in CI; add any missing dev deps.
- Files/commands:
  - `crates/core/Cargo.toml` (verify dev-deps: `tempfile`, `assert_cmd`)
  - `.github/workflows/ci.yml` (create if missing)
- Acceptance: `cargo test --manifest-path crates/core/Cargo.toml` runs in CI.
- Notes: Run locally to confirm before pushing.

T002 - Add serialization unit tests for data models [P]

- Purpose: Ensure `ProjectConfig`, `Session`, `ArtifactMetadata`, and prompt/call log entries round-trip with serde.
- Files:
  - `crates/core/tests/test_session_serialization.rs` (exists - verify)
  - `crates/core/tests/test_artifact_metadata.rs` (exists - verify)
  - Add `crates/core/tests/test_project_config.rs`
- Acceptance: New tests compile and pass locally (`cargo test --manifest-path crates/core/Cargo.toml`).
- Dependency: T001

T003 - Implement full patch applier unit tests (TDD) [P]

- Purpose: Drive the patch applier behavior with tests first (insert, replace, delete, conflict, dry-run).
- Files to add/modify:
  - `crates/core/tests/patch_applier.rs` (extend with explicit tests):
    - test_apply_replace_multiline
    - test_apply_insert_at_end
    - test_apply_delete_range
    - test_conflicting_hunk_reports_conflict
    - test_dry_run_does_not_write
- Acceptance: Tests fail initially (red) so implementation can be developed.
- Dependency: T002 (models exist), T001

T004 - Implement patch applier (implementation) [X]

- Purpose: Replace the stub in `crates/core/src/patch.rs` with a robust implementation that:
  - applies hunks (1-based inclusive line ranges)
  - detects out-of-range or conflicting hunks and returns a `PatchReport` with conflict metadata
  - honors `dry_run` (does not write files)
- Files:
  - `crates/core/src/patch.rs` (edit)
- Acceptance: All `crates/core` tests pass; patch applier unit tests pass.
- Dependency: T003 (tests), T002

T005 - Add expected-original validation to hunks (optional, follow-up) [X]

- Purpose: Allow hunks to optionally include an `expected_original` snippet; when present, the applier validates the original content matches before applying.
- Files:
  - `crates/core/src/patch.rs` (extend Hunk struct)
  - Update `crates/core/tests/patch_applier.rs`
- Acceptance: New tests asserting validation behavior pass.
- Dependency: T004

T006 - Wire CLI commands: `gen`, `preview`, `apply` (CLI scaffolding)

- Purpose: Hook the CLI to core functionality and provide UX described in quickstart.
- Files:
  - `crates/cli/src/main.rs` (add subcommands)
  - `crates/cli/Cargo.toml` (ensure dependencies: `clap`, `serde_json`, `assert_cmd` as dev dep)
- Behavior:
  - `gen` → generates session artifacts via Backend and writes session file (or dry-run)
  - `preview <session_id>` → reads session artifacts and calls `apply_patch_to_working_tree` with `dry_run=true` and prints JSON report
  - `apply <session_id>` → calls applier with `dry_run=false`; if `--git-commit`, call `create_branch_and_commit`
- Acceptance: CLI integration tests compile and the CLI can run preview/apply flows against temp repo.
- Dependency: T004 (applier implemented)

T007 - CLI integration tests for preview/apply (TDD)

- Purpose: Add integration tests that exercise preview vs apply in a temporary git repo.
- Files:
  - `crates/cli/tests/apply_preview.rs`
- Tests:
  - preview returns a PatchReport and does not modify files
  - apply writes files and optionally commits when `--git-commit` used
- Acceptance: Integration tests pass in CI. Use `assert_cmd` and `tempfile`.
- Dependency: T006, T004

T008 - Add `--dry-run`, `--git-commit` flag support and safe defaults

- Purpose: Ensure `apply` defaults to safe behavior and flags are validated.
- Files:
  - `crates/cli/src/main.rs` (flag parsing and validation)
  - Update quickstart examples in `specs/001-create-a-coding/quickstart.md` if needed
- Acceptance: CLI unit/integration tests demonstrate correct behavior.
- Dependency: T006, T007

T009 - Validation runner (optional sandbox) [P]

- Purpose: Run user-provided `validate_cmd` (e.g., cargo test) in an isolated environment; recommend Docker via `--docker` flag.
- Files:
  - `crates/cli/src/validate.rs` (new helper)
  - Add tests under `crates/cli/tests/validate.rs`
- Acceptance: Validation runner supports optional Docker mode; test demonstrates sandboxed validation.
- Dependency: T006, T007

T010 - Adapter implementations & integration tests [P]

- Purpose: Implement concrete adapters and tests for `mock`, `ollama`, `lmstudio` backends.
- Files:
  - `crates/core/src/adapters/*` (mock adapter exists; add ollama & lmstudio adapters)
  - `crates/core/tests/*_adapter.rs` (adapter behavior tests)
- Acceptance: Mock adapter tests pass; ollama/lmstudio adapters provide basic HTTP/IPC wiring and have tests that can be skipped in CI if not available.
- Dependency: T002, T006

T011 - Documentation & quickstart polish [P]

- Purpose: Update `specs/001-create-a-coding/quickstart.md` and README snippets to match CLI flags and behavior.
- Files:
  - `specs/001-create-a-coding/quickstart.md` (update examples)
  - `README.md` examples for `gen`, `preview`, `apply`
- Acceptance: Examples are copy-pastable and match implemented CLI.
- Dependency: T006, T008

T012 - Housekeeping & polish

- Purpose: Lint, format, small fixes, CI badges, and prepare for code review.
- Files:
  - `Cargo.toml` (version bumps if required)
  - `.github/workflows/ci.yml` (ensure tests and lint run)
- Acceptance: All tests and linters pass in CI.

---

Parallel groups (examples):

- Group A [P]: T002, T003, T010 (serialization tests, applier tests, adapters' unit tests)
- Group B [P]: T006, T007, T008 (CLI wiring and integration tests)

How to run a task locally (example):

1. Run unit tests for core crate:

```bash
cargo test --manifest-path crates/core/Cargo.toml
```
````

2. Run a specific integration test (example):

```bash
cargo test --manifest-path crates/cli/Cargo.toml --test apply_preview
```

---

If you'd like, I can start immediately on T003 (add applier unit tests) or T004 (finish applier implementation). Currently `crates/core` has a basic applier implementation but more tests are needed to lock down semantics.

```
# Tasks (Phase 2) — TDD-first, ordered backlog

This file contains an ordered, TDD-first task list to implement the `001-create-a-coding` feature. Each task is written so tests/specs are created before implementation. Priorities map to the Claudecode gap analysis and the feature spec. Estimates are rough: S=Small (1-2d), M=Medium (2-5d), L=Large (1+ week).

Ordering principles

- TDD: Create failing tests first (unit → integration → e2e).
- Dependency order: core library (models, adapters) before CLI wiring and docs.
- Safety-first: Dry-run and preview behaviors must be test-covered before any apply/commit behavior.

Priority A — Safe apply & validation (High impact)

1. Task 001 — Add structured patch model and unit tests (S)

   - Create `core::patch::Patch` model (path, diff/unified-hunks, structured ops).
   - Tests: `crates/core/tests/patch_model.rs` (serialize/deserialize, round-trip, validation of hunk metadata).
   - Acceptance: tests fail (no implementation yet).

2. Task 002 — Implement safe patch applier library + unit tests (M)

   - Implement applier API: `apply_patch_to_working_tree(patch, workdir, dry_run) -> Result<Report>`.
   - Tests: unit tests for single-file replace/insert/delete, hunk conflict detection, dry-run returns preview only.
   - Files: `crates/core/src/patch.rs`, `crates/core/tests/patch_applier.rs`.

3. Task 003 — CLI integration: `preview` and `apply --yes|--git-commit` with tests (M)

   - Add CLI tests: `crates/cli/tests/apply_preview.rs` (uses a temp git repo), ensure `preview` doesn't change FS, `apply --yes` modifies FS, `apply --git-commit` creates a commit on a branch.
   - Acceptance: tests initially fail while wiring is missing; then implement CLI to make tests pass.

4. Task 004 — Git branch & commit helpers + tests (S)

   - Implement small git helper (spawn `git` for MVP) in `crates/core::git` with unit tests mocking command execution where possible.
   - Add tests verifying `--git-branch` creates branch and `--git-commit` includes a deterministic commit message.

5. Task 005 — Document apply safety and default behavior (S)
   - Add `specs/001-create-a-coding/apply.md` documenting default preview/dry-run behavior and flags.

Priority A — Validation runner (Medium)

6. Task 006 — Add configurable validation command and tests (S)

   - Extend `ProjectConfig` with `validate_cmd` (defaults to `cargo test` for Rust projects).
   - Unit tests for config parsing: `crates/core/tests/test_config.rs` (ensure `validate_cmd` parsed and accessible).

7. Task 007 — Implement `--validate` CLI flag and run harness (M)

   - CLI `gen --validate` or `apply --validate` runs `validate_cmd` in the workspace and returns structured result (exit code, stdout/stderr) captured in session artifacts.
   - Tests: integration test `crates/cli/tests/validate_runner.rs` using a temp project with a failing test, verify the runner reports failures.

8. Task 008 — Add optional `--docker` sandbox execution and tests (L)
   - Document and add `--docker` behavior that will `docker run` a container to run `validate_cmd` when requested.
   - Tests: smoke tests that skip if Docker not present (mark as optional in CI). Add gating logic in tests to run only when DOCKER_TESTS=1.

Priority B — Streaming, async migration, and adapters (Medium)

9. Task 009 — Define streaming output event schema and tests (S)

   - Create schema `GenerationEvent { kind, payload, sequence, partial }` and unit tests under `crates/core/tests/streaming.rs`.

10. Task 010 — Adapter async/streaming interface (M)

    - Migrate `Backend` to `AsyncBackend` with streaming variant (use `tokio` + `async-trait`).
    - Add tests and a minimal `AsyncMockAdapter` streaming test under `crates/core/tests/async_adapter.rs`.

11. Task 011 — Update Ollama/LMStudio adapters to support streaming (M-L)
    - Implement streaming semantics for each adapter (where host supports it), and tests verifying partial event assembly.

Priority B — Structured patch applier features & conflict handling (Medium)

12. Task 012 — Preview UI improvements & hunk-level acceptance (S)
    - Add CLI flag `--accept-hunks` to `apply` to accept only non-conflicting hunks; tests ensure conflicts are reported.

Priority C — Mocks, CI, docs, and connectors (Low→High)

13. Task 013 — Harden MockAdapter and persistence tests (S)

    - Add tests ensuring `--dry-run` does not advance `mock_state.json` (unit/integration tests already exist; extend if needed).

14. Task 014 — Add e2e tests for intermittent failure flows (M)

    - Use temp workdir and sequential mock state to reproduce intermittent failures across process restarts; tests in `crates/cli/tests/intermittent_e2e.rs`.

15. Task 015 — Docs: Quickstart and enterprise/security pages (S)

    - Update `specs/001-create-a-coding/quickstart.md` with apply/validate examples; add `docs/security.md` and `docs/enterprise.md` placeholders.

16. Task 016 — Add example MCP connector & plugin API design doc (L)

    - Design a plugin interface for external data connectors; implement a file-fetcher plugin and tests.

17. Task 017 — CI updates to run new tests and optional Docker harness (S)

    - Update GitHub Actions to include optional Docker test job and to run the new integration suites.

18. Task 018 — Accessibility, UX polish & telemetry (future)
    - Add telemetry opt-in, improve CLI UX, and add additional acceptance tests.

How to execute (developer workflow)

- Create one PR per 2–4 tasks; prefer small, focused PRs that add tests + implementation.
- For each task: write failing tests, run `cargo test`, implement code, run `cargo test` until green.

Estimated total scope: 3–6 weeks for a small team (2 engineers) to reach a usable MVP with apply+validation+git-commit support and basic streaming.

If you want, I can now (a) generate the specific test scaffolding files (failing tests) for the top 4 tasks, or (b) open PR diffs for the first few tasks. Reply with "scaffold tests" or "start task 001" to proceed.

# Tasks: Coding CLI (cprcodr)

**Input**: design docs from `/home/thawkins/projects/rust/cprcodr/specs/001-create-a-coding/`
**Prerequisites**: `plan.md`, `research.md`, `data-model.md`, `contracts/`

## Phase 3.1: Setup (must run first)

- T001 Initialize repository workspace and CLI crate

  - Create Cargo workspace at `/Cargo.toml` with members: `crates/cli`, `crates/core`
  - Create `crates/cli/Cargo.toml` and `crates/cli/src/main.rs` with a placeholder `main()` that prints help
  - Files: `/Cargo.toml`, `/crates/cli/Cargo.toml`, `/crates/cli/src/main.rs`
  - Success: `cargo build --workspace` completes (placeholder build OK)

- T002 Add toolchain and formatting config

  - Add `/rust-toolchain.toml` pinning stable toolchain and components `rustfmt` and `clippy`
  - Add `rustfmt.toml` if needed
  - Files: `/rust-toolchain.toml`, `/rustfmt.toml`

- T003 Configure CI to enforce constitution rules

  - Add `.github/workflows/ci.yml` with steps:
    - checkout, setup Rust toolchain
    - run `cargo fmt -- --check`
    - run `cargo clippy -- -D warnings`
    - run `cargo test --workspace`
    - run `cargo audit` (optional step/failure policy configurable)
  - Files: `.github/workflows/ci.yml`

- T004 Add Docker helpers for sandboxed execution (optional)
  - Add `scripts/run-in-docker.sh` that: builds a small Docker image, mounts project, runs command passed as arg
  - Files: `/scripts/run-in-docker.sh`, `/Dockerfile.tools` (small base image)

## Phase 3.2: Tests First (TDD) ⚠️ MUST COMPLETE BEFORE IMPLEMENTATION

CRITICAL: Create failing tests that reflect the contracts and behaviors in the spec. Tests MUST fail before implementation tasks begin.

-- T010 [P] Create contract test for `/generate` (failing) [X]

- Path: `/tests/contract/test_generate.rs`
- Behavior: POST to local CLI adapter generate endpoint (mocked) with sample prompt and assert JSON shape matches `/specs/001-create-a-coding/contracts/cprcodr-api-openapi.yaml` `200` response schema (artifacts array)
- Test must fail (no implementation yet)

-- T011 [P] Create contract test for `/session` (failing) [X]

- Path: `/tests/contract/test_session.rs`
- Behavior: POST to create session, assert `session_id` returned and valid format
- Test must fail

-- T012 [P] Add unit tests for `ProjectConfig` parsing (failing) [X]

- Path: `/crates/core/tests/test_config.rs`
- Behavior: attempt to parse several `cprcodr.toml` samples (empty, minimal, full) and assert expected fields or errors
- Test must fail

-- T013 [P] Add integration test for `cprcodr init` behavior (failing) [X]

- Path: `/tests/integration/test_init.rs`
- Behavior: run `crates/cli/target/debug/cprcodr init` in a temp dir and assert created files: `cprcodr.toml`, `.cprcodr/session/` directory skeleton
- Test must fail

## Phase 3.3: Core Implementation (only after tests fail)

- T020 Implement `crates/core` crate: core data types and ProjectConfig

  - Files: `/crates/core/src/lib.rs`, `/crates/core/src/config.rs`
  - Implement types: ProjectConfig, Session, Artifact and their (de)serialization
  - Unit tests should pass for parsing example configs

- T021 Implement backend adapter trait and Ollama adapter

  - Files: `/crates/core/src/backend.rs`, `/crates/core/src/adapters/ollama.rs`
  - Provide interface: `trait Backend { fn generate(...) -> Result<Artifacts> }`
  - Ollama adapter implements trait and accepts URL + optional API key from config

- T022 Implement LMStudio adapter

  - Files: `/crates/core/src/adapters/lmstudio.rs`
  - Same interface as Ollama adapter

- T023 Implement CLI command handlers in `crates/cli`

  - Files: `/crates/cli/src/main.rs`, `/crates/cli/src/commands.rs`
  - Commands to implement: `init`, `gen`, `preview`, `apply`, `session`
  - Wire commands to call into `crates/core` adapters

- T024 Implement session persistence and artifact storage
  - Files: `/crates/core/src/session.rs`, `.cprcodr/session/*`
  - Persist session metadata and artifact pointers

## Phase 3.4: Integration & Infrastructure

- T030 Connect CLI to local backend endpoints and mock adapters for tests

  - Files: tests and adapter mock files under `/tests/helpers/`
  - Ensure contract tests exercise the adapters

- T031 Add logging, structured output, and error formatting
  - Files: `/crates/core/src/logging.rs`, add `tracing` config in CLI

## Phase 3.5: Polish

- T040 [P] Add unit tests for core logic and validation (e.g., config, session)

  - Files: `/crates/core/tests/*`

- T041 [P] Add performance benchmarks for generation latency

  - Files: `/crates/core/benches/*`

- T042 [P] Update docs and quickstart with concrete commands
  - Files: `/docs/quickstart.md`, update `/specs/001-create-a-coding/quickstart.md` if needed

## Parallel execution groups (examples)

- Group A (run in parallel): T010, T011, T012, T013 (contract/unit/integration tests)
- Group B (run in parallel after core lib exists): T020 (core) and T021/T022 adapters can be developed in parallel but T020 should start first

## Dependency notes

- Setup (T001-T004) must complete before tests are runnable.
- Tests (T010-T013) must exist and fail before implementation tasks T020-T024.
- Core crate (T020) must be implemented before CLI wiring (T023) and session persistence (T024).

## Task agent commands (examples)

- Example task agent for T010:

  - "Create a Rust contract test at `/tests/contract/test_generate.rs` that sends a
    POST with a sample prompt to the generate handler and asserts the response
    contains an `artifacts` array with objects having `path`, `type`, and `summary` fields. Test should currently fail."

- Example task agent for T020:
  - "Implement the `ProjectConfig` struct in `/crates/core/src/config.rs` with
    serde (de)serialization and add parsing tests at `/crates/core/tests/test_config.rs`."

## Validation checklist (final)

- [ ] All contract files have failing tests in `/tests/contract/`
- [ ] All data-model entities have model creation tasks in `/crates/core/`
- [ ] TDD order enforced: tests exist and fail before implementation
- [ ] Each task includes explicit file path and success criteria
```
