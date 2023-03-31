use juniper::{EmptySubscription, FieldResult, GraphQLInputObject, GraphQLObject, RootNode};

#[derive(GraphQLObject)]
#[graphql(description = "Customer information")]
struct User {
    id: String,
    name: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Customer information input")]
struct NewUser {
    name: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn user(_id: String) -> FieldResult<User> {
        Ok(User {
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_user(new_user: NewUser) -> FieldResult<User> {
        Ok(User {
            id: "1234".to_owned(),
            name: new_user.name,
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
