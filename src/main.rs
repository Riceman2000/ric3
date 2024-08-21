use std::{net::SocketAddr, path::PathBuf};

use askama::Template;
use axum::{
    extract::Path,
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
use ric3::ssl_redirect::redirect_http_to_https;

#[derive(Template)]
#[template(path = "blog-post.html", escape = "none")]
struct BlogTemplate<'a> {
    content: &'a str,
}

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
        .route("/posts/:post_id", get(posts));

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

#[tracing::instrument]
async fn posts(Path(post_id): Path<String>) -> impl IntoResponse {
    info!("Blog post requested with id {post_id}");

    let markdown_input = format!("Markdown with {post_id}!\n- bullet\n- bullet\n# Header!");
    let parser = pulldown_cmark::Parser::new(&markdown_input);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    // Place post in template then render
    let blog_html = BlogTemplate {
        content: &html_output,
    };
    Html(blog_html.render().unwrap())
}
