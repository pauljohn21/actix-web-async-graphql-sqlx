use async_graphql::*;
use crate::users::models::{Users, UsersDTO};
use sqlx::PgPool;

/// 变更根节点
pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn create_user(&self, ctx: &Context<'_>, username: String, email: String, password: String) -> FieldResult<UsersDTO> {
        let pool = ctx.data_unchecked::<PgPool>();
        let users = Users::create(pool, &username, &email, &password).await?;
        Ok(users)
    }
}