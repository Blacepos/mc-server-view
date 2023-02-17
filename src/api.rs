
use rocket::Shutdown;
use rocket::tokio::select;
use rocket::{State, futures::StreamExt};
use rocket::serde::json::Json;
use rocket::tokio::sync::mpsc::Sender;
use rocket::tokio::time::{self, Duration};
use rocket::response::stream::{Event, EventStream};

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

#[get("/start")]
pub async fn start_get() -> &'static str {
    "Use a POST request to start the server"
}

// #[get("/events")]
// pub async fn events(
//     control: &State<Sender<ControlCmd>>,
//     mut shutdown: Shutdown
// ) -> EventStream![Event + '_] {
    
//     const PING_INTERVAL_SEC: Duration = Duration::from_secs(1);
    
//     EventStream! {
//         loop {
//             select! {
//                 _ = time::sleep(PING_INTERVAL_SEC) => {
//                     yield Event::json(&ServerOnline {
//                         is_online: check_server_online(control).await
//                     });
//                 }
//                 _ = &mut shutdown => {
//                     break
//                 }
//             }
            
//         }
//     }
// }