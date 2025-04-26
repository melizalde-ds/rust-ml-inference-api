use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/", get(hello_world).fallback(get(anything_else)));
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
