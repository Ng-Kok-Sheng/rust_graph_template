use crate::model::user_schema;
use actix_web::{
    get, route,
    web::{self, Data},
    HttpResponse, Responder,
};
use actix_web_lab::respond::Html;
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

pub fn get_user_scope() -> actix_web::Scope {
    let schema = std::sync::Arc::new(user_schema::create_schema());
    web::scope("")
        .app_data(Data::from(schema.clone()))
        .service(graphql)
        .service(graphql_playground)
        .service(index)
}

pub fn get_user_scope_2() -> actix_web::Scope {
    let schema = std::sync::Arc::new(user_schema::create_schema());
    web::scope("/app")
        .app_data(Data::from(schema.clone()))
        .service(graphql)
        .service(graphql_playground)
        .service(index)
}

#[get("/index")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
}

// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(
    st: web::Data<user_schema::Schema>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}

// Graphql playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}
