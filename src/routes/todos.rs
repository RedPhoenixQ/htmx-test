use std::sync::Arc;

use axum::{extract::State, response::Html, routing::*, Router};
use html_node::{
    text,
    typed::{self, elements::*},
    Node,
};

#[derive(Debug)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

pub fn todos_router() -> Router<Arc<Vec<Todo>>> {
    Router::new().route("/", get(get_todos).post(add_todo))
}

async fn add_todo(todos: State<Arc<Vec<Todo>>>) {
    dbg!(todos);
}

async fn get_todos(todos: State<Arc<Vec<Todo>>>) -> Html<String> {
    todos_table(&todos).to_string().into()
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
        </tr>
    )
}

fn todos_table(todos: &Vec<Todo>) -> Node {
    typed::html!((hx)
        <table>
            <thead>
                <tr>
                    <th>"Title"</th>
                    <th>"Done"</th>
                </tr>
            </thead>
            <tbody>
                {todos.iter().map(|todo| single_todo_row(todo))}
            </tbody>
        </table>
    )
}
