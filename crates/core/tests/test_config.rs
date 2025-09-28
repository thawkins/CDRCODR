use cprcodr_core::config::ProjectConfig;

#[test]
fn parse_project_config_examples() {
    let sample = r#"
id = "testproject"
default_backend = "ollama"
default_model = "gpt-4o-mini"
"#;

    let cfg = ProjectConfig::from_str(sample).expect("parse should succeed");
    assert_eq!(cfg.id.unwrap(), "testproject");
    assert_eq!(cfg.default_backend.unwrap(), "ollama");
    assert_eq!(cfg.default_model.unwrap(), "gpt-4o-mini");
}
