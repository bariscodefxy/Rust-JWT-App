use crate::Logs;
use rocket_db_pools::Connection;
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::status;
use serde::Deserialize;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Deserialize)]
pub struct User {
    username: String,
    password: String,
}

#[post("/register", format = "json", data = "<user>")]
pub async fn handler(mut db: Connection<Logs>, user: Json<User>) -> Result<status::Custom<&'static str>, status::Custom<&'static str>> {
    let hashed_password = match hash(&user.password, DEFAULT_COST) {
        Ok(hp) => hp,
        Err(_) => return Err(status::Custom(Status::InternalServerError, "Failed to hash password")),
    };

    let result = sqlx::query!(
        "INSERT INTO users (username, password) VALUES (?, ?)",
        user.username,
        hashed_password
    )
    .execute(&mut **db)
    .await;

    match result {
        Ok(_) => Ok(status::Custom(Status::Created, "User registered successfully")),
        Err(_) => Err(status::Custom(Status::InternalServerError, "Failed to register user")),
    }
}