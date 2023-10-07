use diesel::prelude::*;
use domain::models::Users;
use infrastructure::establish_postgresql_connection;

pub fn list_users() -> Vec<Users> {
    use domain::schema::users;

    match users::table
        .select(users::all_columns)
        .load::<Users>(&mut establish_postgresql_connection())
    {
        Ok(mut users) => {
            users.sort();
            users
        }
        Err(err) => {
            panic!("Database error - {}", err);
        }
    }
}
