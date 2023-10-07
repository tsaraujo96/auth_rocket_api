use application::login::login_uc;
use application::user::{create, read};
use domain::models::{NewUser, Users};
use rocket::{get, post};
use rocket::form::Form;
use shared::response_models::{Response as Resp, ResponseBody};

use rocket::response::status::Created;

use rocket::serde::json::Json;
use serde_json::Value;
use application::login::request_access_token::AuthorizedUser;

#[get("/get_users")]
pub fn list_users(_auth: AuthorizedUser) -> Value {
    let users: Vec<Users> = read::list_users();
    let response = Resp {
        body: ResponseBody::Users(users),
    };

    serde_json::to_value(response).unwrap()
}

#[allow(warnings)]
#[post("/create_user", format = "application/json", data = "<user_request>")]
pub fn create_user(user_request: Json<NewUser>) -> Created<Json<Value>> {
    let json_response = create::create_user(user_request);
    let json_data = Json(json_response);

    Created::new("").body(json_data)
}

#[allow(warnings)]
#[get("/login", data = "<user_request>")]
pub fn login(user_request: Form<NewUser>) -> Value {
    login_uc::login_user(user_request)
}
