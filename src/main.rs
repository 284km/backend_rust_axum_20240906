use axum::{Router, routing::get};

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3051").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

