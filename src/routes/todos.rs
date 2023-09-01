use std::collections::BTreeMap;

use anyhow::anyhow;
use axum::{routing::*, Form, Router};
use html_node::{
    text,
    typed::{elements::*, html},
    Node,
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Thing, Value};

use crate::{layout, AppError, Htmx, DB};

pub const TABLE_INIT: &str = "
    DEFINE TABLE todo SCHEMAFULL;
    DEFINE FIELD title ON TABLE todo TYPE string;
    DEFINE FIELD done ON TABLE todo TYPE bool;    
";

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: Thing,
    pub title: String,
    pub done: bool,
}

impl Into<Node> for &Todo {
    fn into(self) -> Node {
        let mut checkbox = html!((hx)
            <input
                class="checkbox"
                type="checkbox"
                name="checked"
                hx-patch="/todo"
            />
        );

        if self.done {
            checkbox = if let Node::Element(mut el) = checkbox {
                el.attributes
                    .push(("checked".to_string(), Some(String::from("true"))));
                Node::Element(el)
            } else {
                checkbox
            };
        }

        html!((hx)
            <tr
                class="[&:has(input:checked)]:bg-green-500"
                id=self.id.id.to_string()
                hx-swap="outerHTML"
                hx-target="closest tr"
            >
                <td>{text!("{}", self.title)}</td>
                <td>{checkbox}</td>
                <td><button hx-delete="/todo">X</button></td>
            </tr>
        )
    }
}

impl Into<Node> for Todo {
    fn into(self) -> Node {
        (&self).into()
    }
}

#[derive(Deserialize)]
struct TodoForm {
    title: String,
}

#[derive(Debug, Deserialize)]
struct Check {
    checked: Option<String>,
}

pub fn todos_router() -> Router {
    Router::new().route(
        "/",
        get(get_todos)
            .delete(remove_todo)
            .post(create_todo)
            .patch(check_todo),
    )
}

async fn create_todo(Form(form): Form<TodoForm>) -> Result<Node, AppError> {
    let todo: Todo = DB
        .create("todo")
        .content(BTreeMap::<&str, Value>::from([
            ("title", form.title.into()),
            ("done", false.into()),
        ]))
        .await?;
    Ok(todo.into())
}

async fn check_todo(htmx: Htmx, Form(form): Form<Check>) -> Result<Node, AppError> {
    let todo: Todo = DB
        .update((
            "todo",
            htmx.target.ok_or(anyhow!("no target for todo check"))?,
        ))
        .merge(BTreeMap::from([("done", form.checked.is_some())]))
        .await?;

    dbg!(&todo);

    Ok(todo.into())
}

async fn remove_todo(htmx: Htmx) -> Result<(), AppError> {
    let id = htmx.target.ok_or(anyhow!("No target for todo delete"))?;
    let _todo: Option<Todo> = DB.delete(("todo", id)).await?;
    Ok(())
}

async fn get_todos(htmx: Htmx) -> Result<Node, AppError> {
    dbg!("Is htmx request", &htmx);
    let todos: Vec<Todo> = DB.select("todo").await?;

    let node = html!((hx)
        <div class="m-auto w-64 table-wrapper">
            <form id="todo-form"
                class="flex"
                hx-post="/todo"
                hx-target="next table tbody"
                hx-swap="beforeend"
                hx-on="htmx:afterRequest: if (!event?.detail?.failed) this.reset()"
            >
                <input class="flex-1" type="text" name="title" />
                <button>Add</button>
            </form>
            <table class="w-full [&_td,&_th]:w-full text-center">
                <thead>
                    <tr>
                        <th>Title</th>
                        <th>Done</th>
                    </tr>
                </thead>
                <tbody>
                    {todos.into_iter().map(|todo| -> Node {todo.into()})}
                </tbody>
            </table>
        </div>
    );

    Ok(if htmx.fullpage { layout(node) } else { node })
}
