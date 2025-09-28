use cprcodr_core::session::Session;

// Offline contract test for session creation/persistence using Session struct.
#[test]
fn contract_session_creation_returns_session_id() {
    let s = Session::new(Some("test-project".to_string()));
    // save to a temp dir
    let td = tempfile::tempdir().expect("tempdir");
    s.save(td.path()).expect("save session");
    // load it back
    let mut p = td.path().to_path_buf();
    p.push(format!("{}.json", s.id));
    let loaded = Session::load(&p).expect("load session");
    assert_eq!(loaded.id, s.id, "loaded session id should match");
}
