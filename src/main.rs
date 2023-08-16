use std::sync::Arc;

use axum::{response::Html, routing::*, Router};
use html_node::typed::{self, elements::*};

mod routes;

use routes::*;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new()
        .route("/", get(home::get))
        .route("/test", get(test_function))
        .nest("/todo", todos::todos_router())
        .with_state(Arc::new(vec![
            todos::Todo {
                id: 0,
                title: "Test title".to_string(),
                done: false,
            },
            todos::Todo {
                id: 1,
                title: "Maybe tit".to_string(),
                done: false,
            },
            todos::Todo {
                id: 2,
                title: "To be done".to_string(),
                done: true,
            },
        ]));

    axum::Server::bind(&"0.0.0.0:42069".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn test_function() -> Html<String> {
    let div = typed::html! { (hx)
        <div class="p-2" hx-swap="outerHtml" >
            <h1>Title Heading</h1>
            <p>This is a parameter</p>
        </div>
    };
    div.to_string().into()
}
