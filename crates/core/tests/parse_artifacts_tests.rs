use cprcodr_core::adapters::trait_adapter::parse_artifacts_from_text;

#[test]
fn parses_json_array_with_content() {
    let txt = r#"[
        {"path":"a.txt","content":"hello world"},
        {"path":"b.txt","content":"second file"}
    ]"#;
    let res = parse_artifacts_from_text(txt);
    assert_eq!(res.len(), 2);
    assert_eq!(res[0].path, "a.txt");
    assert_eq!(res[0].content.as_deref().unwrap(), "hello world");
}

#[test]
fn parses_json_object_with_content() {
    let txt = r#"{"path":"single.txt","content":"single content"}"#;
    let res = parse_artifacts_from_text(txt);
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].path, "single.txt");
    assert_eq!(res[0].content.as_deref().unwrap(), "single content");
}

#[test]
fn parses_plain_text_to_single_artifact() {
    let txt = "This is a plain text artifact output\nwith multiple lines";
    let res = parse_artifacts_from_text(txt);
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].path, "generated/artifact-1.txt");
    assert!(res[0]
        .content
        .as_deref()
        .unwrap()
        .starts_with("This is a plain text artifact"));
}
