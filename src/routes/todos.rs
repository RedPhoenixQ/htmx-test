use axum::{routing::*, Form, Router};
use html_node::{
    text,
    typed::{elements::*, html},
    Node,
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::{layout, AppError, Htmx, DB};

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
                hx-swap="outerHTML"
                hx-target="closest tr"
                hx-vals={format!("\"id\": \"{}\"", self.id.id)}
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

#[derive(Debug, Deserialize, Serialize)]
struct NewTodo {
    title: String,
    done: bool,
}

#[derive(Deserialize)]
struct TodoForm {
    title: String,
}

#[derive(Debug, Deserialize)]
struct Id {
    id: String,
}

#[derive(Debug, Deserialize)]
struct Check {
    id: String,
    checked: Option<String>,
}

#[derive(Debug, Serialize)]
struct Done {
    done: bool,
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
        .content(NewTodo {
            title: form.title,
            done: false,
        })
        .await?;

    Ok(todo.into())
}1

async fn check_todo(Form(form): Form<Check>) -> Result<Node, AppError> {
    let todo: Todo = DB
        .update(("todo", form.id))
        .merge(Done {
            done: form.checked.is_some(),
        })
        .await?;

    dbg!(&todo);
    
    Ok(todo.into())
}

async fn remove_todo(Form(id): Form<Id>) -> Result<(), AppError> {
    let _todo: Option<Todo> = DB.delete(("todo", id.id)).await?;
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
