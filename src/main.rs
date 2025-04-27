mod api;
mod errors;
mod inference;
mod models;

use crate::inference::{load_session, InferenceImpl};
use axum::routing::get;
use axum::Router;
use ort::session::Session;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct AppState {
    pub session: Arc<Session>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let session = Arc::new(load_session("mnist-8.onnx".to_string())?);
    let inference = Arc::new(InferenceImpl {
        session: session.clone(),
    });

    let app = Router::new()
        .route("/", get(hello_world))
        .nest("/predict", api::router(inference.clone()))
        .fallback(get(anything_else));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    let _ = axum::serve(listener, app).await?;
    Ok(())
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}

async fn anything_else() -> &'static str {
    "This is a fallback route."
}
