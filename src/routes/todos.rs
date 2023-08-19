use std::sync::{Arc, Mutex};

use axum::{extract::State, routing::*, Form, Router};
use html_node::{
    text,
    typed::{elements::*, html},
    Node,
};
use serde::Deserialize;

use crate::{layout, Htmx};

#[derive(Debug)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Clone)]
pub struct Todos(Arc<Mutex<Vec<Todo>>>);

impl Todos {
    fn new(todos: Vec<Todo>) -> Self {
        Self(Arc::new(Mutex::new(todos)))
    }
}

#[derive(Deserialize)]
struct TodoForm {
    title: String,
}

#[derive(Debug, Deserialize)]
struct Id {
    id: u32,
}

#[derive(Debug, Deserialize)]
struct Check {
    id: u32,
    checked: Option<String>,
}

pub fn todos_router() -> Router {
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
    Router::new()
        .route(
            "/todo",
            get(get_todos)
                .delete(remove_todo)
                .post(add_todo)
                .patch(check_todo),
        )
        .route("/todo/form", get(|| async { todo_form() }))
        .with_state(Todos::new(todos))
}

async fn add_todo(State(todos): State<Todos>, Form(form): Form<TodoForm>) -> Node {
    dbg!("Is htmx request");
    dbg!(&todos);

    let mut todos_locked = todos.0.lock().unwrap();
    let id = todos_locked.iter().map(|t| t.id).max().unwrap_or(0) + 1;
    let todo = Todo {
        id,
        title: form.title,
        done: false,
    };
    let row = single_todo_row(&todo);

    todos_locked.push(todo);
    row
}

async fn check_todo(
    State(todos): State<Todos>,
    htmx: Htmx,
    Form(form): Form<Check>,
) -> Result<Node, ()> {
    dbg!("Is htmx request", htmx, &form);
    if let Some(todo) = todos.0.lock().unwrap().iter_mut().find(|t| t.id == form.id) {
        todo.done = form.checked.is_some();
        Ok(single_todo_row(&todo))
    } else {
        Err(())
    }
}

async fn remove_todo(State(todos): State<Todos>, htmx: Htmx, Form(id): Form<Id>) {
    dbg!("Is htmx request", htmx, &id);
    let mut locked_todos = todos.0.lock().unwrap();
    if let Some((i, _)) = locked_todos.iter().enumerate().find(|(_, t)| t.id == id.id) {
        locked_todos.remove(i);
    }
    dbg!(&locked_todos);
}

async fn get_todos(htmx: Htmx, State(todos): State<Todos>) -> Node {
    dbg!("Is htmx request", &htmx);
    let table = todos_table(todos.0.lock().unwrap().iter());
    let form = todo_form();

    let node = html!((hx)
        <div>
            {form}
            {table}
        </div>
    );

    if htmx.0 {
        node
    } else {
        layout(node)
    }
}

fn todo_form() -> Node {
    html!((hx)
        <form id="todo-form"
            hx-post="/todo"
            hx-target="next table tbody"
            hx-swap="afterbegin"
            hx-on="htmx:afterRequest: if (!event?.detail?.failed) this.reset()"
        >
            <label>
                <input type="text" name="title" />
            </label>
            <button>Add</button>
        </form>
    )
}

fn single_todo_row(todo: &Todo) -> Node {
    let mut checkbox = html!((hx)
        <input
            class="checkbox"
            type="checkbox"
            name="checked"
            hx-patch="/todo"
            hx-swap="outerHTML"
            hx-target="closest tr"
            hx-vals={format!("\"id\": {}", todo.id)}
        />
    );

    if todo.done {
        checkbox = if let Node::Element(mut el) = checkbox {
            el.attributes
                .push(("checked".to_string(), Some(String::from("true"))));
            Node::Element(el)
        } else {
            checkbox
        };
    }

    html!((hx)
        <tr>
            <td>{text!("{}", todo.title)}</td>
            <td>{checkbox}</td>
            <td>
                <button hx-delete="/todo" hx-target="closest tr" hx-swap="outerHTML" hx-vals={format!("\"id\": {}", todo.id)}>X</button>
            </td>
        </tr>
    )
}

fn todos_table<'a>(todos: impl Iterator<Item = &'a Todo>) -> Node {
    html!((hx)
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
