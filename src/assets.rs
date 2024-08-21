use std::path::PathBuf;

use axum::{
    body::Body,
    extract::Path,
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tokio::fs;
use tokio_util::io::ReaderStream;
use tracing::info;

pub fn asset_router() -> Router {
    Router::new()
        .route("/assets/style.css", get(style))
        .route("/assets/img/:img", get(asset_image))
        .route("/assets/favicon/:img", get(asset_favicon))
        .route(
            "/favicon.ico",
            get(|| asset_favicon(Path("favicon.ico".to_string()))),
        )
}

#[tracing::instrument]
pub async fn asset_favicon(Path(img): Path<String>) -> impl IntoResponse {
    info!("Favicon requested");
    let image_path = format!("assets/favicon/{img}");
    image_response(image_path.into()).await
}

#[tracing::instrument]
pub async fn asset_image(Path(img): Path<String>) -> impl IntoResponse {
    info!("Image requested");

    let image_path = format!("assets/img/{img}");
    image_response(image_path.into()).await
}

async fn image_response(image_path: PathBuf) -> impl IntoResponse {
    let not_found_path = "assets/img/not-found.png";
    let file_name = image_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let Some(content_type) = mime_guess::from_path(&image_path).first_raw() else {
        return Err((StatusCode::BAD_REQUEST, "MIME Type couldn't be determined"));
    };
    let file = match fs::File::open(image_path).await {
        Ok(file) => file,
        Err(_e) => fs::File::open(not_found_path)
            .await
            .expect("fallback image missing"),
    };
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_str(content_type).unwrap(),
        )
        .header(
            header::CONTENT_DISPOSITION,
            HeaderValue::from_str(&file_name).unwrap(),
        )
        .header(
            header::CACHE_CONTROL,
            HeaderValue::from_static("max-age=3600, must-revalidate"),
        )
        .body(body)
        .unwrap())
}

#[tracing::instrument]
pub async fn style() -> impl IntoResponse {
    info!("style requested");
    let content = fs::read_to_string("assets/style.css")
        .await
        .expect("Style sheet is missing");

    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/css; charset=utf-8"),
        )
        .header(
            header::CACHE_CONTROL,
            HeaderValue::from_static("max-age=3600, must-revalidate"),
        )
        .body(Body::from(content))
        .unwrap()
}
