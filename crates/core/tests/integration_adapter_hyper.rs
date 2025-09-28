#![cfg(feature = "integration-tests")]

use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use serde_json::json;

use cprcodr_core::adapters::trait_adapter::LLMRequest;
use cprcodr_core::adapters::{LLMAdapter, LMStudioAdapter, OllamaAdapter};
use cprcodr_core::Backend;

async fn run_hyper_server(addr: SocketAddr) {
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|req: Request<Body>| async move {
            // handle POST /generate and POST /api/generate
            let path = req.uri().path().to_string();
            if req.method() == Method::POST && path == "/generate" {
                let body = json!({
                    "text": "ollama text response",
                    "artifacts": [{"path": "a.txt", "summary": "s", "content": "c"}]
                });
                let resp = Response::new(Body::from(body.to_string()));
                return Ok::<_, Infallible>(resp);
            }
            if req.method() == Method::POST && path == "/api/generate" {
                let body = json!({
                    "text": "lmstudio text response",
                    "results": [{"path": "b.txt", "summary": "s2", "content": "c2"}]
                });
                let resp = Response::new(Body::from(body.to_string()));
                return Ok::<_, Infallible>(resp);
            }
            Ok::<_, Infallible>(Response::new(Body::from("not found")))
        }))
    });

    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

#[tokio::test]
async fn test_adapters_with_hyper_server() {
    // Bind to an available port
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
    let addr = listener.local_addr().unwrap();
    drop(listener);

    // Start the hyper server on the existing tokio runtime using tokio::spawn
    tokio::spawn(async move {
        run_hyper_server(addr).await;
    });

    // Give server a moment to start
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    // Test Ollama async call
    let ollama = OllamaAdapter::new(format!("http://{}", addr), None);
    let req = LLMRequest {
        prompt: "hi".into(),
        max_tokens: None,
    };
    let resp = ollama.call(req).await.expect("ollama call ok");
    // expect at least one artifact whose summary or path contains response info
    assert!(!resp.is_empty());

    // Test Ollama sync generate (should parse artifacts) via spawn_blocking
    let options = serde_json::json!({});
    let ollama_sync = OllamaAdapter::new(format!("http://{}", addr), None);
    let options_clone = options.clone();
    let arts = tokio::task::spawn_blocking(move || ollama_sync.generate("p", options_clone))
        .await
        .expect("spawn_blocking join")
        .expect("ollama generate ok");
    assert!(!arts.artifacts.is_empty());

    // Test LMStudio async call
    let lm = LMStudioAdapter::new(format!("http://{}", addr), None);
    let req2 = LLMRequest {
        prompt: "hi".into(),
        max_tokens: None,
    };
    let resp2 = lm.call(req2).await.expect("lmstudio call ok");
    assert!(!resp2.is_empty());

    // Test LMStudio sync generate via spawn_blocking
    let lm_sync = LMStudioAdapter::new(format!("http://{}", addr), None);
    let arts2 = tokio::task::spawn_blocking(move || lm_sync.generate("p", serde_json::json!({})))
        .await
        .expect("spawn_blocking join")
        .expect("lm generate ok");
    assert!(!arts2.artifacts.is_empty());
}
