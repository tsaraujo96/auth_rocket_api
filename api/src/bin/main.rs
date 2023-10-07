#[macro_use]
extern crate rocket;

use std::str::FromStr;
use rocket::serde::json::Json;
use rocket_cors::{AllowedMethods, AllowedOrigins, CorsOptions};
use domain::constants::{ErrorResponse, NOT_FOUND_JSON, UNAUTHORIZED_JSON, UNKNOWN_JSON};

use api::api_routes::{create_user, list_users, login};

#[launch]
async fn rocket() -> _ {

    let allowed_origins = AllowedOrigins::All;
    let allowed_methods: AllowedMethods = ["Get", "Post", "Delete"]
        .iter()
        .map(|s| FromStr::from_str(s).unwrap())
        .collect();


    let cors = CorsOptions::default()
        .allowed_origins(allowed_origins)
        .allowed_methods(allowed_methods)
        .allow_credentials(true);
    rocket::build()
        .mount(
            "/",
            routes![
                login,
                list_users,
                create_user
            ],
        )
        .manage(cors.to_cors())
        .register(
            "/",
            catchers![unauthorized, not_found, internal_sever_error,],
        )
}

#[catch(401)]
pub fn unauthorized() -> Json<ErrorResponse> {
    Json(UNAUTHORIZED_JSON)
}

#[catch(404)]
pub fn not_found() -> Json<ErrorResponse> {
    Json(NOT_FOUND_JSON)
}

#[catch(500)]
pub fn internal_sever_error() -> Json<ErrorResponse> {
    Json(UNKNOWN_JSON)
}

