# Quickstart: cprcodr coding CLI

This quickstart shows the common developer flows for the coding CLI features.

1. Install & initialize

```bash
# build locally (cargo install in workspace)
cargo build -p cprcodr

# create a project skeleton
./target/debug/cprcodr init
```

2. Generate code

```bash
# Generate artifacts with a prompt using the mock backend
CPRCODR_OUTPUT=$(pwd)/.cprcodr/session \
MOCK_ADAPTER_RESPONSE='{"artifacts":[{"path":"src/lib.rs","summary":"lib","content":"// stub"}]}' \
./target/debug/cprcodr gen "create a library" --backend mock
```

3. Preview & apply

```bash
# Preview generated artifacts for session_id
./target/debug/cprcodr preview <session_id>

# Apply artifacts (writes files)
./target/debug/cprcodr apply <session_id>
```

4. Dry-run and safe edits

```bash
# Generate without persisting session or writing files
./target/debug/cprcodr gen "sketch feature" --backend mock --dry-run

# Reset mock state
./target/debug/cprcodr mock-reset
```
