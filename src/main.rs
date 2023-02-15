#[macro_use] extern crate rocket;

use std::net::Ipv4Addr;

use dotenvy::dotenv;
use rocket::Config;

mod control;
mod attempt;
mod navigation;
mod api;
mod endpoint_helpers;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file should exist");
    ensure_env()?;
    
    let server_path = std::env::var("SERVER_PATH").unwrap(); //
    let run_command = std::env::var("RUN_COMMAND").unwrap(); // nice
    let path = std::path::Path::new(&server_path).join(run_command);

    if let Ok(mut child) = std::process::Command::new(&path).current_dir(&server_path).spawn().unwrap() {
        println!("Could not execute run.sh. Does the server have one?");
    };

    // let (tx, rx) = rocket::tokio::sync::mpsc::channel(5);

    // // Start the server control "thread"
    // rocket::tokio::spawn(async move {
    //     control::control(rx).await;
    // });

    // println!("Control Started.");
    
    // // Start the web server
    // let _ = rocket::build()
    //     .configure(Config {
    //         address: Ipv4Addr::UNSPECIFIED.into(),
    //         port: 3000,
    //         ..Default::default()
    //     })
    //     .manage(tx) // Webserver can send messages to control thread
    //     .mount("/api", routes![api::query, api::address, api::start])
    //     .mount("/", routes![navigation::index])
    //     .launch()
    //     .await?;

    Ok(())
}

/// Ensure all environment variables are present and that the non-string values are parsable.
/// This is so we can safely unwrap when retrieving them later.
fn ensure_env() -> Result<(), Box<dyn std::error::Error>> {

    std::env::var("SERVER_PATH")?;
    std::env::var("RUN_COMMAND")?;
    std::env::var("RCON_PASSWORD")?;
    std::env::var("WEBSERVER_PORT")?.parse::<u16>()?;
    std::env::var("MINECRAFT_PORT")?.parse::<u16>()?;
    std::env::var("RCON_PORT")?.parse::<u16>()?;
    std::env::var("QUERY_PORT")?.parse::<u16>()?;
    std::env::var("MINECRAFT_IDLE_TIMEOUT")?.parse::<u64>()?; // Duration::from_secs accepts u64
    std::env::var("ADDRESS_HINT")?;

    Ok(())
}
