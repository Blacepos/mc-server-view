#[macro_use] extern crate rocket;

use dotenvy::dotenv;

mod control;
mod attempt;
mod navigation;
mod api;
mod endpoint_helpers;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file should exist");
    ensure_env()?;
    
    let (tx, rx) = rocket::tokio::sync::mpsc::channel(5);

    // Start the server control "thread"
    rocket::tokio::spawn(async move {
        control::control(rx).await;
    });

    println!("Control Started.");
    
    // Start the web server
    let _ = rocket::build()
        .manage(tx) // Webserver can send messages to control thread
        .mount("/api", routes![api::query, api::address, api::start])
        .mount("/", routes![navigation::index])
        .launch()
        .await?;

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
