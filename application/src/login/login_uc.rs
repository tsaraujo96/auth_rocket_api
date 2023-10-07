use crate::login::create_token::encode_token;
use diesel::prelude::*;
use diesel::result::Error;
use rocket::form::Form;
use domain::constants::EXPIRATION_TOKEN;
use domain::models::{LoginError, NewUser, Users};
use infrastructure::establish_postgresql_connection;
use serde_json::{json, Value};

use crate::private::JWT_SECRET;

pub fn login_user(user_request: Form<NewUser>) -> Value {
    let request: NewUser = user_request.into_inner();

    match match_user(request) {
        Ok(Some(_request)) => match validate_token(_request.id) {
            LoginError::Ok(token) => json!(token),
            _ => json!({"error": "Unknown"}),
        },
        Ok(None) => json!({"error": "UserNotFound"}),
        Err(err) => {
            let error_response = json!({
                "error": "Database error",
                "message": format!("{}", err),
            });
            error_response
        }
    }
}

fn match_user(mut request: NewUser) -> Result<Option<Users>, Error> {
    use domain::schema::users;

    request.gen_password();

    let user_result = users::table
        .select(users::all_columns)
        .filter(users::email.eq(&request.email))
        .filter(users::password.eq(&request.password))
        .first::<Users>(&mut establish_postgresql_connection())
        .optional();

    user_result
}

fn validate_token(id: i32) -> LoginError {
    {
        match encode_token(
            id,
            JWT_SECRET,
            EXPIRATION_TOKEN,
        ) {
            Ok(token) => LoginError::Ok(token),
            Err(_) => LoginError::Unknown,
        }
    }
}
