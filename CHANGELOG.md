# Changelog

## Unreleased

- applier: `expected_original` now requires exact equality (after trimming trailing newlines) of the target range; partial/substring matches are considered conflicts. This avoids accidental partial matches and makes patch application deterministic.
- applier: overlapping hunks within the same patch are detected and recorded as conflicts for the later hunk(s); callers should avoid overlapping hunks. Overlap detection is conservative and intended for small patches produced by the CLI.
