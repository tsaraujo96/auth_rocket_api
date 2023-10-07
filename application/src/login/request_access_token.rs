use crate::login::create_token::{decode_jwt, DecodeJwtHelper};
use crate::private::JWT_SECRET;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

pub struct AuthorizedUser {
    pub user_id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthorizedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        match check_data_from_auth_header(auth_header) {
            Ok(vec_header) => match decode_jwt(vec_header[1].to_string(), JWT_SECRET) {
                DecodeJwtHelper::Ok(token_data) => {
                    Outcome::Success(AuthorizedUser {
                    user_id: token_data.claims.user_id,
                })},
                DecodeJwtHelper::Err => Outcome::Failure((Status::Unauthorized, ())),
            },
            Err(_) => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}

pub fn check_data_from_auth_header(auth_header: Option<&str>) -> Result<Vec<&str>, ()> {
    return if let Some(auth_string) = auth_header {
        let vec_header = auth_string.split_whitespace().collect::<Vec<_>>();
        if vec_header.len() != 2
            && vec_header[0] == "Bearer"
            && !vec_header[0].is_empty()
            && !vec_header[1].is_empty()
        {
            Err(())
        } else {
            Ok(vec_header)
        }
    } else {
        Err(())
    };
}
