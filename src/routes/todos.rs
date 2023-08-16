use std::{borrow::BorrowMut, collections::HashMap, hash::Hasher, sync::{Arc, Mutex}, ops::DerefMut};

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, Response},
    routing::*,
    Form, Router,
};
use html_node::{
    text,
    typed::{self, elements::*},
    Node,
};
use serde::Deserialize;

use crate::Htmx;

#[derive(Debug)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Deserialize)]
struct Id {
    id: u32,
}

pub fn todos_router() -> Router {
    let mut todos = HashMap::new();
    todos.insert(0, Todo {
                id: 0,
                title: "Test title".to_string(),
                done: false,
            });
    todos.insert(1, Todo {
                id: 1,
                title: "Maybe tit".to_string(),
                done: false,
            });
    todos.insert(2, Todo {
                id: 2,
                title: "To be done".to_string(),
                done: true,
           } );
    Router::new()
        .route("/", get(get_todos).post(add_todo).delete(remove_todo))
        .with_state(Arc::new(Mutex::new(todos)))
}

async fn add_todo(htmx: Htmx, todos: State<Arc<Mutex<HashMap<u32, Todo>>>>) {
    dbg!("Is htmx request", htmx);
    dbg!(todos);
}

async fn remove_todo(mut todos: State<Arc<Mutex<HashMap<u32, Todo>>>>, htmx: Htmx, Form(id): Form<Id>) {
    dbg!("Is htmx request", htmx, &id);
    todos.deref_mut().lock().unwrap().remove_entry(&id.id);
    dbg!(todos);   
}

async fn get_todos(htmx: Htmx, todos: State<Arc<Mutex<HashMap<u32, Todo>>>>) -> Html<String> {
    dbg!("Is htmx request", &htmx);
    let table = todos_table(todos.lock().unwrap().values()).to_string();
    if htmx.0 {
        table.into()
    } else {
        format!("<head>
            <script src=\"https://unpkg.com/htmx.org@1.9.4\" 
                integrity=\"sha384-zUfuhFKKZCbHTY6aRR46gxiqszMk5tcHjsVFxnUo8VMus4kHGVdIYVbOYYNlKmHV\" 
                crossorigin=\"anonymous\"></script>
            </head>
            <body>{}</body>"
            , &table
        ).into()
    }
}

fn single_todo_row(todo: &Todo) -> Node {
    typed::html!((hx)
        <tr>
            <td>{text!("{}", todo.title)}</td>
            <td>
                <input
                    class="checkbox"
                    type="checkbox"
                    checked=format!("{:?}", todo.done)
                    hx-post="/todo"
                    hx-vals={format!("\"id\": {}", todo.id)}
                />
            </td>
            <td>
                <button hx-delete="/todo" hx-vals={format!("\"id\": {}", todo.id)}>X</button>
            </td>
        </tr>
    )
}

fn todos_table<'a>(todos: impl Iterator<Item = &'a Todo>) -> Node {
    typed::html!((hx)
        <table>
            <thead>
                <tr>
                    <th>"Title"</th>
                    <th>"Done"</th>
                </tr>
            </thead>
            <tbody>
                {todos.map(|todo| single_todo_row(&todo))}
            </tbody>
        </table>
    )
}
