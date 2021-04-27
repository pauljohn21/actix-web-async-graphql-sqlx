use async_graphql::*;
use sqlx::PgPool;

use crate::domain::users::Users;
use crate::service::users::{ExtUsersService, UsersService};

/// 变更根节点
#[derive(MergedObject, Default)]
pub struct MutationRoot(UsersMutation);

/// 用户变更 Mutation
#[derive(Default)]
pub struct UsersMutation;

#[Object]
impl UsersMutation {
    /// 创建用户
    async fn create_user(&self, ctx: &Context<'_>, username: String, email: String, password: String) -> FieldResult<Users> {
        let pool = ctx.data::<PgPool>()?;
        let users = UsersService::create(pool, &username, &email, &password).await?;
        // let id = Users::create(pool, &username, &email, &password).await?;
        Ok(users)
    }
}