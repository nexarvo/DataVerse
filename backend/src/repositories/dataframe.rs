use sqlx::{PgPool, Error};
use uuid::Uuid;

use crate::models::dataframe::DataFrame;

pub async fn save_dataframe(pool: &PgPool, dataframe_id: Uuid, transformation_id: Uuid, dataframe_duckdb_reference: String) -> Result<DataFrame, sqlx::Error> {
    let dataframe = DataFrame::new(dataframe_id, transformation_id, dataframe_duckdb_reference);
    
    sqlx::query!(
        r#"
        INSERT INTO dataframe (id, transformation_id, dataframe_duckdb_reference)
        VALUES ($1, $2, $3)
        "#,
        dataframe.id,
        dataframe.transformation_id,
        dataframe.dataframe_duckdb_reference
    )
    .execute(pool)
    .await?;

    Ok(dataframe)
}

pub async fn get_dataframe_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<DataFrame>, Error> {
    let dataframe = sqlx::query_as!(
        DataFrame,
        r#"
        SELECT id, transformation_id, dataframe_duckdb_reference, created_at, created_by, updated_at, updated_by
        FROM dataframe
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(dataframe)
}