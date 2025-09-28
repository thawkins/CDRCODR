## Unreleased

- Add `content: Option<String>` to `ArtifactMetadata` to preserve full artifact
  contents returned by adapters when available. Update parser, adapters, and
  CLI mapping to preferentially use `content` (falling back to `summary`).
  Add parser unit tests.

- Guidance: adapters should populate `content` with the full artifact body
  (when available) instead of only a short `summary` to enable deterministic
  patch generation and preview. The CLI will prefer `content` and fall back to
  `summary` when `content` is None.

# Changelog

## Unreleased

- applier: `expected_original` now requires exact equality (after trimming trailing newlines) of the target range; partial/substring matches are considered conflicts. This avoids accidental partial matches and makes patch application deterministic.
- applier: overlapping hunks within the same patch are detected and recorded as conflicts for the later hunk(s); callers should avoid overlapping hunks. Overlap detection is conservative and intended for small patches produced by the CLI.
