use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use crate::auth::{self, Credentials};
use crate::rocket_routes::server_error;
use crate::{repositories::UserRepository, rocket_routes::DbConn};
use rocket::serde::json::Value;
use rocket::serde::json::json;

#[rocket::post("/login", format = "json", data= "<credentials>")]
pub async fn login(mut db: Connection<DbConn>, credentials: Json<Credentials>,) -> Result<Value, Custom<Value>> {
    UserRepository::find_by_username(&mut db, &credentials.username)
        .await
        .map(|user| {
            if let Ok(token) = auth::authorize_user(&user, credentials.into_inner()) {
                return json!(token);
            }
            json!("Unauthorized")
        })
        .map_err(|e| server_error(e.into()))
}