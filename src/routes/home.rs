use std::collections::HashMap;

use axum::extract::Query;

pub async fn get(Query(params): Query<HashMap<String, String>>) -> String {
    dbg!(params);
    String::from("<div> lol as das d</div>")
}
