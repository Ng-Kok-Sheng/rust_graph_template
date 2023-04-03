use crate::util::postgres;

pub struct GraphQLContext {
    pub pool: postgres::PostgresPool,
}
