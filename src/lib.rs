use axum::{routing::get, Router};
use tower_service::Service;
use worker::*;

mod header;
mod macros;

use header::wrap_html_with_header;

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once(); // Log rust panics to console.error in JS
    Ok(router().call(req).await?)
}

fn router() -> Router {
    Router::new()
        .route("/favicon.ico", get(favicon))
        .route("/", get(root))
        .route("/projects", get(projects))
        .fallback(get(not_found))
}

/// Handler for the `/favicon.ico` route.
async fn favicon() -> axum::http::Response<axum::body::Body> {
    axum::http::Response::builder()
        .header("Content-Type", "image/x-icon")
        .body(axum::body::Body::from(
            include_bytes!("../static/favicon.ico").to_vec(),
        ))
        .unwrap()
}

/// Handler for the `/` route.
async fn root() -> axum::http::Response<axum::body::Body> {
    wrap_html_with_header(include_markdown!("../static/pages/index.md"))
}

/// Handler for the `/projects` route.
async fn projects() -> axum::http::Response<axum::body::Body> {
    wrap_html_with_header(include_markdown!("../static/pages/projects.md"))
}

async fn not_found() -> axum::http::Response<axum::body::Body> {
    wrap_html_with_header(include_markdown!("../static/pages/404.md"))
}
