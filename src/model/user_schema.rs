use crate::api;
use diesel::prelude::*;
use juniper::{EmptySubscription, FieldResult, GraphQLInputObject, GraphQLObject, RootNode};

#[derive(GraphQLObject)]
#[graphql(description = "Customer information")]
struct User {
    id: i32,
    full_name: String,
    username: String,
    password: String,
    email_address: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Customer information input")]
struct NewUser {
    full_name: String,
    username: String,
    password: String,
    email_address: String,
}

pub struct QueryRoot;

#[juniper::graphql_object(Context = api::GraphQLContext)]
impl QueryRoot {
    #[graphql(name = "get_user_by_id")]
    fn get_user_by_id(_context: &api::GraphQLContext, _id: i32) -> FieldResult<User> {
        Ok(User {
            id: 1,
            full_name: "Luke".to_owned(),
            username: "Luke".to_owned(),
            password: "Luke".to_owned(),
            email_address: "Luke".to_owned(),
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = api::GraphQLContext)]
impl MutationRoot {
    fn create_user(context: &api::GraphQLContext, _new_user: NewUser) -> FieldResult<User> {
        let _conn: &PgConnection = &context.pool.get().unwrap();

        Ok(User {
            id: 1,
            full_name: "Luke".to_owned(),
            username: "Luke".to_owned(),
            password: "Luke".to_owned(),
            email_address: "Luke".to_owned(),
        })
    }
}

pub type Schema =
    RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<api::GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
