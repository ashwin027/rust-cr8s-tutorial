use chrono::NaiveDateTime;
use diesel::prelude::*;
use rocket::serde;
use crate::schema::*;

#[derive(Queryable, serde::Serialize, serde::Deserialize)]
pub struct Rustacean {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime
}

#[derive(Insertable, serde::Deserialize)]
#[diesel(table_name=rustaceans)]
pub struct NewRustacean {
    name: String,
    email: String
}

#[derive(Queryable, serde::Serialize, serde::Deserialize)]
pub struct Crate {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime
}

#[derive(Insertable, serde::Deserialize)]
#[diesel(table_name=crates)]
pub struct NewCrate{
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}