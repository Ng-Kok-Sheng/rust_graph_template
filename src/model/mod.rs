pub mod user_schema;
use crate::util::types;
use juniper::{EmptySubscription, RootNode};

// Aggregated queries struct
pub struct RootQuery;
#[juniper::graphql_object(Context = types::GraphQLContext)]
impl RootQuery {
    fn users(&self) -> user_schema::UserQuery {
        user_schema::UserQuery
    }
}

// Aggregated mutations struct
pub struct RootMutation;
#[juniper::graphql_object(Context = types::GraphQLContext)]
impl RootMutation {
    fn users(&self) -> user_schema::UserQuery {
        user_schema::UserQuery
    }
}

pub type Schema =
    RootNode<'static, RootQuery, RootMutation, EmptySubscription<types::GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(RootQuery {}, RootMutation {}, EmptySubscription::new())
}
