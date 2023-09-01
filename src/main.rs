use anyhow::Result;
use axum::{
    async_trait,
    body::{self, Empty, Full},
    extract::{FromRequestParts, Path},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::*,
    Router,
};
mod layout;
mod routes;

use include_dir::{include_dir, Dir};
use layout::layout;
use routes::*;
use surrealdb::{
    self,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static");

static DB: Surreal<Client> = Surreal::init();

async fn init_db() -> Result<()> {
    println!("connecting to db...");
    DB.connect::<Ws>("127.0.0.1:8000").await?;
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    println!("Connection established {:?}", DB.health());
    DB.use_ns("todo").use_db("todo").await?;
    println!("ns and db in use");

    DB.query(todos::TABLE_INIT).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    init_db().await?;
    println!("Hello, world!");

    let app = Router::new()
        .route("/static/*path", get(static_path))
        .route("/", get(home::get))
        .nest("/todo", todos::todos_router());

    let server = axum::Server::bind(&"0.0.0.0:8080".parse()?).serve(app.into_make_service());

    server.await?;

    Ok(())
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

struct AppError(anyhow::Error);

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        eprintln!("{}", self.0.to_string());
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

#[derive(Debug)]
struct Htmx {
    fullpage: bool,
    target: Option<String>,
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
            target: parts
                .headers
                .get("HX-Target")
                .and_then(|v| v.to_str().ok().and_then(|s| Some(s.to_string()))),
        })
    }
}
