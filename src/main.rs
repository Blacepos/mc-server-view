#[macro_use] extern crate rocket;

use std::net::Ipv4Addr;

use control::ControlCmd;
use dotenvy::dotenv;
use rocket::{Config, fairing::{Fairing, Kind, Info}, Request, http::Header, Response};

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

    let cors = rocket_cors::CorsOptions::default()
        .allowed_origins(rocket_cors::AllowedOrigins::All);

    cors.validate()?;
    
    // Start the web server
    let _ = rocket::build()
        .configure(Config {
            address: Ipv4Addr::UNSPECIFIED.into(),
            port: settings.webserver_port,
            log_level: rocket::log::LogLevel::Debug,
            ..Default::default()
        })
        .manage(settings)
        .manage(tx) // Webserver can send messages to control thread
        .attach(Cors)
        .mount("/api", routes![api::query, api::address, api::start, api::start_get])
        .mount("/", routes![navigation::index])
        .launch()
        .await?;

    Ok(())
}


pub struct Cors;

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