use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

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
pub struct Todos(Arc<Mutex<HashMap<u32, Todo>>>);

impl Todos {
    fn new(todos: HashMap<u32, Todo>) -> Self {
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
    let mut todos = HashMap::new();
    todos.insert(
        0,
        Todo {
            id: 0,
            title: "Test title".to_string(),
            done: false,
        },
    );
    todos.insert(
        1,
        Todo {
            id: 1,
            title: "Maybe tit".to_string(),
            done: false,
        },
    );
    todos.insert(
        2,
        Todo {
            id: 2,
            title: "To be done".to_string(),
            done: true,
        },
    );
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
    let id = todos_locked.keys().max().unwrap_or(&0) + 1;
    let todo = Todo {
        id,
        title: form.title,
        done: false,
    };
    let row = single_todo_row(&todo);
    let form = todo_form();

    todos_locked.insert(id, todo);
    html!((hx)
        {form}
        {row}
    )
}

async fn check_todo(
    State(todos): State<Todos>,
    htmx: Htmx,
    Form(form): Form<Check>,
) -> Result<Node, ()> {
    dbg!("Is htmx request", htmx, &form);
    if let Some(todo) = todos.0.lock().unwrap().get_mut(&form.id) {
        todo.done = form.checked.is_some();
        Ok(single_todo_row(&todo))
    } else {
        Err(())
    }
}

async fn remove_todo(State(todos): State<Todos>, htmx: Htmx, Form(id): Form<Id>) {
    dbg!("Is htmx request", htmx, &id);
    todos.0.lock().unwrap().remove_entry(&id.id);
    dbg!(todos);
}

async fn get_todos(htmx: Htmx, State(todos): State<Todos>) -> Node {
    dbg!("Is htmx request", &htmx);
    let table = todos_table(todos.0.as_ref().lock().unwrap().values());
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
        <form id="todo-form" hx-post="/todo" hx-target="next table tbody" hx-swap="afterbegin" hx-swap-oob="true">
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
