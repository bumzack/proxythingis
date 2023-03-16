use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::schema::person;

#[derive(Queryable, Deserialize, Serialize, Clone)]
pub struct Person {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub created: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = person)]
pub struct NewPerson<'a> {
    pub firstname: &'a str,
    pub lastname: &'a str,
}

#[derive(Deserialize, Serialize)]
pub struct NewPersonPost {
    pub firstname: String,
    pub lastname: String,
}

/// An API error serializable to JSON.
#[derive(Serialize, Debug)]
pub struct ErrorMessage {
    pub(crate) code: u16,
    pub(crate) message: String,
}
