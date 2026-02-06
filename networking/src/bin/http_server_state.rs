use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Instant;
use tower_http::cors::{Any, CorsLayer};

struct AppState {
    counter: Arc<Mutex<i32>>,
}

#[tokio::main]
async fn main() {
    let app = setup_router();
    let addr = "127.0.0.1:3007";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("状态与中间件服务器运行在 http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

fn setup_router() -> Router {
    let state = Arc::new(AppState {
        counter: Arc::new(Mutex::new(0)),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any);

    Router::new()
        .route("/counter", get(get_counter))
        .layer(middleware::from_fn(logging_middleware))
        .layer(cors)
        .with_state(state)
}

async fn get_counter(State(state): State<Arc<AppState>>) -> String {
    let mut counter = state.counter.lock().await;
    *counter += 1;
    format!("计数: {}", *counter)
}

async fn logging_middleware(req: Request<axum::body::Body>, next: Next) -> Result<Response, StatusCode> {
    let start = Instant::now();
    let uri = req.uri().clone();
    let response = next.run(req).await;
    println!("请求 {} 处理耗时: {:?}", uri, start.elapsed());
    Ok(response)
}