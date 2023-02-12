use rocket::{tokio::sync::mpsc::Sender, serde::json::Json, State};

use crate::{control::ControlCmd, endpoint_helpers::{query_server, QueryResult}};

#[get("/query")]
pub async fn query(control: &State<Sender<ControlCmd>>) -> Json<QueryResult> {
    Json(query_server(control).await)
}

#[get("/address")]
pub fn address() -> String {
    std::env::var("ADDRESS_HINT").unwrap()
}

#[post("/start")]
pub async fn start(control: &State<Sender<ControlCmd>>) -> &'static str {
    match control.send(ControlCmd::StartServer).await {
        Ok(_) => "Starting server...",
        Err(_) => "Unable to communicate with control thread...",
    }
}
