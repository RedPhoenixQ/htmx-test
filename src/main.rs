use axum::{async_trait, extract::FromRequestParts, response::Html, routing::*, Router};
use html_node::typed::{self, elements::*};

mod routes;

use routes::*;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new()
        .route("/", get(home::get))
        .route("/test", get(test_function))
        .nest("/todo", todos::todos_router());

    axum::Server::bind(&"0.0.0.0:42069".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn test_function(htmx: Htmx) -> Html<String> {
    let div = typed::html! { (hx)
        <div class="p-2" hx-swap="outerHtml" >
            <h1>Title Heading</h1>
            <p>This is a parameter</p>
        </div>
    };
    div.to_string().into()
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
