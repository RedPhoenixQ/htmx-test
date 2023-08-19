use axum::{async_trait, extract::FromRequestParts, routing::*, Router};
mod layout;
mod routes;

use layout::layout;
use routes::*;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new()
        .route("/", get(home::get))
        .merge(todos::todos_router());

    axum::Server::bind(&"0.0.0.0:42069".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug)]
struct Htmx(bool);

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
        Ok(Htmx(parts.headers.contains_key("HX-Request")))
    }
}
