# Research: Create a coding CLI (cprcodr)

This research file collects decisions and assumptions required to implement the
`001-create-a-coding` feature. Several clarifications in the feature spec were
left open; to unblock implementation the following sensible defaults were used
— these are documented so they can be changed later.

## Key Clarifications & Assumptions

- Session model: persist sessions by default as JSON under the resolved output
  directory (`CPRCODR_OUTPUT` or `.cprcodr/session`). A Session will contain:
  - id (uuid), created_at, project_id (cwd), selected backend, selected model,
    prompt history (list), artifacts metadata (paths, summaries), and a small
    call log with timestamps. Sessions are retained indefinitely by default but
    users may delete them using `cprcodr session rm` or set a retention policy
    later in configuration.
- Credentials: assume users will typically connect to either managed services or
  self-hosted endpoints. Credentials are provided via environment variables by
  default. We include a later task to add an encrypted local store or keyring
  integration (optional).
- Execution sandbox: the default validation/run will be opt-in and run as a
  local subprocess; for safety, the CLI will recommend Docker and offer a
  `--docker` flag to run validations inside a container. Sandboxing by default
  is out of scope for the MVP.
- Apply/edits behavior: `apply` will write files to disk by default when the
  user explicitly runs it. A `--dry-run` will print the patch. A `--git-commit`
  flag will create a branch and commit changes (requires `git` on PATH).

## Research Notes

- Terminal-first UX: Claude Code emphasizes a terminal-first workflow. For
  parity, cprcodr should prioritize composable CLI outputs (JSON mode), and a
  streaming option for long-running generations. The current CLI already
  supports machine-readable output and `--dry-run` semantics and will be
  extended where needed.
- Deterministic testing: To enable reproducible tests, we will keep the
  `MockAdapter` and its persistence mechanism (mock_state.json). Async
  migration is planned and will be implemented separately.

## Decisions

- Persist sessions as JSON files under the resolved session directory (simple
  file-based persistence meets the library-first and test-first constraints).
- Credentials via environment variables for MVP with an opt-in encrypted
  store task in Phase 2.
- Execution validation will be opt-in and support a `--docker` flag for safer
  runs.

## Output

Decisions above will be used to generate Phase 1 artifacts (data model,
contracts, quickstart) and to update plan.md progress tracking.

```markdown
# Research: Coding CLI (cprcodr-coding-cli)

**Path**: /home/thawkins/projects/rust/cprcodr/specs/001-create-a-coding/research.md
**Date**: 2025-09-28
**Related spec**: /home/thawkins/projects/rust/cprcodr/specs/001-create-a-coding/spec.md

## Purpose

Collect decisions and resolve clarifications required to implement the Coding CLI
feature. This research file records assumptions taken so planning can proceed.

## Unresolved clarifications (from spec)

- Ollama / LMStudio hosting model (self-hosted vs managed): NO ANSWER PROVIDED
- Credential storage approach (env | keychain | encrypted file): NO ANSWER PROVIDED
- Execution model for generated code (docker | local | none): NO ANSWER PROVIDED
- Session persistence (ephemeral | project-scoped | user-profile): NO ANSWER PROVIDED
- Default backend/model selection UX: NO ANSWER PROVIDED

## Assumptions (made to proceed with planning)

To avoid blocking the plan phase the following explicit assumptions are adopted
for now. They are recorded so they can be revisited in a follow-up research or
clarification step.

1. Hosting model: Support both self-hosted and managed endpoints. Default
   assumption for initial implementation: targets are self-hosted local hosts
   (user runs Ollama/LMStudio locally) with endpoints configurable via
   `cprcodr.toml`. Managed endpoints supported via hostname/API key in config.
   Rationale: maximizes flexibility and aligns with the common usage patterns of
   Ollama/LMStudio.

2. Credential storage: Use environment variables for quick usage and support an
   encrypted local file (AES-GCM) for per-project credentials, with optional
   OS keychain integration planned as follow-up. Rationale: env vars are
   simplest; encrypted file gives a secure default for local workflows.

3. Execution model: Generated code will NOT be executed automatically by
   default. Provide optional commands to run generated code inside a Docker
   container (recommended) or directly when user explicitly requests it.
   Rationale: safer default; provides sandbox option for users who want it.

4. Session persistence: Sessions are project-scoped by default (session data
   saved under `.cprcodr/session/` in the project) and expire after a
   configurable TTL (default 24h). Rationale: project-scoped sessions make it
   simple to reproduce and share state for a given workspace.

5. Default backend selection UX: Default backend is per-project in
   `cprcodr.toml` but can be overridden per-request via `--backend`. When no
   default exists, CLI prompts interactively once and offers `--yes` for
   non-interactive flows.

## Alternatives considered

- Force self-hosted only: rejected since managed endpoints are a common use-case
  and adding flexible config is low-cost.
- Secrets only via OS keychain: ideal but adds cross-platform complexity; plan
  for optional keychain integration later.

## Decisions for Phase 1

- Proceed with a flexible backend adapter design: abstract backend interface
  with concrete adapters for Ollama and LMStudio.
- Use `cprcodr.toml` for project config with fields for default backend, model,
  and execution preferences.
- Provide Docker-based run helpers for safe execution in addition to optional
  local execution.

## Next steps

1. Phase 1 design: derive data model from requirements and draft contracts for
   the CLI API and JSON outputs.
2. Produce quickstart and failing contract tests to drive implementation (TDD).
```
