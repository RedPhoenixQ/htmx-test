use std::collections::HashMap;

use axum::extract::Query;
use html_node::{
    typed::{elements::*, html},
    Node,
};

use crate::layout;

pub async fn get(Query(params): Query<HashMap<String, String>>) -> Node {
    dbg!(params);
    layout(html!((hx)
        <div>"lol as das d"</div>
    ))
}
