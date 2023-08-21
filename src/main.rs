use axum::{
    async_trait,
    body::{self, Empty, Full},
    extract::{FromRequestParts, Path},
    http::{header, HeaderValue, Response, StatusCode},
    response::IntoResponse,
    routing::*,
    Router,
};
mod layout;
mod routes;

use include_dir::{include_dir, Dir};
use layout::layout;
use routes::*;

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new()
        .route("/static/*path", get(static_path))
        .route("/", get(home::get))
        .merge(todos::todos_router());

    axum::Server::bind(&"0.0.0.0:42069".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    match STATIC_DIR.get_file(path) {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
        Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(body::boxed(Full::from(file.contents())))
            .unwrap(),
    }
}

#[derive(Debug)]
struct Htmx {
    fullpage: bool,
}

#[async_trait]
impl<S> FromRequestParts<S> for Htmx
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Htmx {
            fullpage: !parts.headers.contains_key("HX-Request")
                || parts.headers.contains_key("HX-Boosted"),
        })
    }
}
