use crate::users::models::{Users, UsersDTO};
use sqlx::PgPool;
use anyhow::Result;

impl Users {
    pub async fn create(pool: &PgPool,
                        username: &str,
                        email: &str,
                        password_hash: &str,
    ) -> Result<UsersDTO>
    {
        //language=PostgreSQL
        let row = sqlx::query_as::<_, UsersDTO>("INSERT INTO users(username, email, password_hash) VALUES ($1, $2, $3) RETURNING *")
            .bind(username)
            .bind(email).bind(password_hash)
            .fetch_one(pool).await?;
        Ok(row)
    }

    pub async fn find_by_username(pool: &PgPool, username: &str) -> Result<UsersDTO> {
        //language=sql
        let users = sqlx::query_as::<_, UsersDTO>("SELECT * FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(pool)
            .await?;
        Ok(users)
    }
}