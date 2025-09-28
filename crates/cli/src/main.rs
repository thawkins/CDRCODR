use clap::{Parser, Subcommand};
use serde_json::json;

use cprcodr_core::adapters::MockAdapter;
use cprcodr_core::adapters::{LLMAdapter, LMStudioAdapter, OllamaAdapter};
use cprcodr_core::session::Session;
use cprcodr_core::Backend;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Global output directory for sessions/artifacts (overrides CPRCODR_OUTPUT)
    #[arg(long)]
    output: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    /// Generate artifacts from a prompt
    Gen {
        /// Prompt text
        prompt: String,
        /// Backend to use (ollama|lmstudio)
        #[arg(long, default_value = "ollama")]
        backend: String,
        /// Optional LLM adapter to use instead of the synchronous backend (mock|ollama|lmstudio)
        #[arg(long)]
        adapter: Option<String>,
        /// Backend URL
        #[arg(long, default_value = "http://127.0.0.1:3000")]
        url: String,
        /// Adapter URL (for LLM adapters)
        #[arg(long)]
        adapter_url: Option<String>,
        /// Adapter API key (for LLM adapters)
        #[arg(long)]
        adapter_api_key: Option<String>,
        /// Model to use on the backend
        #[arg(long)]
        model: Option<String>,
        /// Execution timeout in seconds
        #[arg(long)]
        timeout: Option<u64>,
        /// Output path for artifacts (optional)
        #[arg(long)]
        output: Option<String>,
        /// If set, do not persist session or write artifact stubs (preview only)
        #[arg(long)]
        dry_run: bool,
    },
    /// Reset mock adapter state (remove mock_state.json in the output dir)
    MockReset,
    /// Preview artifacts saved in a session
    Preview {
        /// Session UUID
        session_id: String,
        /// If set, also run applier in dry-run mode (default true for preview)
        #[arg(long)]
        dry_run: bool,
    },
    /// Apply artifacts from a session into the workspace (creates stub files)
    Apply {
        /// Session UUID
        session_id: String,
        /// Commit changes to git on success (creates a branch and commit)
        #[arg(long)]
        git_commit: bool,
    },
    /// Manage saved sessions
    Session {
        /// Subcommand for sessions: list, show, rm
        #[command(subcommand)]
        action: SessionAction,
    },
}

#[derive(Subcommand)]
enum SessionAction {
    /// List saved sessions
    List,
    /// Show session metadata
    Show { session_id: String },
    /// Remove session and artifacts
    Rm { session_id: String },
}

fn main() {
    let cli = Cli::parse();
    // local helper to resolve output dir: prefer per-command -> global flag -> env var -> default
    let resolve_output = |cmd_output: Option<String>| -> PathBuf {
        if let Some(o) = cmd_output {
            if !o.is_empty() {
                return PathBuf::from(o);
            }
        }
        if let Some(o) = cli.output.clone() {
            if !o.is_empty() {
                return PathBuf::from(o);
            }
        }
        if let Ok(env_out) = std::env::var("CPRCODR_OUTPUT") {
            if !env_out.is_empty() {
                return PathBuf::from(env_out);
            }
        }
        PathBuf::from(".cprcodr/session")
    };
    match &cli.command {
        Some(Commands::Init) => {
            println!("cprcodr init: not implemented yet");
            std::process::exit(1);
        }
        Some(Commands::Gen {
            prompt,
            backend,
            adapter,
            url,
            adapter_url,
            adapter_api_key,
            model,
            timeout,
            output,
            dry_run,
        }) => {
            let options = json!({
                "model": model.clone().unwrap_or_default(),
                "timeout": timeout.unwrap_or(0),
                "output": output.clone().unwrap_or_default(),
                "dry_run": *dry_run,
            });
            // If --adapter is provided, use the async LLMAdapter path which returns a single text response
            let result = if let Some(adapter_name) = &adapter {
                // build adapter from name
                match adapter_name.as_str() {
                    "mock" => {
                        // use the LLMMockAdapter wrapper
                        let a = cprcodr_core::adapters::LLMMockAdapter::new("mock:");
                        // run tokio runtime to call async adapter and map to artifacts
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        let resp = rt.block_on(async {
                            a.call(cprcodr_core::adapters::trait_adapter::LLMRequest {
                                prompt: prompt.clone(),
                                max_tokens: None,
                            })
                            .await
                        });
                        match resp {
                            Ok(arts) => {
                                let vals: Vec<serde_json::Value> = arts.into_iter().map(|m| serde_json::json!({"path": m.path, "summary": m.summary, "content": m.content.unwrap_or(m.summary)})).collect();
                                Ok(cprcodr_core::backend::Artifacts { artifacts: vals })
                            }
                            Err(e) => Err(cprcodr_core::backend::BackendError::Protocol(e)),
                        }
                    }
                    "ollama" => {
                        let url = adapter_url.clone().unwrap_or_else(|| url.clone());
                        let a = cprcodr_core::adapters::OllamaAdapter::new(
                            url,
                            adapter_api_key.clone(),
                        );
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        let resp = rt.block_on(async {
                            a.call(cprcodr_core::adapters::trait_adapter::LLMRequest {
                                prompt: prompt.clone(),
                                max_tokens: None,
                            })
                            .await
                        });
                        match resp {
                            Ok(arts) => {
                                let vals: Vec<serde_json::Value> = arts.into_iter().map(|m| serde_json::json!({"path": m.path, "summary": m.summary, "content": m.content.unwrap_or(m.summary)})).collect();
                                Ok(cprcodr_core::backend::Artifacts { artifacts: vals })
                            }
                            Err(e) => Err(cprcodr_core::backend::BackendError::Protocol(e)),
                        }
                    }
                    "lmstudio" => {
                        let url = adapter_url.clone().unwrap_or_else(|| url.clone());
                        let a = cprcodr_core::adapters::LMStudioAdapter::new(
                            url,
                            adapter_api_key.clone(),
                        );
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        let resp = rt.block_on(async {
                            a.call(cprcodr_core::adapters::trait_adapter::LLMRequest {
                                prompt: prompt.clone(),
                                max_tokens: None,
                            })
                            .await
                        });
                        match resp {
                            Ok(arts) => {
                                let vals: Vec<serde_json::Value> = arts.into_iter().map(|m| serde_json::json!({"path": m.path, "summary": m.summary, "content": m.content.unwrap_or(m.summary)})).collect();
                                Ok(cprcodr_core::backend::Artifacts { artifacts: vals })
                            }
                            Err(e) => Err(cprcodr_core::backend::BackendError::Protocol(e)),
                        }
                    }
                    other => {
                        eprintln!("unknown adapter: {}", other);
                        std::process::exit(2);
                    }
                }
            } else {
                match backend.as_str() {
                    "ollama" => {
                        let a = OllamaAdapter::new(url.clone(), None);
                        a.generate(prompt, options)
                    }
                    "mock" => {
                        let a = MockAdapter::new();
                        a.generate(prompt, options)
                    }
                    "lmstudio" => {
                        let a = LMStudioAdapter::new(url.clone(), None);
                        a.generate(prompt, options)
                    }
                    other => {
                        eprintln!("unknown backend: {}", other);
                        std::process::exit(2);
                    }
                }
            };
            match result {
                Ok(art) => {
                    println!("{}", serde_json::to_string_pretty(&art.artifacts).unwrap());
                    // Resolve output directory and persist session metadata and artifacts (unless dry_run)
                    let dir = resolve_output(output.clone());

                    if !*dry_run && !dir.to_string_lossy().is_empty() {
                        // create session
                        let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
                        let project_id = cwd.to_string_lossy().to_string();
                        let session = Session::new(Some(project_id));
                        // save session metadata
                        fs::create_dir_all(&dir).expect("failed to create output dir");
                        session.save(&dir).expect("failed to save session");
                        // save artifacts list
                        let mut artifacts_path = dir.clone();
                        artifacts_path.push(format!("{}-artifacts.json", session.id));
                        fs::write(
                            &artifacts_path,
                            serde_json::to_string_pretty(&art.artifacts).unwrap(),
                        )
                        .expect("failed to save artifacts");
                        println!(
                            "Saved session {} and artifacts to {}/",
                            session.id,
                            dir.display()
                        );
                    } else if *dry_run {
                        println!("Dry-run: not persisting session or artifacts");
                    }
                }
                Err(e) => {
                    eprintln!("generate failed: {}", e);
                    std::process::exit(3);
                }
            }
        }
        Some(Commands::Preview {
            session_id,
            dry_run: _dry_run,
        }) => {
            let dir = resolve_output(None);
            let mut p = dir.clone();
            p.push(format!("{}-artifacts.json", session_id));
            if !p.exists() {
                eprintln!(
                    "artifacts for session {} not found at {}",
                    session_id,
                    p.display()
                );
                std::process::exit(4);
            }
            let data = fs::read_to_string(&p).expect("read artifacts");
            let v: serde_json::Value = serde_json::from_str(&data).expect("parse artifacts");
            // If artifacts are a list of artifacts with path/summary/content, attempt to run applier in dry-run for each
            if let Some(arr) = v.as_array() {
                let mut reports = Vec::new();
                for item in arr {
                    let path = item.get("path").and_then(|s| s.as_str()).unwrap_or("");
                    let content = item.get("content").and_then(|s| s.as_str()).unwrap_or("");
                    // Build a simple Patch for single-file replacement of entire file
                    let patch = cprcodr_core::patch::Patch {
                        path: path.to_string(),
                        hunks: vec![cprcodr_core::patch::Hunk {
                            start: 1,
                            end: 1,
                            content: content.to_string(),
                            expected_original: None,
                        }],
                        metadata: None,
                    };
                    let report = cprcodr_core::patch::apply_patch_to_working_tree(
                        &patch,
                        &std::env::current_dir().unwrap(),
                        true,
                    )
                    .unwrap_or_else(|e| cprcodr_core::patch::PatchReport { conflicts: vec![e] });
                    reports.push(serde_json::json!({"path": path, "report": {"conflicts": report.conflicts}}));
                }
                println!("{}", serde_json::to_string_pretty(&reports).unwrap());
            } else {
                println!("Artifacts for session {}:\n{}", session_id, data);
            }
        }
        Some(Commands::Apply {
            session_id,
            git_commit,
        }) => {
            // Apply by building patches for each artifact and invoking the applier
            let dir = resolve_output(None);
            let mut p = dir.clone();
            p.push(format!("{}-artifacts.json", session_id));
            if !p.exists() {
                eprintln!("artifacts for session {} not found", session_id);
                std::process::exit(4);
            }
            let data = fs::read_to_string(&p).expect("read artifacts");
            let v: serde_json::Value = serde_json::from_str(&data).expect("parse artifacts");
            let cwd = std::env::current_dir().unwrap();
            if let Some(arr) = v.as_array() {
                let mut any_conflict = false;
                for item in arr {
                    let path = item.get("path").and_then(|s| s.as_str()).unwrap_or("");
                    let content = item.get("content").and_then(|s| s.as_str()).unwrap_or("");
                    let patch = cprcodr_core::patch::Patch {
                        path: path.to_string(),
                        hunks: vec![cprcodr_core::patch::Hunk {
                            start: 1,
                            end: 1,
                            content: content.to_string(),
                            expected_original: None,
                        }],
                        metadata: None,
                    };
                    let report =
                        cprcodr_core::patch::apply_patch_to_working_tree(&patch, &cwd, false)
                            .unwrap_or_else(|e| cprcodr_core::patch::PatchReport {
                                conflicts: vec![e],
                            });
                    if !report.conflicts.is_empty() {
                        any_conflict = true;
                        eprintln!("conflicts applying {}: {:?}", path, report.conflicts);
                    } else {
                        // Backwards-compatible message used by existing tests
                        println!("Wrote artifact stub: {}", path);
                    }
                }
                if any_conflict {
                    eprintln!("apply finished with conflicts");
                    std::process::exit(5);
                }

                if *git_commit {
                    // create branch and commit (simple behavior)
                    if let Err(e) = cprcodr_core::git::create_branch_and_commit(
                        &cwd,
                        "cprcodr-apply",
                        "cprcodr: apply artifacts",
                    ) {
                        eprintln!("git commit failed: {}", e);
                        std::process::exit(6);
                    } else {
                        println!("created branch and committed changes");
                    }
                }
            }
        }
        Some(Commands::Session { action }) => {
            // session directory
            let dir = resolve_output(None);

            match action {
                SessionAction::List => {
                    if !dir.exists() {
                        println!("no sessions found");
                        return;
                    }
                    let mut entries: Vec<_> = std::fs::read_dir(&dir)
                        .unwrap()
                        .filter_map(|e| e.ok())
                        .filter(|e| e.path().extension().map(|s| s == "json").unwrap_or(false))
                        .collect();
                    if entries.is_empty() {
                        println!("no sessions found");
                        return;
                    }
                    entries.sort_by_key(|e| e.path());
                    for e in entries {
                        println!("{}", e.path().display());
                    }
                }
                SessionAction::Show { session_id } => {
                    let mut p = dir.clone();
                    p.push(format!("{}.json", session_id));
                    if !p.exists() {
                        eprintln!("session {} not found", session_id);
                        std::process::exit(4);
                    }
                    let data = fs::read_to_string(&p).expect("read session");
                    println!("Session {}:\n{}", session_id, data);
                }
                SessionAction::Rm { session_id } => {
                    let mut p = dir.clone();
                    p.push(format!("{}.json", session_id));
                    if !p.exists() {
                        eprintln!("session {} not found", session_id);
                        std::process::exit(4);
                    }
                    // remove session file and artifacts file if present
                    std::fs::remove_file(&p).expect("remove session");
                    let mut art = dir.clone();
                    art.push(format!("{}-artifacts.json", session_id));
                    if art.exists() {
                        std::fs::remove_file(&art).ok();
                    }
                    println!("removed session {}", session_id);
                }
            }
        }
        Some(Commands::MockReset) => {
            let dir = resolve_output(None);
            let mut p = dir.clone();
            p.push("mock_state.json");
            if p.exists() {
                std::fs::remove_file(&p).expect("failed to remove mock state");
                println!("removed mock state: {}", p.display());
            } else {
                println!("no mock state found at {}", p.display());
            }
        }
        None => {
            println!("cprcodr: CLI placeholder. Use --help");
        }
    }
}
