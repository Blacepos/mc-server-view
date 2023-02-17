use std::time::Duration;

use mc_query::status::StatusResponse;
use rocket::response::stream::Event;
use rocket::tokio::{time::timeout, sync::mpsc::Sender};
use rocket::serde::Serialize;
use rocket::State;

use crate::control::ControlCmd;

pub async fn poll_server_events(control: &State<Sender<ControlCmd>>) -> Option<Event> {
    use ServerEvent::*;
    let event = if check_server_online(control).await { MinecraftOnline } else { MinecraftOffline };
    Some(event.into_sse())
}

async fn check_server_online(control: &State<Sender<ControlCmd>>) -> bool {
    
    use QueryResult::*;
    
    match query_server(control).await {
        Success { status: _ } => true,
        Failure { message: _ } => false
    }
}

/// Ask the control thread to query Minecraft
pub async fn query_server(control: &State<Sender<ControlCmd>>) -> QueryResult {

    use QueryResult::*;

    let (tx, rx) = rocket::tokio::sync::oneshot::channel();

    if control.send(ControlCmd::Query(tx)).await.is_err() {
        println!("Control thread sender dropped");
    }
    
    match timeout(Duration::from_secs(10), rx).await {
        Ok(Ok(Some(status))) => {
            Success {
                status
            }
        },
        _ => {
            Failure {
                message: "Unable to query Minecraft. Is the server online?"
            }
        },
    }
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")] // we have to tell the Serialize macro where serde is (it was re-exported by rocket)
pub enum QueryResult {
    Success {
        status: StatusResponse
    },
    Failure {
        message: &'static str
    }
}

/// Defines the different message types that the webserver can send over /events
enum ServerEvent {
    MinecraftOnline,
    MinecraftOffline,
}

impl ServerEvent {
    fn into_sse(self) -> Event {
        let name = match self {
            ServerEvent::MinecraftOnline => "minecraft_online",
            ServerEvent::MinecraftOffline => "minecraft_offline",
        };

        // This may change if some events carry data
        Event::empty().event(name)
    }
}
