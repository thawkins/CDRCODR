# Contributing

This project follows a TDD-first workflow. Quick checklist to contribute:

- Write a failing test that reproduces the desired behavior (unit/integration/e2e).
- Implement the minimal code changes to make the test pass.
- Run `cargo test --workspace` and ensure all tests pass.
- Open a PR and describe the change and any design tradeoffs.

Local tips:

- Use `MOCK_ADAPTER_RESPONSE` to seed deterministic responses for tests.
- Prefer the `mock` backend for offline tests.
- Keep adapters synchronous for now; migrating to async is a more invasive refactor.
