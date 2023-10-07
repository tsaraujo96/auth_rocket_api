use diesel::prelude::*;
use domain::models::{NewUser, Users};
use infrastructure::establish_postgresql_connection;
use rocket::serde::json::Json;
use serde_json::Value;
use shared::response_models::{Response, ResponseBody};

pub fn create_user(user_request: Json<NewUser>) -> Value {
    use domain::schema::users;

    let mut request: NewUser = user_request.into_inner();
    request.gen_password();

    match diesel::insert_into(users::table)
        .values(&request)
        .get_result::<Users>(&mut establish_postgresql_connection())
    {
        Ok(request) => {
            let response = Response {
                body: ResponseBody::User(request),
            };

            serde_json::to_value(response).unwrap()
        }
        Err(err) => {
            panic!("Database error - {}", err);
        }
    }
}
