use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::get, Json, Router, Server};
use sysinfo::{CpuExt, System, SystemExt};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/", get(root_get))
        .route("/api/cpus", get(cpus_get))
        .with_state(AppState {
            sys: Arc::new(Mutex::new(System::new())),
        });
    let server = Server::bind(&"0.0.0.0:7032".parse().unwrap()).serve(router.into_make_service());
    let addr = server.local_addr();
    println!("Listening on {addr}");
    server.await.unwrap();
}

#[derive(Clone, Debug)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

async fn root_get() -> &'static str {
    "Hello"
}

async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
    let mut sys = state.sys.lock().await;
    sys.refresh_cpu();

    let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
    Json(v)
}
