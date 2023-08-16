use axohtml::{dom::DOMTree, elements::tr, html, text};
use axum::{response::Html, routing::get, Router};

struct Todo {
    id: u32,
    title: String,
    done: bool,
}

pub fn todos_router() -> Router {
    Router::new().route("/", get(get_todos))
}

async fn get_todos() -> Html<String> {
    let todos = vec![
        Todo {
            id: 0,
            title: "Test title".to_string(),
            done: false,
        },
        Todo {
            id: 1,
            title: "Maybe tit".to_string(),
            done: false,
        },
        Todo {
            id: 2,
            title: "To be done".to_string(),
            done: true,
        },
    ];

    todos_table(&todos).to_string().into()
}

fn single_todo_row(todo: &Todo) -> Box<tr<String>> {
    html!(
        <tr>
            <td>{ text!("{}", todo.title)}</td>
            <td><input class="checkbox" checked=todo.done hx-vals={format!("\"id\": {}", todo.id)} /></td>
        </tr>
    )
}

fn todos_table(todos: &Vec<Todo>) -> DOMTree<String> {
    html!(
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
