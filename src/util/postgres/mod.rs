use actix_web_lab::__reexports::tracing::log;
use diesel::r2d2::ConnectionManager;
use diesel::{pg::PgConnection, result::Error};
use dotenv::dotenv;
use juniper::{graphql_value, FieldError};
use r2d2::Pool;
use std::env;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_postgres() -> PostgresPool {
    // it from the environment within this function
    dotenv().ok();
    log::info!("Setting up connection to PostgreSQL.");
    let url = env::var("DATABASE_URL").expect("no DB URL");
    let migr = ConnectionManager::<PgConnection>::new(url);
    log::info!("Connecting to PostgreSQL.");
    r2d2::Pool::builder()
        .build(migr)
        .expect("could not build connection pool")
}

pub fn handle_error(err: Error) -> FieldError {
    FieldError::new(
        err.to_string(),
        graphql_value!({ "internal_error": {err.to_string()} }),
    )
}
