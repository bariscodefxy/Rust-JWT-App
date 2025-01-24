use crate::Logs;
use rocket_db_pools::Connection;
use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::response::status;
use serde::Deserialize;
use jsonwebtoken::{encode, Header, EncodingKey};
use bcrypt::{verify, hash};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(serde::Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[post("/login", format = "json", data = "<login_request>")]
pub async fn handler(mut db: Connection<Logs>, login_request: Json<LoginRequest>) -> Result<status::Custom<String>, status::Custom<&'static str>> {
    let user = sqlx::query!(
        "SELECT username, password FROM users WHERE username = ?",
        login_request.username
    )
    .fetch_one(&mut **db)
    .await;

    match user {
        Ok(record) => {
            if verify(&login_request.password, &record.password).unwrap() {
                let expiration = chrono::Utc::now()
                    .checked_add_signed(chrono::Duration::seconds(60))
                    .expect("valid timestamp")
                    .timestamp();

                let claims = Claims {
                    sub: record.username,
                    exp: expiration as usize,
                };

                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();

                Ok(status::Custom(Status::Ok, token))
            } else {
                Err(status::Custom(Status::Unauthorized, "Invalid credentials"))
            }
        },
        Err(_) => Err(status::Custom(Status::Unauthorized, "Invalid credentials")),
    }
}