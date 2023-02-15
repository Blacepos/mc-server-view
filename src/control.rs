// Control thread

use std::{process::Child, time::Instant};
use std::time::Duration;
use std::path::Path;
use mc_query::{rcon::RconClient, status::StatusResponse};
use rocket::tokio::{sync::{mpsc, oneshot}, time::timeout};
use thiserror::Error;

use crate::attempt::{self, attempt};

const IDLE_QUERY_PERIOD_SEC: u64 = 30;

#[derive(Debug)]
pub enum ControlCmd {
    StartServer,
    StopServer,
    Query(oneshot::Sender<Option<StatusResponse>>) // I love this.
}

pub async fn control(mut msg: mpsc::Receiver<ControlCmd>) {
    
    use ControlCmd::*;
    
    loop {
        // Thread idle (mc server offline)
        let (mut mc_server, mut rcon_client) = loop {
            match msg.recv().await {
                Some(StartServer) => {
                    if let Ok((mc, rc)) = start_server().await {
                        break (mc, rc);
                    }
                },
                // Note: if Query is sent, tx will be immediately dropped, so rx won't block the webserver
                Some(other) => println!("Web server sent a message other than \"StartServer\": {other:?}"),
                None => println!("Error while awaiting web server message"),
            }
        };

        let idle_timeout = std::env::var("MINECRAFT_IDLE_TIMEOUT").unwrap().parse().unwrap();
        let idle_timeout = Duration::from_secs(idle_timeout);
        let mut idle_begin = Instant::now();

        // Thread active (mc server online)
        loop {
            // Check for messages from the webserver
            match timeout(Duration::from_secs(IDLE_QUERY_PERIOD_SEC), msg.recv()).await {
                Ok(Some(StopServer)) => {
                    try_stop_server(&mut mc_server, &mut rcon_client).await;
                    break;
                },
                Ok(Some(Query(webserver_tx))) => {
                    // query minecraft server and tell webserver result
                    let response = query_server().await.ok();

                    if webserver_tx.send(response).is_err() {
                        println!("Webserver did not get the status (receiver hung up)");
                    }
                },
                Ok(Some(_)) => {},
                Ok(None) => {},
                Err(_) => {}, // No messages
            }

            // query number of players; if > 0, reset timer. if timer > timeout, stop server.
            // also, if query fails, we must close the server because we don't want it running
            // indefinitely
            match query_server().await {
                Ok(status) => {
                    println!("Queried Minecraft, got status {status:?}");
                    
                    // reset the idle timer if there are players online
                    if status.players.online > 0 {
                        idle_begin = Instant::now();
                    }

                    // Stop the server if it has been idle for too long
                    if Instant::now() - idle_begin > idle_timeout {
                        try_stop_server(&mut mc_server, &mut rcon_client).await;
                        break;
                    }
                },
                Err(_) => {
                    try_stop_server(&mut mc_server, &mut rcon_client).await;
                    break;
                },
            }
        }
    }
}

#[derive(Error, Debug)]
enum StartServerError {
    #[error("Unable to spawn Minecraft as a child process")]
    ProcessStart,
    #[error("Unable to connect RCON to the Minecraft server")]
    RconConnect,
    #[error("Unable to authenticate RCON with the Minecraft server")]
    RconAuth,
}

async fn start_server() -> Result<(Child, RconClient), StartServerError> {
    
    use StartServerError::*;

    let server_path = std::env::var("SERVER_PATH").unwrap(); //
    let run_command = std::env::var("RUN_COMMAND").unwrap(); // nice
    let rcon_pass = std::env::var("RCON_PASSWORD").unwrap(); //

    let rcon_port: u16 = std::env::var("RCON_PORT").unwrap().parse().unwrap(); // yuck


    let path = Path::new(&server_path).join(run_command);

    // Attempt to execute "run.sh" on the server located at SERVER_PATH
    let Ok(mut child) = std::process::Command::new(&path).current_dir(&server_path).spawn() else {
        println!("Could not execute run.sh. Does the server have one?");

        return Err(ProcessStart);
    };

    // Attempt to get an RCON handle on the server
    let Ok(mut rcon_client) = attempt(
        attempt::Method::Timeout(Duration::from_secs(10)),
        || RconClient::new("localhost", rcon_port)
    ).await else {
        println!("Killing the Minecraft server: could not start RCON!");
        if child.kill().is_err() {
            println!("Minecraft server was already dead.");
        }
        
        return Err(RconConnect);
    };

    // Attempt to authenticate the RCON client
    if rcon_client.authenticate(&rcon_pass).await.is_err() {
        println!("Killing the Minecraft server: couldn't authenticate RCON!");
        if child.kill().is_err() {
            println!("Minecraft server was already dead.");
        }

        return Err(RconAuth);
    }

    Ok((child, rcon_client))
}

#[derive(Error, Debug)]
enum StopServerError {
    #[error("Unable to kill the Minecraft process (it's probably already dead)")]
    ProcessKill,
}

async fn stop_server(mc_server: &mut Child, rcon: &mut RconClient) -> Result<(), StopServerError> {
    
    use StopServerError::*;
    
    // Try gracefully shutting down the server with rcon
    if rcon.run_command("stop").await.is_err() {
        // if rcon fails, try to kill the process instead
        mc_server.kill().map_err(|_| ProcessKill)?;
    }

    Ok(())
}

async fn try_stop_server(mc_server: &mut Child, rcon_client: &mut RconClient) {
    if let Err(e) = stop_server(mc_server, rcon_client).await {
        println!("There was a problem shutting down Minecraft: {e}");
    }
}

async fn query_server() -> std::io::Result<StatusResponse> {

    let port = std::env::var("QUERY_PORT").unwrap().parse().unwrap();

    mc_query::status("localhost", port).await
}