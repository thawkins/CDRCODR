# cprcodr

cprcodr is a small CLI for generating code artifacts using configurable LLM backends.

Testing & mock adapters

- Use the `mock` backend for deterministic offline tests.

Integration tests

- The adapters integration test (an in-process `hyper` server) lives in `crates/core/INTEGRATION_TESTS.md` and is feature-gated so it doesn't run by default.
- To run the integration tests for `crates/core` locally:

```bash
# run only core tests and enable the integration-tests feature
cargo test --manifest-path crates/core/Cargo.toml --tests --features integration-tests
```

- Seed the mock adapter by setting the `MOCK_ADAPTER_RESPONSE` env var. The value
  may be either:
  - a path to a JSON file containing an array of artifacts or an object with an `artifacts` key
  - a JSON string (for convenience in CI)
- If you seed the mock adapter with an object containing `{"error": "msg"}` the adapter
  will return a protocol error (useful to exercise error handling paths).

Advanced mock behaviors

- You can provide a top-level array to `MOCK_ADAPTER_RESPONSE` to have the mock return
  sequential seeds across multiple `generate()` calls (useful for stateful flows in tests).
- Each seed object may include `timeout_ms` to simulate a backend delay (in milliseconds).
- If a seed object includes `partial: true` the adapter will strip the `content` field
  from returned artifacts to simulate partial/streaming responses.

Output directories

- Commands that persist sessions/artifacts look for an output directory in the following order:

  - per-command `--output <dir>` flag (for `gen`)
  - global `--output <dir>` flag
  - `CPRCODR_OUTPUT` environment variable
  - fallback to `.cprcodr/session`

  Dry-run

  - Use `cprcodr gen --dry-run` to run generation and preview returned artifacts without
    persisting session metadata or writing artifact stubs to disk. This is useful for quick
    previews in CI or during development.

CI

- A GitHub Actions workflow is included at `.github/workflows/ci.yml` which runs fmt, clippy
  and the test suite.
