use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    age: u32,
}

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    age: u32,
}

static USER_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

#[tokio::main]
async fn main() {
    let app = Router::new().route("/users", post(create_user));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3002").await.unwrap();
    println!("JSON API 服务器运行在 http://127.0.0.1:3002");
    axum::serve(listener, app).await.unwrap();
}

async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    let id = USER_ID_COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    Json(User {
        id,
        name: payload.name,
        age: payload.age,
    })
}
