use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;

pub const EXPIRATION_TOKEN: i64 = 3600;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub(crate) cause: &'static str,
}

pub const ERROR_WRONG_REQUEST_STATUS: Status = Status::BadRequest;
pub const WRONG_REQUEST_JSON: ErrorResponse = ErrorResponse {
    cause: "Wrong request",
};

pub const WRONG_REQUEST: (Status, Json<ErrorResponse>) =
    (ERROR_WRONG_REQUEST_STATUS, Json(WRONG_REQUEST_JSON));

pub const ERROR_UNKNOWN_STATUS: Status = Status::InternalServerError;
pub const UNKNOWN_JSON: ErrorResponse = ErrorResponse {
    cause: "Internal Server Error",
};

pub const ERROR_UNAUTHORIZED_STATUS: Status = Status::Unauthorized;
pub const UNAUTHORIZED_JSON: ErrorResponse = ErrorResponse {
    cause: "Unauthorized",
};

pub const UNKNOWN: (Status, Json<ErrorResponse>) = (ERROR_UNKNOWN_STATUS, Json(UNKNOWN_JSON));
pub const UNAUTHORIZED: (Status, Json<ErrorResponse>) =
    (ERROR_UNAUTHORIZED_STATUS, Json(UNAUTHORIZED_JSON));


pub const ERROR_NOT_FOUND_STATUS: Status = Status::NotFound;
pub const NOT_FOUND_JSON: ErrorResponse = ErrorResponse { cause: "Not found" };
pub const NOT_FOUND: (Status, Json<ErrorResponse>) = (ERROR_NOT_FOUND_STATUS, Json(NOT_FOUND_JSON));
