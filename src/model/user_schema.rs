use crate::api;
use crate::schema::users;
use actix_web::Error;
use diesel::prelude::*;
use juniper::{
    graphql_value, EmptySubscription, FieldError, FieldResult, GraphQLInputObject, GraphQLObject,
    RootNode,
};

#[derive(GraphQLObject, Queryable)]
#[graphql(description = "Customer information")]
struct User {
    id: i32,
    full_name: String,
    username: String,
    password: String,
    email_address: String,
}

#[derive(GraphQLInputObject, Insertable)]
#[diesel(table_name = users)]
#[graphql(description = "New customer information")]
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
    fn create_user<'db>(context: &'db api::GraphQLContext, new_user: NewUser) -> FieldResult<User> {
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();

        match diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(conn)
        {
            Ok(user) => Ok(user),
            Err(err) => Err(FieldError::new(
                err.to_string(),
                graphql_value!({ "internal_error": {err.to_string()} }),
            )),
        }
    }
}

pub type Schema =
    RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<api::GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
