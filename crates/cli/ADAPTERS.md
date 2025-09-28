# Adapters: CLI usage and examples

This file documents how to call adapters from the `cprcodr` CLI and how to configure the common adapters (mock, ollama, lmstudio).

Examples

- Use the default synchronous backend (sync Backend API). This returns structured artifacts when the adapter implements `Backend`:

  cprcodr gen "Describe a small Rust function" --backend ollama --url http://localhost:3000 --model gpt-4-mini

- Call the async LLM adapter surface directly. This maps the adapter text result into a single artifact `generated/<adapter>.txt`:

  cprcodr gen "Write a README" --adapter ollama --adapter-url http://localhost:3000 --adapter-api-key MYKEY

- Use the mock LLM adapter (no network needed):

  cprcodr gen "Test prompt" --adapter mock

Mock adapter seeding

- To provide deterministic responses for the Backend-style mock adapter, set `MOCK_ADAPTER_RESPONSE` to either a JSON string or the path to a JSON file. The mock supports many testing behaviors (arrays of artifacts, intermittent failures, timeouts) — see `crates/core/src/adapters/mock.rs` for details.

Environment and persistence

- `CPRCODR_OUTPUT` controls the directory where sessions and the mock state file are written. If not set, the CLI uses `.cprcodr/session` in the current working directory.

Notes

- Currently `--backend` and `--adapter` are complementary modes. `--backend` uses the sync `Backend` adapters and is the default. `--adapter` uses the async `LLMAdapter` trait and maps the single-text output into a single artifact. We plan to extend adapters to return structured artifacts directly in the async path.
