use crate::models::User;
use crate::repositories::UserRepository;
use rocket::http::Status;
use rocket::request;
use rocket::request::FromRequest;
use rocket::response::status::Custom;
use rocket::serde::json::json;
use rocket::serde::json::Value;
use rocket::Request;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::Connection;
use rocket_db_pools::Database;
use std::error::Error;
pub mod authorization;
pub mod crates;
pub mod rustaceans;
#[derive(Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);
#[derive(Database)]
#[database("redis")]
pub struct CacheConn(rocket_db_pools::deadpool_redis::Pool);

pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error."))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // Authorization: Bearer SESSION_ID_128_CHARACTERS_LONG
        let session_header = req
            .headers()
            .get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");

        if let Some(header_value) = session_header {
            let mut cache = req
                .guard::<Connection<CacheConn>>()
                .await
                .expect("Cannot connect to redis in request guard");

            let mut db = req
                .guard::<Connection<DbConn>>()
                .await
                .expect("Cannot connect to db in request guard");

            let result = cache
                .get::<String, i32>(format!("sessions/{}", header_value[1]))
                .await;
            if let Ok(user_id) = result {
                if let Ok(user) = UserRepository::find(&mut db, user_id).await {
                    return rocket::outcome::Outcome::Success(user);
                }
            }
        }

        rocket::outcome::Outcome::Error((Status::Unauthorized, ()))
    }
}
