use async_graphql::*;
use sqlx::PgPool;
use crate::service::users::Users;

/// 变更根节点
pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    /// 创建用户
    async fn create_user(&self, ctx: &Context<'_>, username: String, email: String, password: String) -> FieldResult<Users> {
        let pool = ctx.data::<PgPool>()?;
        let id = Users::create(pool, &username, &email, &password).await?;
        Ok(id)
    }
}