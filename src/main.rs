#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};
use rocket_db_pools::{Database, sqlx};
use tracing_subscriber::fmt::Subscriber;

pub mod endpoints;

#[derive(Database)]
#[database("sqlite_logs")]
pub struct Logs(sqlx::SqlitePool);

fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(Logs::init())
        .mount("/", routes![
            endpoints::index::handler,
            endpoints::login::handler,
            endpoints::register::handler,
        ])
}

#[tokio::main]
async fn main() -> Result<(), rocket::Error> {
    // Initialize logging
    Subscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting Rocket server...");

    match rocket().launch().await {
        Ok(_) => {
            tracing::info!("🚀 Rocket has launched successfully!");
            Ok(())
        },
        Err(e) => {
            tracing::error!("Failed to launch Rocket: {:?}", e);
            Err(e)
        }
    }
}