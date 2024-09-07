use std::env;

use axum::{
    http::StatusCode, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};

use dotenv::dotenv;

async fn root() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    // logging
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();
    // .ok()の意味:
    // dotenv()はResult型を返します
    // .ok()は、成功時には結果を無視し、
    // 失敗（エラー）が発生した場合もエラーを無視するために使います
    // つまり、.envファイルが存在しない、
    // もしくは何かしら理由で読み込めなかったとしても、
    // そのエラーを無視して続行することを意味します
    dotenv().ok();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3051").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1001,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
