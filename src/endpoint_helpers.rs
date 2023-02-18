use std::time::Duration;

use mc_query::status::StatusResponse;
use rocket::response::stream::Event;
use rocket::tokio::{sync::{broadcast, mpsc, oneshot}, time::timeout};
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
pub async fn query_server(control: &State<mpsc::Sender<ControlCmd>>) -> Option<StatusResponse> {

    let (tx, rx) = oneshot::channel();

    if control.send(ControlCmd::Query(tx)).await.is_err() {
        println!("Control thread sender dropped");
    }
    
    match timeout(Duration::from_secs(10), rx).await {
        Ok(Ok(Some(status))) => Some(status),
        _ => None,
    }
}

pub async fn get_last_event(control: &State<mpsc::Sender<ControlCmd>>) -> Option<ControlEvent> {

    let (tx, rx) = oneshot::channel();

    if control.send(ControlCmd::LastEvent(tx)).await.is_err() {
        println!("Control thread sender dropped");
    }

    match timeout(Duration::from_secs(10), rx).await {
        Ok(Ok(last_event)) => Some(last_event),
        _ => None,
    }
}

impl ControlEvent {
    pub fn to_event_name(&self) -> String {
        String::from(match self {
            ControlEvent::Started => "started",
            ControlEvent::Starting => "starting",
            ControlEvent::Stopped => "stopped",
            ControlEvent::Crashed => "crashed",
            ControlEvent::Empty => "empty",
            ControlEvent::Occupied => "occupied",
        })
    }
}