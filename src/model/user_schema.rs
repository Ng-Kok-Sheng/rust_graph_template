use crate::schema::users;
use crate::util::types;
use diesel::prelude::*;
use juniper::{
    graphql_value, EmptySubscription, FieldError, FieldResult, GraphQLInputObject, GraphQLObject,
    RootNode,
};

#[derive(GraphQLObject, Queryable, Clone)]
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

#[juniper::graphql_object(Context = types::GraphQLContext)]
impl QueryRoot {
    #[graphql(name = "get_user_by_id")]
    fn get_user_by_id(context: &types::GraphQLContext, query_id: i32) -> FieldResult<User> {
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();

        match users::table
            .filter(users::id.eq(query_id))
            .limit(1)
            .load::<User>(conn)
        {
            Ok(users) => Ok(users[0].clone()),
            Err(err) => Err(FieldError::new(
                err.to_string(),
                graphql_value!({ "internal_error": {err.to_string()} }),
            )),
        }
    }

    #[graphql(name = "get_multiple_users")]
    fn get_users(context: &types::GraphQLContext) -> FieldResult<Vec<User>> {
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();

        match users::table.load::<User>(conn) {
            Ok(users) => Ok(users),
            Err(err) => Err(FieldError::new(
                err.to_string(),
                graphql_value!({ "internal_error": {err.to_string()} }),
            )),
        }
    }
}

pub struct MutationRoot;

#[juniper::graphql_object(Context = types::GraphQLContext)]
impl MutationRoot {
    fn create_user<'db>(
        context: &'db types::GraphQLContext,
        new_user: NewUser,
    ) -> FieldResult<User> {
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
    RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<types::GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
