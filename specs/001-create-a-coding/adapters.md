# Adapters (Ollama, LMStudio, Mock)

This document describes the adapter story for the cprcodr project: the small, async `LLMAdapter` trait in `crates/core/src/adapters/trait_adapter.rs`, the available adapter implementations (mock, ollama, lmstudio), and how to configure and use them from the CLI.

Quick summary

- `LLMAdapter` is an async trait used by higher-level code paths to get a single text response back from an LLM-like service. It is intentionally minimal: `LLMRequest { prompt, max_tokens } -> LLMResponse { text }`.
- There are two kinds of adapters in the codebase:
  - Backend-style adapters (implement `Backend`) which return structured artifacts synchronously (used by the existing `gen` path).
  - LLMAdapter-style adapters (implement `LLMAdapter`) which are async and return plain text; these are used when `--adapter` is provided on the CLI and are mapped into a single artifact stub by the CLI.

Adapters available

- MockAdapter (Backend): deterministic, file-seeded mock used by tests; supports state persistence via `CPRCODR_OUTPUT/mock_state.json` and seeding with `MOCK_ADAPTER_RESPONSE`.
- LLMMockAdapter (LLMAdapter): lightweight prefixing mock used for quick experiments and CLI `--adapter mock` flows.
- OllamaAdapter: supports both Backend (sync) generation and LLMAdapter async `call`. Async `call` performs a POST to `{url}/generate` and extracts a `text` field or returns the full JSON string.
- LMStudioAdapter: similar to OllamaAdapter but targets LMStudio-style `/api/generate` endpoints and looks for `text` or stringifies the JSON.

CLI behavior

- The CLI supports two complementary ways to generate artifacts:
  1. `--backend` (legacy): chooses one of the sync `Backend` adapters (`mock`, `ollama`, `lmstudio`) and calls `generate(prompt, options)` returning structured artifacts. This is the default and used by the current test suite.
  2. `--adapter` (new): chooses an async `LLMAdapter` implementation and calls its `call()` method. The CLI maps the adapter's text response into a single artifact with path `generated/<adapter>.txt` and content equal to the adapter text. Use this for quick experimentation or when you want to call a raw LLM.

Configuration

- `--adapter-url`: override the adapter's base URL. Falls back to `--url` if not provided.
- `--adapter-api-key`: optional API key passed as an Authorization header (Ollama) or X-API-Key (LMStudio), depending on adapter.
- `CPRCODR_OUTPUT`: directory used by the mock adapter state persistence and for saving sessions/artifacts.
- `MOCK_ADAPTER_RESPONSE`: optional env var to seed the mock adapter deterministic responses (can be a JSON object or a path to a JSON file).

Next steps / improvements

- Map LLMAdapter outputs to multiple artifacts (current mapping is a single artifact per call).
- Make adapter discovery pluggable via a registry so new adapters can be loaded by name from config files.
- Add integration tests that spin up a local HTTP server and assert the adapters parse expected JSON shapes.
  Adapters for LLM backends

This document describes the adapter abstraction and the currently available adapters in the project.

LLMAdapter (core)

- Trait: `LLMAdapter` (async trait) defined at `crates/core/src/adapters/trait_adapter.rs`.
- Shapes: `LLMRequest { prompt: String, max_tokens: Option<usize> }` and `LLMResponse { text: String }`.
- Implementations may perform HTTP calls, local process calls, or return deterministic values for tests.

Available adapters (current)

- `LLMMockAdapter` (core)
  - Lightweight async mock that prefixes the prompt with a configured string and returns it.
  - Useful for tests and local development.
- `MockAdapter` (core)
  - Rich, synchronous Backend-based mock used by existing CLI flows and tests. Supports seeded responses via `MOCK_ADAPTER_RESPONSE` and persistent call state in `CPRCODR_OUTPUT/mock_state.json`.
- `OllamaAdapter` (core)
  - Async adapter that hits `POST {url}/generate` and returns the `text` field or the stringified JSON as the response.
  - Also contains a synchronous `Backend` implementation that maps API JSON to artifacts (used by `--backend ollama`).
- `LMStudioAdapter` (core)
  - Async adapter that hits `POST {url}/api/generate` and returns the `text` field or the JSON string.
  - Also contains a synchronous `Backend` implementation that maps API JSON to artifacts (used by `--backend lmstudio`).

How adapters are selected

- CLI `gen` command supports two related concepts:
  - `--backend` (sync backends): existing behavior for `mock`, `ollama`, `lmstudio` which call the `Backend` trait's `generate()` method and return structured artifacts.
  - `--adapter` (async LLM adapters): experimental path where the CLI will call the `LLMAdapter::call()` method and wrap the single textual response into a single artifact (path `generated/<adapter>.txt`).
- Adapter-related flags:
  - `--adapter` — adapter name (e.g. `mock`, `ollama`, `lmstudio`). If not provided, the CLI uses the `--backend` sync flow.
  - `--adapter-url` — overrides the default backend URL for the adapter.
  - `--adapter-api-key` — API key header for the adapter when required.

Notes and next steps

- Mapping LLM responses to multiple artifacts is an enhancement: currently `--adapter` maps the adapter's string output into a single artifact. A future task: parse structured JSON responses from the adapter into multiple artifact objects.
- Add integration tests with a mock HTTP server to validate positive parsing paths for Ollama and LMStudio.
- Expand `LLMRequest` and `LLMResponse` with richer fields (metadata, channels, tool calls) as needed.
