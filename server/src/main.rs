use axum::{routing::get, serve, Router};
use human_panic::setup_panic;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    setup_panic!();

    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    let listener = TcpListener::bind("localhost:3000").await.unwrap();

    serve(listener, app).await.unwrap();
}
