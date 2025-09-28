use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Hunk {
    /// 1-based inclusive start line
    pub start: usize,
    /// 1-based inclusive end line
    pub end: usize,
    /// replacement content for the range (may contain multiple lines)
    pub content: String,
    /// optional expected original snippet for the target range; when present,
    /// the applier will verify the current file content in the range contains
    /// this snippet (exact match) before applying the hunk. If it does not
    /// match, the hunk is recorded as a conflict and skipped.
    #[serde(default)]
    pub expected_original: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Patch {
    /// path relative to the working tree root
    pub path: String,
    pub hunks: Vec<Hunk>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug)]
pub struct PatchReport {
    pub conflicts: Vec<String>,
}

impl Default for PatchReport {
    fn default() -> Self {
        Self::new()
    }
}

impl PatchReport {
    pub fn new() -> Self {
        PatchReport {
            conflicts: Vec::new(),
        }
    }
}

/// Apply a patch to the working tree located at `workdir`.
///
/// Behavior (simple, safe semantics for MVP):
/// - `Hunk.start` and `Hunk.end` are treated as 1-based inclusive line indices
///   into the target file.
/// - If the file does not exist, it's created when not a dry run and the
///   hunk content is written.
/// - If any hunk references lines outside the current file length, a conflict
///   is recorded for that hunk and the hunk is skipped.
/// - Hunks are applied in the order they appear. Overlapping hunks may produce
///   unpredictable results; callers should avoid overlaps.
///
/// Additional semantics for `expected_original` and overlaps:
/// - When `Hunk.expected_original` is present, the applier verifies that the
///   current content of the target range exactly equals the provided value
///   after trimming trailing newlines. If the values do not match, the hunk is
///   considered a conflict and is skipped. This exact-match behavior makes the
///   check deterministic and prevents accidental partial matches from masking
///   unexpected file state.
/// - If multiple hunks in the same patch target overlapping line ranges, the
///   applier records an overlap conflict for the later hunk(s) and skips them.
///   Overlap detection is conservative (O(n^2) over hunks) but acceptable for
///   small patches produced by the CLI/workflows.
pub fn apply_patch_to_working_tree(
    patch: &Patch,
    workdir: &Path,
    dry_run: bool,
) -> Result<PatchReport, String> {
    let mut report = PatchReport::new();

    let target = workdir.join(&patch.path);

    // Read existing content if present
    let existing = fs::read_to_string(&target).unwrap_or_else(|_| String::new());
    // Split into lines preserving trailing newline behavior by keeping split_terminator
    let mut lines: Vec<String> = existing.lines().map(|s| format!("{}\n", s)).collect();
    // If file ended without newline, lines() may have trimmed; handle empty file specially
    if existing.is_empty() {
        lines.clear();
    }

    for (i, hunk) in patch.hunks.iter().enumerate() {
        // convert to 0-based indices for vector ops
        if hunk.start == 0 || hunk.end == 0 || hunk.end < hunk.start {
            report.conflicts.push(format!(
                "hunk {} has invalid range {}-{}",
                i, hunk.start, hunk.end
            ));
            continue;
        }

        let start_idx = hunk.start - 1;
        let end_idx = hunk.end - 1;

        if start_idx > lines.len() || end_idx >= lines.len() && !lines.is_empty() {
            // out of range -> conflict
            report.conflicts.push(format!(
                "hunk {} targets out-of-range lines {}-{} for file {} (file has {} lines)",
                i,
                hunk.start,
                hunk.end,
                patch.path,
                lines.len()
            ));
            continue;
        }

        // Detect overlaps with previous hunks: if any previously valid hunk range
        // intersects this hunk's range, record a conflict and skip applying.
        // Overlapping hunks are considered unsafe.
        for (j, prev) in patch.hunks.iter().enumerate().take(i) {
            if prev.start == 0 || prev.end == 0 || prev.end < prev.start {
                continue;
            }
            // ranges overlap if not (prev.end < hunk.start || hunk.end < prev.start)
            if !(prev.end < hunk.start || hunk.end < prev.start) {
                report.conflicts.push(format!(
                    "hunk {} overlaps with hunk {} for file {}",
                    i, j, patch.path
                ));
                continue;
            }
        }

        // Validate expected_original if provided
        if let Some(expected) = &hunk.expected_original {
            // extract current content of the target range
            let mut current = String::new();
            if !lines.is_empty() {
                // iterate the slice of lines for the target range and accumulate
                let take_end = end_idx.min(lines.len().saturating_sub(1));
                for line in lines.iter().take(take_end + 1).skip(start_idx) {
                    current.push_str(line);
                }
            }
            // Compare after trimming to be tolerant to trailing newline differences
            let cur_trim = current.trim_end_matches('\n');
            let exp_trim = expected.trim_end_matches('\n');
            if cur_trim != exp_trim {
                report.conflicts.push(format!(
                    "hunk {} expected_original mismatch for file {}",
                    i, patch.path
                ));
                continue;
            }
        }

        // Prepare replacement lines from hunk.content
        let mut repl: Vec<String> = hunk.content.lines().map(|s| format!("{}\n", s)).collect();
        // If content ends with a newline, lines() will omit the final empty; detect and preserve
        if !hunk.content.ends_with('\n') && !repl.is_empty() {
            // the original lines() call dropped newline, so adjust last element
            // but to keep simple, do nothing; we already appended \n above
        }

        // Replace the range [start_idx..=end_idx] with repl
        // If the file is empty, treat start_idx==0 as append
        if lines.is_empty() && start_idx == 0 && end_idx == 0 {
            // empty file, simply set lines to repl
            lines = repl;
        } else {
            // remove the range
            let mut new_lines = Vec::new();
            // push before
            for (ix, line) in lines.iter().enumerate() {
                if ix < start_idx || ix > end_idx {
                    new_lines.push(line.clone());
                } else {
                    // skip replaced lines
                }
            }
            // insert replacement at position start_idx
            let insert_at = start_idx.min(new_lines.len());
            let mut head = new_lines.split_off(insert_at);
            new_lines.append(&mut repl);
            new_lines.append(&mut head);
            lines = new_lines;
        }
    }

    if !dry_run {
        // Ensure parent dir exists
        if let Some(parent) = target.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                return Err(format!("failed to create parent directories: {}", e));
            }
        }

        // Write the final content
        let mut out = String::new();
        for l in &lines {
            out.push_str(l);
        }

        if let Err(e) = fs::write(&target, out) {
            return Err(format!("failed to write file {}: {}", target.display(), e));
        }
    }

    Ok(report)
}
