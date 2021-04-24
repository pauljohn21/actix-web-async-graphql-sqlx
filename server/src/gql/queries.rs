use async_graphql::*;
use crate::service::users::Users;
use sqlx::PgPool;

/// 定义查询根节点
#[derive(MergedObject, Default)]
pub struct QueryRoot(UsersQuery);

/// 用户查询 queries
#[derive(Default)]
pub struct UsersQuery;

#[Object]
impl UsersQuery {
    /// 通过用户名查询
    async fn find_by_username(&self, ctx: &Context<'_>, username: String) -> FieldResult<Option<Users>> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Users::find_by_username(pool, &username).await?)
    }

    /// 检查用户名是否存在
    async fn exists_by_username(&self, ctx: &Context<'_>, username: String) -> FieldResult<bool> {
        let pool = ctx.data::<PgPool>()?;
        Ok(Users::exists_by_username(pool, &username).await?)
    }
}