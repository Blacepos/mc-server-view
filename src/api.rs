
use rocket::Shutdown;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast;
use rocket::State;
use rocket::serde::json::Json;
use rocket::tokio::sync::mpsc::Sender;
use rocket::tokio::time::Duration;
use rocket::response::stream::{Event, EventStream};

use crate::control::ControlEvent;
use crate::{control::ControlCmd, endpoint_helpers::{query_server, QueryResult, await_events}};

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

#[get("/events")]
pub async fn events(
    event_sub: &State<broadcast::Sender<ControlEvent>>,
    mut shutdown: Shutdown
) -> EventStream![Event + '_] {
    
    const HEARTBEAT_INTERVAL_SEC: Duration = Duration::from_secs(1);
    let mut events = event_sub.subscribe();
    
    EventStream! {
        loop {
            select! {
                evt = await_events(&mut events) => {
                    if let Some(evt) = evt {
                        yield evt;
                    }
                }
                _ = &mut shutdown => {
                    yield Event::empty().event("goodbye");
                    break;
                }
            }
            
        }
    }
}