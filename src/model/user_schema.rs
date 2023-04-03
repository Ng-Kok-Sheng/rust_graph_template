use crate::util::types;
use crate::{schema::users, util::postgres::handle_error};
use diesel::prelude::*;
use juniper::{FieldResult, GraphQLInputObject, GraphQLObject};

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

pub struct UserQuery;

#[juniper::graphql_object(Context = types::GraphQLContext)]
impl UserQuery {
    #[graphql(name = "get_user_by_id")]
    fn get_user_by_id(context: &types::GraphQLContext, query_id: i32) -> FieldResult<User> {
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();

        match users::table
            .filter(users::id.eq(query_id))
            .limit(1)
            .load::<User>(conn)
        {
            Ok(users) => Ok(users[0].clone()),
            Err(err) => Err(handle_error(err)),
        }
    }

    #[graphql(name = "get_multiple_users")]
    fn get_users(context: &types::GraphQLContext) -> FieldResult<Vec<User>> {
        let conn: &mut PgConnection = &mut context.pool.get().unwrap();

        match users::table.load::<User>(conn) {
            Ok(users) => Ok(users),
            Err(err) => Err(handle_error(err)),
        }
    }
}

pub struct UserMutation;

#[juniper::graphql_object(Context = types::GraphQLContext)]
impl UserMutation {
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
            Err(err) => Err(handle_error(err)),
        }
    }
}
