use crate::schema::users;
use crate::tokens::Token;
use diesel::prelude::*;
use rocket::serde::json::{Value};
use rocket::serde::{Deserialize, Serialize};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use rocket::FromForm;
use utils::hash_password::hash_password;

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Users {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

impl NewUser {
    pub fn gen_password(&mut self) {
        let salt: String = hash_password(&self.password);
        self.password = salt;
    }
}


pub enum LoginError {
    Ok(Token),
    UserNotFound,
    Unknown,
    DatabaseError(Value),
}
