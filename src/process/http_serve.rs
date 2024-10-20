use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::net::TcpListener;
use tracing::{info, warn};

struct HttpServeStatus {
    path: PathBuf,
}

pub async fn process_http_serve(directory: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on [http://0.0.0.0:{}]", directory, port);

    let status = HttpServeStatus { path: directory };
    let app = Router::new()
        .route("/*path", get(file_handler))
        .with_state(Arc::new(status));

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn file_handler(
    State(status): State<Arc<HttpServeStatus>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let path = status.path.join(&path);
    info!("Reading {:?}", path);
    if !path.exists() {
        (
            StatusCode::NOT_FOUND,
            format!("{:?} not found", path.display()),
        )
    } else {
        let content = match tokio::fs::read_to_string(path).await {
            Ok(content) => content,
            Err(e) => {
                warn!("Failed to read file: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to read file: {}", e),
                );
            }
        };
        (StatusCode::OK, content)
    }
}
