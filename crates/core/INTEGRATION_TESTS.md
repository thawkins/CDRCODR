Integration tests for adapters

This file documents the lightweight integration-test pattern used for the LLM adapters in `crates/core`.

Why this exists

- Previous attempts to start an HTTP server with a separate runtime caused a tokio runtime drop panic when tests ran under cargo's async test harness. To avoid that, we use an in-process, runtime-friendly pattern.

Pattern used

- Start a minimal HTTP server (we use `hyper` in tests) and bind it to an ephemeral port.
- Spawn the server onto the current test runtime with `tokio::spawn(...)` so we don't create or drop a secondary tokio runtime inside an async context.
- For synchronous/blocking operations (the library's existing `Backend::generate` uses a blocking reqwest client), call them from the async test via `tokio::task::spawn_blocking(...)` so the blocking work doesn't block the async runtime.

Files

- `crates/core/tests/integration_adapter_hyper.rs` — example hyper-based integration test that exercises `OllamaAdapter` and `LMStudioAdapter`.

How to run locally

- Run the core tests only (faster during iteration):

```bash
cargo test --manifest-path crates/core/Cargo.toml --tests
```

- Run the hyper-based integration tests (feature-gated):

```bash
# run only core tests and enable the integration-tests feature
cargo test --manifest-path crates/core/Cargo.toml --tests --features integration-tests

# or run all tests across the workspace with integration tests enabled
cargo test --all --tests --features integration-tests
```

- Or run the full workspace tests (slower) without integration tests:

```bash
cargo test --all --tests
```

CI notes

- The integration test is feature-gated behind `integration-tests` which maps to the optional `hyper` dependency. This lets CI enable the test in a separate job without affecting the default test matrix.

- Recommended CI approach:

  - Keep the default test job fast and conservative (do not enable `integration-tests`).
  - Add a separate job in the CI matrix that enables integration tests by passing `--features integration-tests` when running cargo test for `crates/core`. That job can be scheduled less frequently or run on demand.

- Security / environment:
  - The hyper server binds to localhost only and does not perform external HTTP requests. CI runners typically run these tests safely, but you can restrict network access in the job environment if desired.

Troubleshooting

- If you see a panic like "Cannot drop a runtime in a context where blocking is not allowed", it's an indicator some code is creating or dropping a runtime inside an async context. Avoid creating `Runtime::new()` within async tests; instead use `tokio::spawn` or spawn a child process for a server.

Contact

- If you change how adapters work (move to async-only clients), consider updating the integration test to call async variants only and remove `spawn_blocking` uses.
