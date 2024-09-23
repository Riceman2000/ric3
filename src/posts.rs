use std::path::PathBuf;

use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use tokio::fs;
use toml::value::Datetime;
use tracing::info;

pub fn router() -> Router {
    Router::new().route("/posts/:post_id", get(post_handler))
}

#[derive(Template)]
#[template(path = "blog-post.html", escape = "none")]
struct BlogTemplate {
    title: String,
    author: String,
    date_published: String,
    date_updated: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct BlogMetadata {
    title: String,
    author: String,
    date_published: Datetime,
    date_updated: Datetime,
    post_type: String,
    synopsis: String,
}

impl BlogTemplate {
    fn from_metadata(metadata: BlogMetadata, content: String) -> Self {
        let date_published = metadata.date_published.date.unwrap().to_string();
        let date_updated = metadata.date_updated.date.unwrap().to_string();
        Self {
            title: metadata.title,
            author: metadata.author,
            date_published,
            date_updated,
            content,
        }
    }
}

#[tracing::instrument]
async fn post_handler(Path(post_id): Path<String>) -> impl IntoResponse {
    info!("Blog post requested with id {post_id}");

    let mut post_path = PathBuf::from(format!("posts/{post_id}"));
    if !post_path.is_dir() {
        post_path = PathBuf::from("posts/default");
    }

    // Gather and parse metadata
    let mut metadata_path = post_path.clone();
    metadata_path.push("metadata.toml");
    if !metadata_path.is_file() {
        return Err((StatusCode::BAD_REQUEST, "No post metadata found"));
    }
    let metadata_string = match fs::read_to_string(metadata_path).await {
        Ok(m) => m,
        Err(_e) => {
            return Err((StatusCode::BAD_REQUEST, "Metadata can not be read"));
        }
    };
    let metadata: BlogMetadata = match toml::from_str(&metadata_string) {
        Ok(m) => m,
        Err(_e) => {
            return Err((StatusCode::BAD_REQUEST, "Metadata can not be parsed"));
        }
    };
    info!("Post metadata parsed as {metadata:#?}");

    // Gather and parse content markdown
    let mut content_path = post_path.clone();
    content_path.push("content.md");
    if !content_path.is_file() {
        return Err((StatusCode::BAD_REQUEST, "No post content found"));
    }
    let content_string = match fs::read_to_string(content_path).await {
        Ok(c) => c,
        Err(_e) => {
            return Err((StatusCode::BAD_REQUEST, "Content can not be read"));
        }
    };
    let parser = pulldown_cmark::Parser::new(&content_string);
    let mut content_html = String::new();
    pulldown_cmark::html::push_html(&mut content_html, parser);

    // Place post in template then render
    let post_template = BlogTemplate::from_metadata(metadata, content_html);
    Ok(Html(post_template.render().unwrap()))
}
