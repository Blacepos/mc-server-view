use std::time::Duration;

use mc_query::status::StatusResponse;
use rocket::response::stream::Event;
use rocket::tokio::{sync::{broadcast, mpsc, oneshot}, time::timeout};
use rocket::serde::Serialize;
use rocket::State;

use crate::control::{ControlCmd, ControlEvent};

/// Listens on the control thread's broadcast channel and converts messages to SSE events
pub async fn await_events(events: &mut broadcast::Receiver<ControlEvent>) -> Option<Event> {
    
    use ControlEvent::*;
    
    let Ok(evt) = events.recv().await else {
        error!("Control thread seems to have dropped its sender");
        return None;
    };
    let evt = match evt {
        Stopped  => "offline",
        Starting => "starting",
        Started  => "online",
        Empty    => "empty",
        Occupied => "occupied",
        Crashed  => "crashed",
    };

    Some(Event::empty().event(evt))
}

/// Ask the control thread to query Minecraft
pub async fn query_server(control: &State<mpsc::Sender<ControlCmd>>) -> QueryResult {

    use QueryResult::*;

    let (tx, rx) = oneshot::channel();

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

// we have to tell the Serialize macro where serde is (it was re-exported by rocket)
#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub enum QueryResult {
    Success {
        status: StatusResponse
    },
    Failure {
        message: &'static str
    }
}

pub async fn get_last_event(control: &State<mpsc::Sender<ControlCmd>>) -> ControlEvent {

    let (tx, rx) = oneshot::channel();

    if control.send(ControlCmd::LastEvent(tx)).await.is_err() {
        println!("Control thread sender dropped");
    }

    match timeout(Duration::from_secs(10), rx).await {
        Ok(Ok(last_event)) => {
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
#[serde(crate = "rocket::serde")]
pub struct LastEvent {

}
