use axum::{
    extract::{Path, Query},
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = setup_router();
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("基础服务器运行在 http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

fn setup_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/users/:id", get(get_user))
        .route("/list_users", get(list_users))
        .route("/login", post(login))
}

async fn root() -> &'static str { "欢迎使用 Axum！" }
async fn hello() -> &'static str { "Hello, Axum!" }

async fn get_user(Path(user_id): Path<String>) -> String {
    format!("用户 ID: {}", user_id)
}

#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}

async fn list_users(Query(params): Query<Pagination>) -> String {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    format!("第 {} 页，每页 {} 条", page, limit)
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    _password: String,
}

async fn login(Form(form): Form<LoginForm>) -> String {
    format!("登录成功: {}", form.username)
}