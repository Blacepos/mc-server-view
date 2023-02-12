use std::time::Duration;

use mc_query::status::StatusResponse;
use rocket::{State, tokio::{sync::mpsc::Sender, time::timeout}, serde::Serialize};

use crate::control::ControlCmd;

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
