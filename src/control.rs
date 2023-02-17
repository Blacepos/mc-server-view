// Control thread

use std::{process::Child, time::Instant};
use std::time::Duration;
use std::path::Path;
use mc_query::{rcon::RconClient, status::StatusResponse};
use rocket::tokio::{sync::{mpsc, oneshot, broadcast}, time::timeout};
use thiserror::Error;

use crate::attempt::{self, attempt};
use crate::env::Env;

const IDLE_QUERY_PERIOD_SEC: u64 = 30;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ControlCmd {
    StartServer,
    StopServer,
    Query(oneshot::Sender<Option<StatusResponse>>) // I love this.
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ControlEvent {
    Started,
    Starting,
    Stopped,
    Crashed,
    Empty,
    Occupied
}

pub async fn control(mut msg: mpsc::Receiver<ControlCmd>, mut evt: broadcast::Sender<ControlEvent>, settings: Env) {

    use ControlEvent::*;

    loop {
        // Thread idle (mc server offline)
        let (mc_server, rcon_client) = thread_idle(&mut msg, &mut evt, &settings).await;

        if evt.send(Started).is_err() {
            error!("Webserver dropped receiver while Minecraft was starting");
        }

        // Thread active (mc server online)
        thread_active(&mut msg, &mut evt, &settings, mc_server, rcon_client).await;

        if evt.send(Stopped).is_err() {
            error!("Webserver dropped receiver while Minecraft was stopping");
        }
    }
}

/// Patiently wait for commands from the webserver. Upon being told to start Minecraft, spawns the 
/// process and returns a process handle and RCON client.
async fn thread_idle(
    msg: &mut mpsc::Receiver<ControlCmd>,
    evt: &mut broadcast::Sender<ControlEvent>,
    settings: &Env
) -> (Child, RconClient) {
    
    use ControlCmd::*;
    use ControlEvent::*;

    loop {
        match msg.recv().await {
            Some(StartServer) => {
                // send a messsage to the end-users listening on /events
                evt.send(Starting);
                
                if let Ok((mc, rc)) = start_server(settings).await {
                    break (mc, rc);
                }
            },
            // Note: if Query is sent, tx will be immediately dropped, so rx won't block the webserver
            Some(Query(_)) => info!("Received a query, but the server wasn't online"),
            Some(other) => warn!("Webserver sent a message other than \"StartServer\": {other:?}"),
            None => error!("Webserver dropped sender"),
        }
    }
}

/// Await webserver commands and ping Minecraft periodically.
/// 
/// Stops Minecraft if:
/// - Webserver says to shut down Minecraft
/// - Periodic pings return 0 online players for longer than MINECRAFT_IDLE_TIMEOUT
/// - A single ping fails for whatever reason
async fn thread_active(
    msg: &mut mpsc::Receiver<ControlCmd>,
    evt: &mut broadcast::Sender<ControlEvent>,
    settings: &Env,
    mut mc_server: Child,
    mut rcon_client: RconClient
) {
    
    use ControlCmd::*;
    use ControlEvent::*;

    let idle_timeout = Duration::from_secs(settings.minecraft_idle_timeout);
    let mut idle_begin = Instant::now();
    let mut is_empty = true;
    
    loop {
        // Check for messages from the webserver
        match timeout(Duration::from_secs(IDLE_QUERY_PERIOD_SEC), msg.recv()).await {
            Ok(Some(StopServer)) => {
                try_stop_server(&mut mc_server, &mut rcon_client).await;
                break;
            },
            Ok(Some(Query(webserver_tx))) => {
                // query minecraft server and tell webserver result
                let response = mc_query::status("localhost", settings.minecraft_port).await
                    .ok();

                if webserver_tx.send(response).is_err() {
                    error!("Webserver did not get the status (receiver hung up)");
                }
            },
            Ok(Some(other)) => warn!("Webserver sent an invalid message: {other:?}"),
            Ok(None) => {
                error!("Webserver dropped sender while Minecraft was online, forcing Minecraft to close");

                try_stop_server(&mut mc_server, &mut rcon_client).await;

                // send a messsage to the end-users listening on /events
                evt.send(Crashed);
            },
            Err(_) => {}, // No messages
        }

        // Query number of players: if > 0, reset timer; if timer > timeout, stop server.
        // Also, if query fails, we must close the server because we don't want it running
        // indefinitely.
        match mc_query::status("localhost", settings.minecraft_port).await {
            Ok(status) => {
                debug!("Queried Minecraft, got status {status:?}");
                

                // reset the idle timer if there are players online
                if status.players.online > 0 {
                    idle_begin = Instant::now();

                    // emit event if server was previously not empty
                    if !is_empty {
                        is_empty = true;
                        if evt.send(Occupied).is_err() {
                            error!("Webserver dropped receiver while Minecraft was online");
                        }
                    }
                // emit event if server was previously empty
                } else if is_empty {
                    is_empty = false;
                    if evt.send(Empty).is_err() {
                        error!("Webserver dropped receiver while Minecraft was online");
                    }
                }


                // Stop the server if it has been idle for too long
                if Instant::now() - idle_begin > idle_timeout {
                    info!("Idle period has expired, shutting down Minecraft");
                    try_stop_server(&mut mc_server, &mut rcon_client).await;
                    break;
                }
            },
            Err(e) => {
                // If we can't get the status of the server, then it's either already dead or
                // some unknown circumstance is causing it to run unsupervised.
                // Either way, we can't pretend like the server is in a valid state, so we must make
                // sure it is dead and then go back to idle.
                warn!("Forcing server shutdown because a status query failed: {e:?}");
                try_stop_server(&mut mc_server, &mut rcon_client).await;

                // send a messsage to the end-users listening on /events
                evt.send(Crashed);
                
                break;
            },
        }
    }
}

/// Describes the ways in which starting Minecraft can fail
#[derive(Error, Debug)]
enum StartServerError {
    #[error("Unable to spawn Minecraft as a child process")]
    ProcessStart,
    #[error("Unable to connect RCON to the Minecraft server")]
    RconConnect,
    #[error("Unable to authenticate RCON with the Minecraft server")]
    RconAuth,
}

/// Spawn the Minecraft instance and connect with RCON
async fn start_server(settings: &Env) -> Result<(Child, RconClient), StartServerError> {
    
    use StartServerError::*;

    let path = Path::new(&settings.server_path).join(&settings.run_command);

    // Attempt to execute "run.sh" on the server located at SERVER_PATH
    let Ok(mut child) = std::process::Command::new(&path)
        .current_dir(&settings.server_path)
        .spawn() else {

        error!("Could not execute run.sh. Does the server have one?");

        return Err(ProcessStart);
    };

    // Attempt to get an RCON handle on the server
    let Ok(mut rcon_client) = attempt(
        attempt::Method::Timeout(Duration::from_secs(10)),
        || RconClient::new("localhost", settings.rcon_port)
    ).await else {
        error!("Killing the Minecraft server: could not start RCON!");
        if child.kill().is_err() {
            warn!("Minecraft server was already dead.");
        }
        
        return Err(RconConnect);
    };

    // Attempt to authenticate the RCON client
    if rcon_client.authenticate(&settings.rcon_password).await.is_err() {
        error!("Killing the Minecraft server: could not authenticate RCON!");
        if child.kill().is_err() {
            warn!("Minecraft server was already dead.");
        }

        return Err(RconAuth);
    }

    Ok((child, rcon_client))
}

/// Describes the ways in which stopping Minecraft can fail
#[derive(Error, Debug)]
enum StopServerError {
    #[error("Unable to kill the Minecraft process (it's probably already dead)")]
    ProcessKill,
}

async fn stop_server(mc_server: &mut Child, rcon: &mut RconClient) -> Result<(), StopServerError> {
    
    use StopServerError::*;

    info!("Stopping server");
    
    // Try gracefully shutting down the server with rcon
    if rcon.run_command("stop").await.is_err() {
        // if rcon fails, try to kill the process instead
        mc_server.kill().map_err(|_| ProcessKill)?;
    }

    Ok(())
}

async fn try_stop_server(mc_server: &mut Child, rcon_client: &mut RconClient) {
    if let Err(e) = stop_server(mc_server, rcon_client).await {
        error!("There was a problem shutting down Minecraft: {e}");
    }
}
