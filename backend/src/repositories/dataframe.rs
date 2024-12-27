use log::{error, info};
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::models::dataframe::DataFrame;

pub async fn save_dataframe(
    pool: &PgPool,
    dataframe_id: Uuid,
    transformation_id: Option<Uuid>,
    dataframe_duckdb_reference: String,
) -> Result<DataFrame, sqlx::Error> {
    // Create the DataFrame instance
    let dataframe = DataFrame::new(
        dataframe_id,
        Some("table_result".to_string()),
        transformation_id,
        dataframe_duckdb_reference.clone(),
    );

    // Perform an insert or update
    sqlx::query!(
        r#"
        INSERT INTO dataframe (id, transformation_id, dataframe_duckdb_reference)
        VALUES ($1, $2, $3)
        ON CONFLICT (id) 
        DO UPDATE SET
            transformation_id = EXCLUDED.transformation_id,
            dataframe_duckdb_reference = EXCLUDED.dataframe_duckdb_reference
        "#,
        dataframe.id,
        dataframe.transformation_id,
        dataframe_duckdb_reference
    )
    .execute(pool)
    .await?;

    Ok(dataframe)
}

// pub async fn get_dataframe_by_id(pool: &PgPool, id: Uuid) -> Result<Option<DataFrame>, Error> {
//     let dataframe = sqlx::query_as!(
//         DataFrame,
//         r#"
//         SELECT id, name, transformation_id, dataframe_duckdb_reference, created_at, created_by, updated_at, updated_by
//         FROM dataframe
//         WHERE id = $1
//         "#,
//         id
//     )
//     .fetch_optional(pool)
//     .await?;

//     Ok(dataframe)
// }

pub async fn get_dataframe_by_id(pool: &PgPool, id: Uuid) -> Result<Option<DataFrame>, Error> {
    let query = r#"
    SELECT id, name, transformation_id, dataframe_duckdb_reference, created_at, created_by, updated_at, updated_by
    FROM dataframe
    WHERE id = $1
    "#;

    info!("Executing query with id: {}", id);

    let dataframe = sqlx::query_as::<_, DataFrame>(query)
        .bind(id)
        .fetch_optional(pool)
        .await;

    match dataframe {
        Ok(result) => {
            info!("Query successful: {:?}", result);
            Ok(result)
        }
        Err(e) => {
            error!("Query execution failed: {}", e);
            Err(e)
        }
    }
}

pub async fn get_dataframes(pool: &PgPool) -> Result<Vec<DataFrame>, Error> {
    let dataframe = sqlx::query_as!(
        DataFrame,
        r#"
        SELECT id, name, transformation_id, dataframe_duckdb_reference, created_at, created_by, updated_at, updated_by
        FROM dataframe
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(dataframe)
}

pub async fn get_dataframes_by_ids(pool: &PgPool, ids: Vec<Uuid>) -> Result<Vec<DataFrame>, Error> {
    let dataframes = sqlx::query_as!(
        DataFrame,
        r#"
        SELECT id, name, transformation_id, dataframe_duckdb_reference, created_at, created_by, updated_at, updated_by
        FROM dataframe
        WHERE id = ANY($1)
        "#,
        &ids
    )
    .fetch_all(pool)
    .await?;

    Ok(dataframes)
}
