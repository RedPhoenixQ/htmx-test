use axohtml::{dom::DOMTree, html};
use axum::{
    response::Html,
    routing::{get, post},
    Router,
};

mod routes;

use crate::routes::*;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new()
        .route("/", get(home::get))
        .route("/test", get(test_function))
        .nest("/todo", todos_router());

    axum::Server::bind(&"0.0.0.0:42069".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn test_function() -> Html<String> {
    let div: DOMTree<String> = html!(
        <div>
            <h1>"Title Heading"</h1>
            <p>"This is a parameter"</p>
        </div>
    );
    div.to_string().into()
}
