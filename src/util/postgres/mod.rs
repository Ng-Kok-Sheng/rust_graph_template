use actix_web_lab::__reexports::tracing::log;
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
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
