use crate::model;
use crate::util::{postgres, types};
use actix_web::{
    get, route,
    web::{self, Data},
    HttpResponse, Responder,
};
use actix_web_lab::{__reexports::tracing::log, respond::Html};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

pub fn get_user_scope() -> actix_web::Scope {
    let schema = std::sync::Arc::new(model::create_schema());

    log::info!("Starting HTTP server on port 8080");
    log::info!("GraphiQL playground: http://localhost:8080/graphiql");
    web::scope("")
        .app_data(Data::from(schema.clone()))
        .service(graphql)
        .service(graphql_playground)
}

// Configure context types
impl juniper::Context for types::GraphQLContext {}

// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(
    // DB Connection Pool
    postgres: web::Data<postgres::PostgresPool>,
    //GraphQl Schema
    schema: web::Data<model::Schema>,
    // Incoming HTTP Request
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let ctx = types::GraphQLContext {
        pool: postgres.get_ref().to_owned(),
    };

    let res = data.execute(&schema, &ctx).await;
    HttpResponse::Ok().json(res)
}

// Graphql playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}
