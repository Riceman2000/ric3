use std::net::{IpAddr, SocketAddr};

use askama::Template;
use axum::{
    extract::{ConnectInfo, Path},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use tracing::info;

pub fn router() -> Router {
    Router::new()
        .route("/qr/:qr_id", get(qr_handler))
        .route("/qr", get(qr_default))
}

#[derive(Template)]
#[template(path = "default-page.html")]
struct QrTemplate {
    title: String,
    content: String,
}

impl QrTemplate {
    fn from_metadata(metadata: &QrMetadata) -> Self {
        let title = String::from("Rice Co.");
        let content = format!("QR-ID: {}, IP: {}", metadata.qr_id, metadata.ip);
        Self { title, content }
    }
}

#[derive(Deserialize, Debug)]
struct QrMetadata {
    qr_id: String,
    ip: IpAddr,
}

#[tracing::instrument]
async fn qr_default(connect_info: ConnectInfo<SocketAddr>) -> impl IntoResponse {
    qr_handler(connect_info, Path("DEFAULT".to_string())).await
}

#[tracing::instrument]
async fn qr_handler(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(qr_id): Path<String>,
) -> impl IntoResponse {
    info!("QR Code requested with id {qr_id}");

    // Gather and parse metadata
    let ip = addr.ip();
    let metadata = QrMetadata { qr_id, ip };
    info!("Qr metadata parsed as {metadata:#?}");

    // Gather and parse content markdown
    let content_string = format!("TEST -> {}", metadata.ip);
    let parser = pulldown_cmark::Parser::new(&content_string);
    let mut content_html = String::new();
    pulldown_cmark::html::push_html(&mut content_html, parser);

    // Place post in template then render
    let post_template = QrTemplate::from_metadata(&metadata);
    Html(post_template.render().unwrap())
}
