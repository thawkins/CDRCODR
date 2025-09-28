# Data Model for cprcodr coding CLI

This document captures the primary entities required by the feature and their
key attributes. The focus is on serializable, testable data shapes (worked in
Rust using serde).

## Entities

### ProjectConfig

- Purpose: store per-project defaults and preferences
- Storage: `cprcodr.toml` in project root
- Fields:
  - `default_backend: String` (e.g., "ollama" | "lmstudio" | "mock")
  - `default_model: Option<String>`
  - `output_dir: Option<String>`
  - `validate_cmd: Option<String>` (e.g., "cargo test")

### Session

- Purpose: interactive session metadata and history
- Storage: JSON file `SESSIONS_DIR/{session_id}.json`
- Fields:
  - `id: Uuid`
  - `created_at: String` (ISO-8601)
  - `project_id: String` (cwd)
  - `backend: String`
  - `model: Option<String>`
  - `prompts: Vec<PromptEntry>`
  - `artifacts: Vec<ArtifactMetadata>`
  - `call_log: Vec<CallLogEntry>`

#### PromptEntry

- `prompt_text: String`
- `response_summary: Option<String>`
- `timestamp: String`

#### CallLogEntry

- `timestamp: String`
- `duration_ms: u64`
- `status: String` ("ok" | "error")

### ArtifactMetadata

- `path: String`
- `summary: String`
- `checksum: Option<String>`

## Validation & Persistence

- Use serde_json for session files, and toml for `cprcodr.toml` config.
- All entities must have unit tests for (de)serialization round-trip.

```markdown
# Data Model: Coding CLI (cprcodr)

**Path**: /home/thawkins/projects/rust/cprcodr/specs/001-create-a-coding/data-model.md
**Date**: 2025-09-28

## Entities

- ProjectConfig

  - id: string (project path or generated id)
  - default_backend: enum[ollama,lmstudio,other]
  - default_model: string
  - auth: optional map (credentials per backend)
  - execution: { mode: enum[docker,local,none], timeout_seconds: int }

- Session

  - id: uuid
  - project_id: string
  - prompts: list of { prompt_text, model_used, timestamp }
  - workspace_path: string
  - created_at, last_updated

- Artifact
  - id: uuid
  - session_id
  - path: string
  - type: enum[file,patch,archive]
  - metadata: map

## Validation rules

- ProjectConfig.default_backend MUST be one of known adapters or `other` with
  adapter config supplied.
- Session prompts MUST include model metadata (backend + model name).

## Relationships

- ProjectConfig 1 - \* Session
- Session 1 - \* Artifact
```
