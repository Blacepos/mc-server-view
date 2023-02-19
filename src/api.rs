
use rocket::Shutdown;
use rocket::serde::json::serde_json::json;
use rocket::tokio::select;
use rocket::tokio::sync::broadcast;
use rocket::State;
use rocket::serde::json;
use rocket::tokio::sync::mpsc::Sender;
use rocket::response::stream::{Event, EventStream};

use crate::control::{ControlEvent, ControlCmd};
use crate::endpoint_helpers::get_last_event;
use crate::{endpoint_helpers::{query_server, await_events}};

#[get("/query")]
pub async fn query(control: &State<Sender<ControlCmd>>) -> json::Value {
    let status = query_server(control).await;

    json!({
        "ok": status.is_some(),
        "status": status
    })
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

#[get("/last-event")]
pub async fn last_event(control: &State<Sender<ControlCmd>>) -> json::Value {
    let event = get_last_event(control).await.map(|s| s.to_event_name());
    
    json!({
        "ok": event.is_some(),
        "last_event": event
    })
}