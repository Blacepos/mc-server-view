// Control thread

use std::process::Child;
use std::time::Duration;
use std::path::Path;
use mc_query::rcon::RconClient;
use rocket::tokio::sync::mpsc::Receiver;
use thiserror::Error;

use crate::attempt::{self, attempt};

#[derive(Debug)]
pub enum ControlCmd {
    StartServer,
    StopServer
}

pub async fn control(mut msg: Receiver<ControlCmd>) {
    
    use ControlCmd::*;
    
    loop {
        // Idle thread (mc server offline)
        let (mut mc_server, mut rcon_client) = loop {
            match msg.recv().await {
                Some(StartServer) => {
                    if let Ok((mc, rc)) = start_server().await {
                        break (mc, rc);
                    }
                },
                Some(other) => println!("Web server sent a message other than \"StartServer\": {other:?}"),
                None => println!("Error while awaiting web server message"),
            }
        };

        // Active thread (mc server online)
        loop {
            match msg.recv().await {
                Some(StopServer) => {

                    // try loop Rcon 5 times until connected

                    // let mut x = RconClient::new("localhost", 25565).await
                    //     .unwrap();
                    
                    // match x.authenticate("admin").await {
                    //     Ok(_) => {},
                    //     Err(RconProtocolError::AuthFailed) => {},
                    // };
                },
                _ => {},
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
    // What's really happening here:
    // 1. Spawn the server
    // 2. Connect the RCON client
    // 3. Authenticate the RCON client as admin

    // Is it possible to not be so verbose?
    
    use StartServerError::*;

    let server_path = std::env::var("SERVER_PATH").unwrap(); //
    let run_command = std::env::var("RUN_COMMAND").unwrap(); // nice
    let rcon_pass = std::env::var("RCON_PASSWORD").unwrap(); //

    // Attempt to execute "run.sh" on the server located at SERVER_PATH
    if let Ok(mut child) = std::process::Command::new(
        Path::new(&server_path).join(run_command)
    ).spawn() {

        // Attempt to get an RCON handle on the server
        if let Ok(mut rcon_client) = attempt(
            attempt::Method::Timeout(Duration::from_secs(10)),
            || RconClient::new("localhost", 25565)
        ).await {

            // Attempt to authenticate the RCON client
            if rcon_client.authenticate(&rcon_pass).await.is_ok() {

                Ok((child, rcon_client))
            } else {
                println!("Killing the Minecraft server: couldn't authenticate RCON!");
                if child.kill().is_err() {
                    println!("Minecraft server was already dead.");
                }

                Err(RconAuth)
            }

        } else {
            println!("Killing the Minecraft server: could not start RCON!");
            if child.kill().is_err() {
                println!("Minecraft server was already dead.");
            }
            Err(RconConnect)
        }

    } else {
        println!("Could not execute run.sh. Does the server have one?");

        Err(ProcessStart)
    }
}
