use std::fs;

use askama::Template;
use axum::{
    body::Body,
    extract::Path,
    http::{header, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use tokio_util::io::ReaderStream;
use tracing::info;

#[derive(Template)]
#[template(path = "blog-post.html", escape = "none")]
struct BlogTemplate<'a> {
    content: &'a str,
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/assets/style.css", get(style))
        .route("/assets/img/:img", get(asset_image))
        .route("/posts/:post_id", get(posts));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[tracing::instrument]
async fn root() -> impl IntoResponse {
    info!("Root requested");
    let index_content = fs::read_to_string("static/index.html").unwrap();
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

#[tracing::instrument]
async fn asset_image(Path(img): Path<String>) -> impl IntoResponse {
    info!("Image requested {img}");

    let image_path = format!("assets/img/{img}");
    let content_type = match mime_guess::from_path(&image_path).first_raw() {
        Some(mime) => mime,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                "MIME Type couldn't be determined".to_string(),
            ))
        }
    };
    let file = match tokio::fs::File::open(image_path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
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
        .body(body)
        .unwrap())

    // TODO: Detect image type
    // Response::builder()
    //     .status(StatusCode::OK)
    //     .header("Content-Type", "image/jpeg")
    //     .body(image_bytes)
    //     .unwrap()
}

#[tracing::instrument]
async fn style() -> impl IntoResponse {
    info!("style requested");
    let content = fs::read_to_string("assets/style.css").unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/css; charset=utf-8"),
        )
        .body(Body::from(content))
        .unwrap()
}
