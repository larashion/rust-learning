use axum::{
    extract::Multipart,
    routing::post,
    Router,
};
use tower_http::services::ServeDir;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/upload", post(upload));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3010").await.unwrap();
    println!("文件服务器运行在 http://127.0.0.1:3010");
    println!("静态文件目录: /static -> ./static");
    axum::serve(listener, app).await.unwrap();
}

async fn upload(mut multipart: Multipart) -> Result<String, String> {
    while let Some(field) = multipart.next_field().await.map_err(|e| e.to_string())? {
        let filename = field.file_name().unwrap_or("unknown").to_string();
        let data = field.bytes().await.map_err(|e| e.to_string())?;

        let _ = tokio::fs::create_dir_all("uploads").await;
        let path = format!("uploads/{}", filename);
        let mut file = File::create(&path).await.map_err(|e| e.to_string())?;
        file.write_all(&data).await.map_err(|e| e.to_string())?;
    }
    Ok("上传成功".to_string())
}
