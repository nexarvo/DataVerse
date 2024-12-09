use sqlx::{PgPool, Error};
use uuid::Uuid;

use crate::models::user::User;


pub async fn find_user_by_email(
    pool: &PgPool,
    user_email: String,
) -> Result<Option<User>, Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, email, password_hash, google_id
        FROM users
        WHERE email = $1
        "#,
        user_email
    )
    .fetch_optional(pool)
    .await?;

    println!("Query result: {:?}", user);

    Ok(user)
}

pub async fn insert_new_user(
    pool: &PgPool,
    user_email: String,
    hashed_password: String,
) -> Result<User, Error> {
    let new_user = User {
        id: Some(Uuid::new_v4()),
        email: user_email,
        password_hash: hashed_password,
        google_id: None,
    };

    sqlx::query!(
        r#"
        INSERT INTO users (id, email, password_hash, google_id)
        VALUES ($1, $2, $3, $4)
        "#,
        new_user.id,
        new_user.email,
        new_user.password_hash,
        new_user.google_id
    )
    .execute(pool)
    .await?;

    Ok(new_user)
}