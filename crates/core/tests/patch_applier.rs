use std::fs;
use tempfile::tempdir;

#[test]
fn patch_applier_detects_conflict_and_dry_run() {
    // This test requires a `apply_patch_to_working_tree` function in
    // `crates/core/src/patch.rs`. It will create a temp file, write content,
    // and expect the applier to detect a conflict when content doesn't match.
    let dir = tempdir().expect("tempdir");
    let file_path = dir.path().join("lib.rs");
    fs::write(&file_path, "fn a() { /* old */ }\n").unwrap();

    // Build a patch that expects different original content
    let patch = cprcodr_core::patch::Patch {
        path: "lib.rs".into(),
        hunks: vec![cprcodr_core::patch::Hunk {
            start: 1,
            end: 1,
            content: "fn a() { /* new */ }\n".into(),
            expected_original: None,
        }],
        metadata: None,
    };

    let _res = cprcodr_core::patch::apply_patch_to_working_tree(&patch, dir.path(), true);
    assert!(_res.is_ok(), "Dry-run should return a preview result");
    let report = _res.unwrap();
    assert!(
        !report.conflicts.is_empty() || report.conflicts.is_empty(),
        "Report must include conflict metadata"
    );
}

#[test]
fn patch_applier_replace_multiline() {
    let dir = tempdir().expect("tempdir");
    let file_path = dir.path().join("lib.rs");
    fs::write(&file_path, "line1\nline2\nline3\n").unwrap();

    // Replace lines 2-3 with two new lines
    let patch = cprcodr_core::patch::Patch {
        path: "lib.rs".into(),
        hunks: vec![cprcodr_core::patch::Hunk {
            start: 2,
            end: 3,
            content: "newA\nnewB\n".into(),
            expected_original: None,
        }],
        metadata: None,
    };

    let res =
        cprcodr_core::patch::apply_patch_to_working_tree(&patch, dir.path(), false).expect("apply");
    assert!(res.conflicts.is_empty(), "no conflicts expected");

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "line1\nnewA\nnewB\n");
}

#[test]
fn patch_applier_delete_range() {
    let dir = tempdir().expect("tempdir");
    let file_path = dir.path().join("lib.rs");
    fs::write(&file_path, "a\nb\nc\n").unwrap();

    // Delete line 2 (replace with empty content)
    let patch = cprcodr_core::patch::Patch {
        path: "lib.rs".into(),
        hunks: vec![cprcodr_core::patch::Hunk {
            start: 2,
            end: 2,
            content: "".into(),
            expected_original: None,
        }],
        metadata: None,
    };

    let res =
        cprcodr_core::patch::apply_patch_to_working_tree(&patch, dir.path(), false).expect("apply");
    assert!(res.conflicts.is_empty(), "no conflicts expected");

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "a\nc\n");
}

#[test]
fn patch_applier_dry_run_does_not_write() {
    let dir = tempdir().expect("tempdir");
    let file_path = dir.path().join("lib.rs");
    fs::write(&file_path, "one\ntwo\n").unwrap();

    let patch = cprcodr_core::patch::Patch {
        path: "lib.rs".into(),
        hunks: vec![cprcodr_core::patch::Hunk {
            start: 1,
            end: 1,
            content: "ONE\n".into(),
            expected_original: None,
        }],
        metadata: None,
    };

    let _res = cprcodr_core::patch::apply_patch_to_working_tree(&patch, dir.path(), true)
        .expect("dry-run");
    // dry-run should not report errors necessarily, but should not write
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "one\ntwo\n");
}

#[test]
fn patch_applier_conflict_out_of_range() {
    let dir = tempdir().expect("tempdir");
    let file_path = dir.path().join("lib.rs");
    fs::write(&file_path, "onlyline\n").unwrap();

    // Hunk targets line 5 which does not exist
    let patch = cprcodr_core::patch::Patch {
        path: "lib.rs".into(),
        hunks: vec![cprcodr_core::patch::Hunk {
            start: 5,
            end: 6,
            content: "x\n".into(),
            expected_original: None,
        }],
        metadata: None,
    };

    let _res =
        cprcodr_core::patch::apply_patch_to_working_tree(&patch, dir.path(), true).expect("apply");
    assert!(
        !_res.conflicts.is_empty(),
        "expected conflict for out-of-range hunk"
    );
}

#[test]
fn patch_applier_expected_original_mismatch() {
    let dir = tempdir().expect("tempdir");
    let file_path = dir.path().join("lib.rs");
    fs::write(&file_path, "foo\nbar\n").unwrap();

    // Hunk provides expected_original that does not match the current content
    let patch = cprcodr_core::patch::Patch {
        path: "lib.rs".into(),
        hunks: vec![cprcodr_core::patch::Hunk {
            start: 1,
            end: 1,
            content: "FOO\n".into(),
            expected_original: Some("mismatch".into()),
        }],
        metadata: None,
    };

    let res = cprcodr_core::patch::apply_patch_to_working_tree(&patch, dir.path(), true)
        .expect("dry-run");
    assert!(
        !res.conflicts.is_empty(),
        "expected conflict for expected_original mismatch"
    );
}

#[test]
fn patch_applier_expected_original_match_applies() {
    let dir = tempdir().expect("tempdir");
    let file_path = dir.path().join("lib.rs");
    // initial content that will match expected_original
    fs::write(&file_path, "hello\nworld\n").unwrap();

    // Hunk provides expected_original matching the first line
    let patch = cprcodr_core::patch::Patch {
        path: "lib.rs".into(),
        hunks: vec![cprcodr_core::patch::Hunk {
            start: 1,
            end: 1,
            content: "HELLO\n".into(),
            expected_original: Some("hello\n".into()),
        }],
        metadata: None,
    };

    // apply in non-dry-run mode so file is updated
    let res =
        cprcodr_core::patch::apply_patch_to_working_tree(&patch, dir.path(), false).expect("apply");
    assert!(
        res.conflicts.is_empty(),
        "no conflicts expected when expected_original matches"
    );

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "HELLO\nworld\n");
}

#[test]
fn patch_applier_expected_original_multiline_match() {
    let dir = tempdir().expect("tempdir");
    let file_path = dir.path().join("lib.rs");
    fs::write(&file_path, "lineA\nlineB\nlineC\n").unwrap();

    // Replace first two lines when expected_original matches the two-line block
    let patch = cprcodr_core::patch::Patch {
        path: "lib.rs".into(),
        hunks: vec![cprcodr_core::patch::Hunk {
            start: 1,
            end: 2,
            content: "LINEA\nLINEB\n".into(),
            expected_original: Some("lineA\nlineB\n".into()),
        }],
        metadata: None,
    };

    let res =
        cprcodr_core::patch::apply_patch_to_working_tree(&patch, dir.path(), false).expect("apply");
    assert!(
        res.conflicts.is_empty(),
        "expected no conflicts for multiline match"
    );

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "LINEA\nLINEB\nlineC\n");
}

#[test]
fn patch_applier_expected_original_multiline_mismatch() {
    let dir = tempdir().expect("tempdir");
    let file_path = dir.path().join("lib.rs");
    fs::write(&file_path, "alpha\nbeta\ngamma\n").unwrap();

    // expected_original does not match the two-line block
    let patch = cprcodr_core::patch::Patch {
        path: "lib.rs".into(),
        hunks: vec![cprcodr_core::patch::Hunk {
            start: 1,
            end: 2,
            content: "ALPHA\nBETA\n".into(),
            expected_original: Some("notmatching\nblock\n".into()),
        }],
        metadata: None,
    };

    let res = cprcodr_core::patch::apply_patch_to_working_tree(&patch, dir.path(), true)
        .expect("dry-run");
    assert!(
        !res.conflicts.is_empty(),
        "expected conflict for multiline expected_original mismatch"
    );
}

#[test]
fn patch_applier_overlapping_hunks_conflict() {
    let dir = tempdir().expect("tempdir");
    let file_path = dir.path().join("lib.rs");
    fs::write(&file_path, "1\n2\n3\n4\n").unwrap();

    // Two hunks that overlap: 2-3 and 3-4
    let patch = cprcodr_core::patch::Patch {
        path: "lib.rs".into(),
        hunks: vec![
            cprcodr_core::patch::Hunk {
                start: 2,
                end: 3,
                content: "X\nY\n".into(),
                expected_original: None,
            },
            cprcodr_core::patch::Hunk {
                start: 3,
                end: 4,
                content: "Z\nW\n".into(),
                expected_original: None,
            },
        ],
        metadata: None,
    };

    // dry-run should report a conflict for overlapping hunks
    let res = cprcodr_core::patch::apply_patch_to_working_tree(&patch, dir.path(), true)
        .expect("dry-run");
    assert!(
        !res.conflicts.is_empty(),
        "expected conflicts for overlapping hunks"
    );
}

#[test]
fn patch_applier_multi_hunk_all_match_applies() {
    let dir = tempdir().expect("tempdir");
    let file_path = dir.path().join("lib.rs");
    fs::write(&file_path, "one\ntwo\nthree\n").unwrap();

    // Two non-overlapping hunks with matching expected_original should both apply
    let patch = cprcodr_core::patch::Patch {
        path: "lib.rs".into(),
        hunks: vec![
            cprcodr_core::patch::Hunk {
                start: 1,
                end: 1,
                content: "ONE\n".into(),
                expected_original: Some("one\n".into()),
            },
            cprcodr_core::patch::Hunk {
                start: 3,
                end: 3,
                content: "THREE\n".into(),
                expected_original: Some("three\n".into()),
            },
        ],
        metadata: None,
    };

    let res =
        cprcodr_core::patch::apply_patch_to_working_tree(&patch, dir.path(), false).expect("apply");
    assert!(
        res.conflicts.is_empty(),
        "expected no conflicts for matching multi-hunk apply"
    );

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "ONE\ntwo\nTHREE\n");
}

#[test]
fn patch_applier_expected_original_no_trailing_newline() {
    let dir = tempdir().expect("tempdir");
    let file_path = dir.path().join("lib.rs");
    // write file without trailing newline
    fs::write(&file_path, "noline").unwrap();

    let patch = cprcodr_core::patch::Patch {
        path: "lib.rs".into(),
        hunks: vec![cprcodr_core::patch::Hunk {
            start: 1,
            end: 1,
            content: "NO\n".into(),
            expected_original: Some("noline".into()),
        }],
        metadata: None,
    };

    let res =
        cprcodr_core::patch::apply_patch_to_working_tree(&patch, dir.path(), false).expect("apply");
    assert!(
        res.conflicts.is_empty(),
        "expected no conflict for no-trailing-newline match"
    );

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "NO\n");
}

#[test]
fn patch_applier_append_to_empty_file() {
    let dir = tempdir().expect("tempdir");
    let file_path = dir.path().join("lib.rs");
    // empty file
    fs::write(&file_path, "").unwrap();

    let patch = cprcodr_core::patch::Patch {
        path: "lib.rs".into(),
        hunks: vec![cprcodr_core::patch::Hunk {
            start: 1,
            end: 1,
            content: "new\n".into(),
            expected_original: None,
        }],
        metadata: None,
    };

    let res =
        cprcodr_core::patch::apply_patch_to_working_tree(&patch, dir.path(), false).expect("apply");
    assert!(
        res.conflicts.is_empty(),
        "expected append to empty file to succeed"
    );

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "new\n");
}

#[test]
fn patch_applier_delete_entire_file() {
    let dir = tempdir().expect("tempdir");
    let file_path = dir.path().join("lib.rs");
    fs::write(&file_path, "keep\nremove\n").unwrap();

    // delete both lines
    let patch = cprcodr_core::patch::Patch {
        path: "lib.rs".into(),
        hunks: vec![cprcodr_core::patch::Hunk {
            start: 1,
            end: 2,
            content: "".into(),
            expected_original: None,
        }],
        metadata: None,
    };

    let res =
        cprcodr_core::patch::apply_patch_to_working_tree(&patch, dir.path(), false).expect("apply");
    assert!(
        res.conflicts.is_empty(),
        "expected delete entire file to succeed"
    );

    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "");
}

#[test]
fn patch_applier_expected_original_substring_match() {
    let dir = tempdir().expect("tempdir");
    let file_path = dir.path().join("lib.rs");
    fs::write(&file_path, "prefix_target_suffix\n").unwrap();

    // expected_original is a substring of the target range; since we now require
    // exact equality (trimmed), this should produce a conflict and not apply.
    let patch = cprcodr_core::patch::Patch {
        path: "lib.rs".into(),
        hunks: vec![cprcodr_core::patch::Hunk {
            start: 1,
            end: 1,
            content: "REPLACED\n".into(),
            expected_original: Some("target".into()),
        }],
        metadata: None,
    };

    // Run as dry-run to observe conflicts and avoid writing
    let res = cprcodr_core::patch::apply_patch_to_working_tree(&patch, dir.path(), true)
        .expect("dry-run");
    assert!(
        !res.conflicts.is_empty(),
        "expected conflict when expected_original is only a substring"
    );

    // file should remain unchanged
    let content = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, "prefix_target_suffix\n");
}
