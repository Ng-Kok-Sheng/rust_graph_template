mod api;
mod model;
mod schema;
mod util;
use actix_web::{middleware, web::Data, App, HttpServer};
use actix_web_lab::__reexports::tracing::log;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let postgres = util::postgres::get_postgres();
    log::info!("Connected to Postgres");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(postgres.clone()))
            .service(api::get_user_scope())
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
