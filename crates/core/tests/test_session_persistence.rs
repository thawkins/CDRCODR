use cprcodr_core::session::Session;
use tempfile::tempdir;

#[test]
fn session_save_and_load_roundtrip() {
    let dir = tempdir().unwrap();
    let s = Session::new(Some("project-x".to_string()));
    s.save(dir.path()).expect("save should succeed");
    let mut p = dir.path().to_path_buf();
    p.push(format!("{}.json", s.id));
    let s2 = Session::load(&p).expect("load should succeed");
    assert_eq!(s.project_id, s2.project_id);
}
