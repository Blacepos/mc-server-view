
use std::time::Duration;
use dotenvy::dotenv;


mod control;
mod attempt;

fn main() {
    dotenv().expect(".env file should exist");
    
    let (tx, rx) = rocket::tokio::sync::mpsc::channel(5);

    rocket::tokio::spawn(async move {
        control::control(rx).await;
    });

    println!("Control Started. Will exit in 10 seconds.");
    std::thread::sleep(Duration::from_secs(10));
}
