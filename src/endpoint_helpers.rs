use std::time::Duration;

use mc_query::status::StatusResponse;
use rocket::response::stream::Event;
use rocket::tokio::{sync::{broadcast::Receiver, mpsc::Sender}, time::timeout};
use rocket::serde::Serialize;
use rocket::State;

use crate::control::{ControlCmd, ControlEvent};

/// Listens on the control thread's broadcast channel and converts messages to SSE events
pub async fn await_events(events: &mut Receiver<ControlEvent>) -> Option<Event> {
    use ControlEvent::*;
    let Ok(evt) = events.recv().await else {
        error!("Control thread seems to have dropped its sender");
        return None;
    };
    Some(match evt {
        Started => Event::empty().event("minecraft_online"),
        Stopped => Event::empty().event("minecraft_offline"),
        Empty => Event::empty().event("minecraft_empty"),
        Occupied => Event::empty().event("minecraft_occupied"),
    })
}

// async fn check_server_online(control: &State<Sender<ControlCmd>>) -> bool {
    
//     use QueryResult::*;
    
//     match query_server(control).await {
//         Success { status: _ } => true,
//         Failure { message: _ } => false
//     }
// }

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
