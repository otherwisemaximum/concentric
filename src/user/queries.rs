use chrono::Utc;
use sqlx::{mysql::MySqlQueryResult, MySql, Pool};
use tracing::info;

use crate::entity::user::{CreateUser, User};

pub async fn create_user(
    user: CreateUser,
    pool: &Pool<MySql>,
    internal: bool,
    active: bool,
    audit_user: String,
) -> Result<MySqlQueryResult, sqlx::Error> {
    // TODO: allow user to be created based on current user roles
    info!("creating user for username: {}", user.username);
    sqlx::query(
        "INSERT INTO users (username, email, password, internal_flag, active, 
            create_timestamp, create_user, update_timestamp, update_user) VALUES
            (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(user.username)
    .bind(user.email)
    .bind(bcrypt::hash(user.password, 10).unwrap())
    .bind(internal)
    .bind(active)
    .bind(Utc::now())
    .bind(audit_user.clone())
    .bind(Utc::now())
    .bind(audit_user)
    .execute(pool)
    .await
}

pub async fn get_user_by_email(email: &str, pool: &Pool<MySql>) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(email)
        .fetch_one(pool)
        .await
}
