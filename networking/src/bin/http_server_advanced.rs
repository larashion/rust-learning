use axum::{
    extract::{ws::{WebSocket, WebSocketUpgrade}, State, ConnectInfo},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use tower_http::compression::CompressionLayer;

#[derive(Clone)]
struct WsAppState {
    tx: broadcast::Sender<String>,
}

struct RateLimiter {
    requests: Arc<Mutex<HashMap<SocketAddr, u32>>>,
}

#[tokio::main]
async fn main() {
    let app = setup_app();
    let addr = "127.0.0.1:3014";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("高级功能服务器运行在 http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

fn setup_app() -> Router {
    let (tx, _) = broadcast::channel(100);
    let ws_state = WsAppState { tx };
    let limiter = Arc::new(RateLimiter {
        requests: Arc::new(Mutex::new(HashMap::new())),
    });

    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(ws_state)
        .route("/limited", get(rate_limited))
        .with_state(limiter)
        .route("/protected", get(protected))
        .layer(CompressionLayer::new())
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<WsAppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: WsAppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(axum::extract::ws::Message::Text(msg)).await.is_err() { break; }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let axum::extract::ws::Message::Text(text) = msg {
                println!("WS 收到: {}", text);
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

async fn rate_limited(State(limiter): State<Arc<RateLimiter>>, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    let mut requests = limiter.requests.lock().await;
    let count = requests.entry(addr).or_insert(0);
    *count += 1;
    if *count > 5 { "速率限制".to_string() } else { format!("请求次数: {}", *count) }
}

async fn protected(headers: HeaderMap) -> Result<&'static str, StatusCode> {
    if let Some(auth) = headers.get("authorization").and_then(|h| h.to_str().ok()) {
        if auth == "Bearer valid_token" { return Ok("认证成功"); }
    }
    Err(StatusCode::UNAUTHORIZED)
}