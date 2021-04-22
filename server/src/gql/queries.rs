use async_graphql::{Context, FieldResult};
use crate::users::models::{Users, UsersDTO};
use sqlx::PgPool;

/// 定义查询根节点
pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn add(&self, a: i32, b: i32) -> i32 {
        log::info!("add(a:1, b:2)");
        a + b
    }

    async fn find_by_username(&self, ctx: &Context<'_>, username: String) -> FieldResult<UsersDTO> {
        let pool = ctx.data_unchecked::<PgPool>();
        let users = Users::find_by_username(pool, &username).await?;
        Ok(users)
    }
}