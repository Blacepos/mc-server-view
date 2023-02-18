#[macro_use] extern crate rocket;

use std::net::Ipv4Addr;

use dotenvy::dotenv;
use rocket::{Config, fairing::{Fairing, Kind, Info}, Request, http::Header, Response, tokio::sync};

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

    let (cmd_tx, cmd_rx) = sync::mpsc::channel(5);
    let (ev_tx, _) = sync::broadcast::channel(5);
    // should   ^this receiver be dropped?
    let s = settings.clone();
    let evt_sub = ev_tx.clone();
    // Start the server control thread
    rocket::tokio::spawn(async move {
        control::control(cmd_rx, ev_tx, s).await;
    });

    info!("Control thread started");
    
    // Start the web server
    let _ = rocket::build()
        .configure(Config {
            address: Ipv4Addr::UNSPECIFIED.into(),
            port: settings.webserver_port,
            log_level: rocket::log::LogLevel::Normal,
            ..Default::default()
        })
        .manage(settings)
        .manage(cmd_tx) // Webserver can send messages to control thread
        .manage(evt_sub) // /events can await signals from control thread. This is broadcast, so tx is needed to make new subscribers
        .attach(Cors)
        .mount("/api", routes![
            api::query,
            api::address,
            api::start,
            api::start_get,
            api::events,
            api::last_event
        ])
        .mount("/", routes![navigation::index])
        .launch()
        .await?;

    Ok(())
}

struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Configuration",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET",
        ));
        // response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        // response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}