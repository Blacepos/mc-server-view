#[macro_use] extern crate rocket;

use std::net::Ipv4Addr;

use control::ControlCmd;
use dotenvy::dotenv;
use rocket::Config;

mod control;
mod attempt;
mod navigation;
mod api;
mod endpoint_helpers;
mod env;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file should exist");
    let settings = env::load_env()?;

    let (tx, rx) = rocket::tokio::sync::mpsc::channel::<ControlCmd>(5);

    let s = settings.clone();
    // Start the server control thread
    rocket::tokio::spawn(async move {
        control::control(rx, s).await;
    });

    info!("Control thread started");
    
    // Start the web server
    let _ = rocket::build()
        .configure(Config {
            address: Ipv4Addr::LOCALHOST.into(),
            port: settings.webserver_port,
            log_level: rocket::log::LogLevel::Debug,
            ..Default::default()
        })
        .manage(settings)
        .manage(tx) // Webserver can send messages to control thread
        .mount("/api", routes![api::query, api::address, api::start, api::start_get])
        .mount("/", routes![navigation::index])
        .launch()
        .await?;

    Ok(())
}

