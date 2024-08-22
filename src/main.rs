use std::{net::SocketAddr, path::PathBuf};

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;
use tokio::fs;
use tracing::info;

use ric3::args::Args;
use ric3::assets;
use ric3::posts;
use ric3::ssl_redirect::redirect_http_to_https;

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // Redirect all HTTP traffic to HTTPS
    tokio::spawn(redirect_http_to_https(args));

    // configure certificate and private key used by https
    let config = RustlsConfig::from_pem_file(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("certs")
            .join("cert.pem"),
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("certs")
            .join("private-key.pem"),
    )
    .await
    .unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/site.webmanifest", get(web_manifest))
        .merge(assets::asset_router())
        .merge(posts::post_router());

    let addr = SocketAddr::from(([0, 0, 0, 0], args.https_port));
    info!("Listening on {addr}");
    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[tracing::instrument]
async fn web_manifest() -> impl IntoResponse {
    info!("Web manifest requested");
    fs::read_to_string("static/site.webmanifest")
        .await
        .expect("Web manifest not found")
}

#[tracing::instrument]
async fn root() -> impl IntoResponse {
    info!("Root requested");
    let index_content = fs::read_to_string("static/index.html")
        .await
        .expect("Root content page not found");
    Html(index_content)
}
