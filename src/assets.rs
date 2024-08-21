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
}
#[tracing::instrument]
pub async fn asset_image(Path(img): Path<String>) -> impl IntoResponse {
    info!("Image requested {img}");

    let image_path = format!("assets/img/{img}");
    let Some(content_type) = mime_guess::from_path(&image_path).first_raw() else {
        return Err((
            StatusCode::BAD_REQUEST,
            "MIME Type couldn't be determined".to_string(),
        ));
    };
    let file = match fs::File::open(image_path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {err}"))),
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
            HeaderValue::from_str(&img).unwrap(),
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
