use std::net::SocketAddr;

use axum::{
    extract::Host,
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    response::Redirect,
    BoxError,
};
use tracing::info;

use crate::args::Args;

/// Task that redirects HTTP traffic to HTTPS
/// # Panics
/// On socket permission errors
pub async fn redirect_http_to_https(args: Args) {
    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(&host, uri, args) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], args.http_port));
    info!("listening on {addr} for https redirect");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, redirect.into_make_service())
        .await
        .unwrap();
}

fn make_https(host: &str, uri: Uri, args: Args) -> Result<Uri, BoxError> {
    info!("Redirecting {host} {uri}");
    let mut parts = uri.into_parts();

    parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

    if parts.path_and_query.is_none() {
        parts.path_and_query = Some("/".parse().unwrap());
    }

    let https_host = host.replace(&args.http_port.to_string(), &args.https_port.to_string());
    parts.authority = Some(https_host.parse()?);

    Ok(Uri::from_parts(parts)?)
}
