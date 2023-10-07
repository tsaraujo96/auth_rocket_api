use diesel::pg::PgConnection;
use diesel::prelude::*;
use settings::settings::AppConfig;


extern crate mongodb;
use mongodb::{options::ClientOptions, Client};

pub fn establish_postgresql_connection() -> PgConnection {
    let config = AppConfig::load();

    let database_url = config.postgres_url;
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn establish_mongo_connection() -> Client {
    let config = AppConfig::load();


    let database_url = config.mongo_url;
    let client_options = ClientOptions::parse(&database_url).await.unwrap();

    Client::with_options(client_options)
        .unwrap_or_else(|_| panic!("Error connecting to {}", &database_url))
}
