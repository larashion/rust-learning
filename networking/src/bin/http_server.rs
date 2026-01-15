// #![allow(unused)] // Cleaned up: Removed global suppression
// ============================================================================
// HTTP 服务端 - Axum
// ============================================================================// 依赖: axum = "0.7", tokio = { version = "1", features = ["full"] }

use axum::routing::{get, post};
use axum::Router;
use axum::Json;
use axum::extract::{Path, Query, State, Multipart};
use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::http::{Request, StatusCode, HeaderMap};
use axum::response::{Response, IntoResponse};
use axum::middleware::{self, Next};
use axum::Form;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};
use std::time::Instant;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tower_http::compression::CompressionLayer;
use std::io::Write;
use tokio::fs::File;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::io::AsyncWriteExt;
use futures_util::{StreamExt, SinkExt};

async fn hello_world() -> &'static str {
    "Hello, World!"
}

#[allow(dead_code)]
#[tokio::main]
async fn example1_hello_world() {
    let app = Router::new().route("/", get(hello_world));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "欢迎使用 Axum！"
}
async fn hello() -> &'static str {
    "Hello, Axum!"
}
async fn goodbye() -> &'static str {
    "Goodbye!"
}
#[allow(dead_code)]
#[tokio::main]
async fn example2_multiple_routes() {
    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/goodbye", get(goodbye));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3001");
    axum::serve(listener, app).await.unwrap();
}

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
static mut USER_ID: u32 = 0;
async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    unsafe {
        USER_ID += 1;
        Json(User {
            id: USER_ID,
            name: payload.name,
            age: payload.age,
        })
    }
}
#[allow(dead_code)]
#[tokio::main]
async fn example3_json_api() {
    let app = Router::new().route("/users", post(create_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3002");
    axum::serve(listener, app).await.unwrap();
}

async fn get_user(Path(user_id): Path<String>) -> String {
    format!("用户 ID: {}", user_id)
}
async fn get_post(Path((user_id, post_id)): Path<(String, String)>) -> String {
    format!("用户 ID: {}, 文章 ID: {}", user_id, post_id)
}
#[allow(dead_code)]
#[tokio::main]
async fn example4_path_params() {
    let app = Router::new()
        .route("/users/:id", get(get_user))
        .route("/users/:id/posts/:post_id", get(get_post));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3003");
    axum::serve(listener, app).await.unwrap();
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
#[allow(dead_code)]
#[tokio::main]
async fn example5_query_params() {
    let app = Router::new().route("/users", get(list_users));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3004").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3004");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}
async fn login(Form(form): Form<LoginForm>) -> String {
    format!("登录: {}", form.username)
}
#[allow(dead_code)]
#[tokio::main]
async fn example6_form_data() {
    let app = Router::new().route("/login", post(login));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3005").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3005");
    axum::serve(listener, app).await.unwrap();
}

use axum::http::HeaderMap as AxumHeaderMap;
async fn get_headers(headers: AxumHeaderMap) -> String {
    let user_agent = headers.get("user-agent").and_then(|v| v.to_str().ok()).unwrap_or("Unknown");
    format!("User-Agent: {}", user_agent)
}
#[allow(dead_code)]
#[tokio::main]
async fn example7_headers() {
    let app = Router::new().route("/headers", get(get_headers));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3006").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3006");
    axum::serve(listener, app).await.unwrap();
}

struct AppState {
    counter: Arc<Mutex<i32>>,
}
async fn get_counter(State(state): State<Arc<AppState>>) -> String {
    let mut counter = state.counter.lock().await;
    *counter += 1;
    format!("计数: {}", *counter)
}
#[allow(dead_code)]
#[tokio::main]
async fn example8_state() {
    let state = Arc::new(AppState {
        counter: Arc::new(Mutex::new(0)),
    });
    let app = Router::new()
        .route("/counter", get(get_counter))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3007").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3007");
    axum::serve(listener, app).await.unwrap();
}

async fn my_middleware(
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let start = Instant::now();
    let uri = req.uri().clone();
    println!("{} - 开始处理", uri);
    let response = next.run(req).await;
    println!("{} - 完成，耗时 {:?}", uri, start.elapsed());
    Ok(response)
}
async fn hello_middleware() -> &'static str {
    "Hello with middleware!"
}
#[allow(dead_code)]
#[tokio::main]
async fn example9_middleware() {
    let app = Router::new()
        .route("/", get(hello_middleware))
        .layer(middleware::from_fn(my_middleware));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3008").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3008");
    axum::serve(listener, app).await.unwrap();
}

async fn cors_handler() -> &'static str {
    "CORS enabled!"
}
#[allow(dead_code)]
#[tokio::main]
async fn example10_cors() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let app = Router::new()
        .route("/", get(cors_handler))
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3009").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3009");
    axum::serve(listener, app).await.unwrap();
}

#[allow(dead_code)]
#[tokio::main]
async fn example11_static_files() {
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3010").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3010");
    axum::serve(listener, app).await.unwrap();
}

async fn hello_logging() -> &'static str {
    "Hello with logging!"
}
#[allow(dead_code)]
#[tokio::main]
async fn example12_logging() {
    let app = Router::new()
        .route("/", get(hello_logging))
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3011").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3011");
    axum::serve(listener, app).await.unwrap();
}

async fn handler_with_error() -> Result<Json<serde_json::Value>, StatusCode> {
    if true {
        Err(StatusCode::BAD_REQUEST)
    } else {
        Ok(Json(json!({ "message": "成功" })))
    }
}
#[allow(dead_code)]
#[tokio::main]
async fn example13_error_handling() {
    let app = Router::new().route("/", get(handler_with_error));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3012").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3012");
    axum::serve(listener, app).await.unwrap();
}

async fn upload(mut multipart: Multipart) -> Result<String, String> {
    let mut uploaded_files = Vec::new();

    while let Some(field) = match multipart.next_field().await {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    } {
        let filename = field.file_name().unwrap_or("unknown").to_string();
        let data = match field.bytes().await {
            Ok(d) => d,
            Err(e) => return Err(e.to_string()),
        };

        let path = format!("uploads/{}", filename);
        let _ = tokio::fs::create_dir_all("uploads").await;
        
        let mut file = match File::create(&path).await {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };
        
        if let Err(e) = file.write_all(&data).await {
            return Err(e.to_string());
        }

        uploaded_files.push(path);
    }

    if uploaded_files.is_empty() {
        Ok("没有文件".to_string())
    } else {
        Ok(format!("上传成功: {:?}", uploaded_files))
    }
}
#[allow(dead_code)]
#[tokio::main]
async fn example14_file_upload() {
    let app = Router::new().route("/upload", post(upload));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3013").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3013");
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct WsAppState {
    tx: broadcast::Sender<String>,
}
async fn ws_handler(
    State(state): State<WsAppState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}
async fn handle_socket(
    socket: WebSocket,
    state: WsAppState,
) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(axum::extract::ws::Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let axum::extract::ws::Message::Text(text) = msg {
                println!("收到: {}", text);
            }
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
}
#[allow(dead_code)]
#[tokio::main]
async fn example15_websocket() {
    let (tx, _) = broadcast::channel(100);
    let state = WsAppState { tx };
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3014").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3014");
    axum::serve(listener, app).await.unwrap();
}

async fn v1_hello() -> &'static str { "API v1" }
async fn v2_hello() -> &'static str { "API v2" }
#[allow(dead_code)]
#[tokio::main]
async fn example16_nested_routes() {
    let app = Router::new()
        .route("/hello", get(v1_hello))
        .nest("/api/v2", Router::new()
            .route("/hello", get(v2_hello)));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3015").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3015");
    axum::serve(listener, app).await.unwrap();
}

async fn get_users_db() -> String {
    let users = HashMap::from([
        ("1", "Alice"),
        ("2", "Bob"),
    ]);
    serde_json::to_string(&users).unwrap()
}
#[allow(dead_code)]
#[tokio::main]
async fn example17_database() {
    let app = Router::new().route("/users", get(get_users_db));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3016").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3016");
    axum::serve(listener, app).await.unwrap();
}

async fn protected(headers: HeaderMap) -> Result<&'static str, StatusCode> {
    let auth_header = headers.get("authorization").and_then(|h| h.to_str().ok());
    if let Some(token) = auth_header {
        if token == "Bearer valid_token" {
            return Ok("认证成功！");
        }
    }
    Err(StatusCode::UNAUTHORIZED)
}
#[allow(dead_code)]
#[tokio::main]
async fn example18_jwt_auth() {
    let app = Router::new().route("/protected", get(protected));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3017").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3017");
    axum::serve(listener, app).await.unwrap();
}

async fn large_response() -> String {
    "A".repeat(10000)
}
#[allow(dead_code)]
#[tokio::main]
async fn example19_compression() {
    let app = Router::new()
        .route("/", get(large_response))
        .layer(CompressionLayer::new());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3018").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3018");
    axum::serve(listener, app).await.unwrap();
}

struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, u32>>>,
}
async fn rate_limited(
    State(limiter): State<Arc<RateLimiter>>,
    addr: axum::extract::ConnectInfo<SocketAddr>,
) -> String {
    let ip = addr.ip().to_string();
    let mut requests = limiter.requests.lock().await;
    let count = requests.entry(ip).or_insert(0);
    *count += 1;
    if *count > 10 {
        "速率限制: 每分钟 10 次请求".to_string()
    } else {
        format!("请求次数: {}", *count)
    }
}
#[allow(dead_code)]
#[tokio::main]
async fn example20_rate_limit() {
    let limiter = Arc::new(RateLimiter {
        requests: Arc::new(Mutex::new(HashMap::new()))
    });
    let app = Router::new()
        .route("/limited", get(rate_limited))
        .with_state(limiter);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3019").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3019");
    axum::serve(listener, app).await.unwrap();
}

fn main() {
    println!("=== HTTP 服务端 - Axum 示例 ===\n");
    println!("Code is uncommented. Run specific examples via cargo run.");
}