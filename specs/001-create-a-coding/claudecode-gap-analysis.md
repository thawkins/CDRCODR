# Gap Analysis: Claudecode (official product) vs cprcodr (this project)

This document lists functionality present in the Claudecode product (per
https://docs.claude.com/en/docs/claude-code/overview) that is missing from
the cprcodr project, along with recommended next steps and rough effort
estimates. Priorities favor high impact and low-to-medium risk items first.

Summary of Claudecode capabilities (from official docs)

- Terminal-first CLI that can edit files, run commands, and create commits.
- Build features from descriptions: plan, generate code, and validate (run tests).
- Debug and fix issues across the codebase.
- Project awareness and external data connectors (MCP) for third-party sources.
- Automations: lint fixes, merge conflict resolution, release notes.
- Composable, scriptable CLI with streaming and piping support.
- IDE integrations and enterprise hosting/security guidance.

Mapping: what's missing in cprcodr today

1. Direct, safe file edits + commit/PR flow

- Missing: ability to accept an LLM-produced patch and apply it safely, with
  optional automatic git commit/PR creation.
- Why: Claudecode's primary value is taking action; cprcodr currently writes
  artifact stubs and offers `apply`, but lacks a safe patch/commit flow.
- Recommended next steps: implement `apply --patch` format support (unified
  diff or structured edit objects), default to `--dry-run`, add `--git-branch`
  and `--git-commit` flags that create a branch and commit. Use `git` via
  spawn for MVP, consider `git2` for richer integration later.
- Effort: Medium

2. Validation/run pipeline (execute tests/lints)

- Missing: automatic validation hook (run project's test/lint commands after
  generation) and report back results.
- Why: Claude Code runs and ensures code works; this increases developer
  confidence in generated patches.
- Recommended: add `--validate` flag to `gen` which runs configured `validate_cmd`
  (from `cprcodr.toml`, e.g., `cargo test`) in a spawn or `--docker` sandbox.
- Effort: Medium

3. Streaming output / composability

- Missing: streaming incremental outputs (JSONL or event stream) to support
  piping and long-running generation observability.
- Recommended: extend adapter contract to optionally yield partially-complete
  artifact events; add `--stream` and machine-readable `--format jsonl` output.
- Effort: Small–Medium (larger when combined with async migration)

4. In-place code edits & patch application semantics (editor-style)

- Missing: structured edit application (apply hunks, insert/replace) and
  conflict handling for existing files.
- Recommended: support structured patch format for artifacts (path + patch)
  and implement a safe applier that can preview hunks and fallback on `--dry-run`.
- Effort: Medium

5. External data & MCP-like connectors

- Missing: connectors to external knowledge sources (Drive, Slack, Figma) and
  an MCP-style plugin system.
- Recommended: design a small plugin API (local executables or HTTP adapters)
  and implement one or two connectors (e.g., file fetcher) as examples.
- Effort: Large

6. IDE integrations

- Missing: IDE/editor plugins and language server integrations.
- Recommended: provide an example VSCode extension later, but postpone until
  the CLI and core flows stabilize.
- Effort: Large

7. Enterprise/security documentation & hosting guidance

- Missing: hosted deployment guidance, security/data usage docs.
- Recommended: add `docs/enterprise.md` and `docs/security.md` with hosting
  and data-handling best practices; re-use assumptions from research.md.
- Effort: Small

8. Automation workflows (lint fixes, merge conflict resolution, release notes)

- Missing: opinionated automations for repetitive tasks.
- Recommended: add discrete commands `cprcodr lint-fix`, `cprcodr release-notes`
  that orchestrate LLM prompts and create commits. Start with `release-notes`.
- Effort: Medium

Prioritization (short list)

- Priority A (implement ASAP): safe patch apply with dry-run + git commit flags;
  validation runner (`--validate`). (Medium each)
- Priority B: streaming & JSONL output; structured patch applier; enterprise docs.
- Priority C: MCP connectors, IDE plugins, advanced automations.

How gaps map to plan phases

- Phase 0 (research): already covered (research.md contains defaults and
  security choices).
- Phase 1 (design): data model was generated and includes Session model used
  for the patch/commit flow.
- Phase 2 (tasks): tasks.md should contain TDD-first tasks for each priority
  (create failing tests first, then implement apply/validate/streaming features).

Suggested immediate actions (next 1-2 days)

1. Implement safe patch apply API in `crates/core` as a library (unit tests).
2. Add CLI flags (`--dry-run`, `--git-branch`, `--git-commit`) and tests.
3. Add `--validate` that runs configured `validate_cmd` with optional `--docker`.

If you want, I can now generate the `/tasks` phase (tasks.md) containing a
TDD-ordered task list that incorporates the above prioritized work items. I
did not create `tasks.md` yet because the plan template expects the `/tasks`
command to run it. Reply "generate tasks" and I'll create `tasks.md` under
`/specs/001-create-a-coding/tasks.md` (TDD-first tasks, estimated effort,
numbered and ordered).
