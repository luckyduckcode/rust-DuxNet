use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, Response},
    routing::get,
    Router,
};
use std::fs;
use tower_http::services::ServeDir;

pub fn create_frontend_router() -> Router {
    Router::new()
        .route("/", get(serve_index))
        .route("/index.html", get(serve_index))
        .nest_service("/static", ServeDir::new("static"))
}

async fn serve_index() -> Result<Html<String>, StatusCode> {
    let html_content = include_str!("../../static/index.html");
    Ok(Html(html_content.to_string()))
}

pub async fn serve_static_file(Path(path): Path<String>) -> Result<Response<String>, StatusCode> {
    let file_path = format!("static/{}", path);
    
    match fs::read_to_string(&file_path) {
        Ok(content) => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .body(content)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(response)
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
} 