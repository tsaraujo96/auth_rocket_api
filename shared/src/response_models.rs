use domain::models::Users;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub enum ResponseBody {
    Users(Vec<Users>),
    User(Users),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}
