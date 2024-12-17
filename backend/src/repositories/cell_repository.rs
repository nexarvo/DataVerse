use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::{
    dto::{
        cell::CellDTO, dataframe::DataframeMetadataDTO, datasets::DatasetMetadataDTO,
        transformation::TransformationDTO,
    },
    models::cell::Cell,
};

pub async fn insert_or_update_cell(pool: &PgPool, cell: Cell) -> Result<Uuid, Error> {
    // Insert or update query using PostgreSQL's ON CONFLICT clause for upsert behavior
    let result = sqlx::query!(
        r#"
        INSERT INTO cell (
            id,
            name,
            first_transformation_id,
            input_dataframe_id,
            input_dataset_id,
            result_dataframe_id,
            created_at,
            created_by,
            updated_at,
            updated_by
        ) 
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10
        )
        ON CONFLICT (id) 
        DO UPDATE SET
            name = EXCLUDED.name,
            first_transformation_id = EXCLUDED.first_transformation_id,
            input_dataframe_id = EXCLUDED.input_dataframe_id,
            input_dataset_id = EXCLUDED.input_dataset_id,
            result_dataframe_id = EXCLUDED.result_dataframe_id,
            created_at = EXCLUDED.created_at,
            created_by = EXCLUDED.created_by,
            updated_at = EXCLUDED.updated_at,
            updated_by = EXCLUDED.updated_by
        RETURNING id
        "#,
        cell.id,
        cell.name,
        cell.first_transformation_id,
        cell.input_dataframe_id,
        cell.input_dataset_id,
        cell.result_dataframe_id,
        cell.created_at,
        cell.created_by,
        cell.updated_at,
        cell.updated_by
    )
    .fetch_one(pool)
    .await?;

    // Return the ID of the inserted or updated cell
    Ok(result.id)
}

pub async fn get_cells(pool: &PgPool) -> Result<Vec<CellDTO>, Error> {
    // Fetch all cell details
    let cell_rows = sqlx::query!(
        r#"
        SELECT 
            c.id,
            c.name,
            c.input_dataframe_id,
            c.input_dataset_id,
            c.result_dataframe_id,
            c.created_at,
            c.created_by,
            c.updated_at,
            c.updated_by
        FROM cell c
        ORDER BY c.created_at ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    // Create a vector to hold all the CellDTOs
    let mut cell_dtos = Vec::new();

    // Fetch details for each cell
    for cell_row in cell_rows {
        // Fetch input_dataframe details if present
        let input_dataframe = if let Some(input_dataframe_id) = cell_row.input_dataframe_id {
            Some(
                sqlx::query!(
                    r#"
                    SELECT 
                        d.id,
                        d.dataframe_duckdb_reference,
                        d.created_at,
                        d.created_by,
                        d.updated_at,
                        d.updated_by
                    FROM dataframe d
                    WHERE d.id = $1
                    "#,
                    input_dataframe_id
                )
                .fetch_one(pool)
                .await
                .map(|row| DataframeMetadataDTO {
                    id: row.id,
                    name: None,              // Add logic if needed
                    transformations: vec![], // Add logic to fetch transformations if required
                    dataframe_duckdb_reference: row.dataframe_duckdb_reference,
                    created_at: row.created_at,
                    created_by: row.created_by,
                    updated_at: row.updated_at,
                    updated_by: row.updated_by,
                })
                .map_err(|e| Error::from(e))?,
            )
        } else {
            None
        };

        // Fetch input_dataset details if present
        let input_dataset = if let Some(input_dataset_id) = cell_row.input_dataset_id {
            Some(
                sqlx::query!(
                    r#"
                    SELECT 
                        ds.id,
                        ds.file_name,
                        ds.file_size,
                        ds.file_type,
                        ds.upload_time,
                        ds.uploaded_by,
                        ds.dataset_url,
                        ds.row_count
                    FROM datasets ds
                    WHERE ds.id = $1
                    "#,
                    input_dataset_id
                )
                .fetch_one(pool)
                .await
                .map(|row| DatasetMetadataDTO {
                    id: Some(row.id),
                    file_name: Some(row.file_name),
                    file_size: Some(row.file_size),
                    file_type: Some(row.file_type),
                    upload_time: row.upload_time,
                    uploaded_by: row.uploaded_by,
                    dataset_url: Some(row.dataset_url),
                    row_count: row.row_count,
                })
                .map_err(|e| Error::from(e))?,
            )
        } else {
            None
        };

        // Fetch result_dataframe details if present
        let result_dataframe = if let Some(result_dataframe_id) = cell_row.result_dataframe_id {
            Some(
                sqlx::query!(
                    r#"
                    SELECT 
                        d.id,
                        d.dataframe_duckdb_reference,
                        d.created_at,
                        d.created_by,
                        d.updated_at,
                        d.updated_by
                    FROM dataframe d
                    WHERE d.id = $1
                    "#,
                    result_dataframe_id
                )
                .fetch_one(pool)
                .await
                .map(|row| DataframeMetadataDTO {
                    id: row.id,
                    name: None,              // Add logic if needed
                    transformations: vec![], // Add logic to fetch transformations if required
                    dataframe_duckdb_reference: row.dataframe_duckdb_reference,
                    created_at: row.created_at,
                    created_by: row.created_by,
                    updated_at: row.updated_at,
                    updated_by: row.updated_by,
                })
                .map_err(|e| Error::from(e))?,
            )
        } else {
            None
        };

        // Construct the CellDTO for the current row
        let cell_dto = CellDTO {
            id: cell_row.id,
            name: cell_row.name,
            input_dataframe,
            input_dataset,
            result_dataframe,
            created_at: cell_row.created_at,
            created_by: cell_row.created_by,
            updated_at: cell_row.updated_at,
            updated_by: cell_row.updated_by,
        };

        // Add the CellDTO to the vector
        cell_dtos.push(cell_dto);
    }

    Ok(cell_dtos)
}

pub async fn get_cell_by_id(pool: &PgPool, cell_id: uuid::Uuid) -> Result<CellDTO, Error> {
    // Fetch the cell details
    let cell_row = sqlx::query!(
        r#"
        SELECT
            c.id,
            c.name,
            c.input_dataframe_id,
            c.input_dataset_id,
            c.result_dataframe_id,
            c.created_at,
            c.created_by,
            c.updated_at,
            c.updated_by
        FROM cell c
        WHERE c.id = $1
        "#,
        cell_id
    )
    .fetch_one(pool)
    .await?;

    // Fetch input_dataframe details if present
    let input_dataframe = if let Some(input_dataframe_id) = cell_row.input_dataframe_id {
        Some(
            sqlx::query!(
                r#"
                SELECT
                    d.id,
                    d.dataframe_duckdb_reference,
                    d.created_at,
                    d.created_by,
                    d.updated_at,
                    d.updated_by
                FROM dataframe d
                WHERE d.id = $1
                "#,
                input_dataframe_id
            )
            .fetch_one(pool)
            .await
            .map(|row| DataframeMetadataDTO {
                id: row.id,
                name: None,              // Add logic if needed
                transformations: vec![], // Add logic to fetch transformations if required
                dataframe_duckdb_reference: row.dataframe_duckdb_reference,
                created_at: row.created_at,
                created_by: row.created_by,
                updated_at: row.updated_at,
                updated_by: row.updated_by,
            })
            .map_err(|e| Error::from(e))?,
        )
    } else {
        None
    };

    // Fetch input_dataset details if present
    let input_dataset = if let Some(input_dataset_id) = cell_row.input_dataset_id {
        Some(
            sqlx::query!(
                r#"
                SELECT
                    ds.id,
                    ds.file_name,
                    ds.file_size,
                    ds.file_type,
                    ds.upload_time,
                    ds.uploaded_by,
                    ds.dataset_url,
                    ds.row_count
                FROM datasets ds
                WHERE ds.id = $1
                "#,
                input_dataset_id
            )
            .fetch_one(pool)
            .await
            .map(|row| DatasetMetadataDTO {
                id: Some(row.id),
                file_name: Some(row.file_name),
                file_size: Some(row.file_size),
                file_type: Some(row.file_type),
                upload_time: row.upload_time,
                uploaded_by: row.uploaded_by,
                dataset_url: Some(row.dataset_url),
                row_count: row.row_count,
            })
            .map_err(|e| Error::from(e))?,
        )
    } else {
        None
    };

    // Fetch result_dataframe details if present
    let result_dataframe = if let Some(result_dataframe_id) = cell_row.result_dataframe_id {
        Some(
            sqlx::query!(
                r#"
                SELECT
                    d.id,
                    d.dataframe_duckdb_reference,
                    d.created_at,
                    d.created_by,
                    d.updated_at,
                    d.updated_by
                FROM dataframe d
                WHERE d.id = $1
                "#,
                result_dataframe_id
            )
            .fetch_one(pool)
            .await
            .map(|row| DataframeMetadataDTO {
                id: row.id,
                name: None,              // Add logic if needed
                transformations: vec![], // Add logic to fetch transformations if required
                dataframe_duckdb_reference: row.dataframe_duckdb_reference,
                created_at: row.created_at,
                created_by: row.created_by,
                updated_at: row.updated_at,
                updated_by: row.updated_by,
            })
            .map_err(|e| Error::from(e))?,
        )
    } else {
        None
    };

    // Construct the CellDTO
    let cell_dto = CellDTO {
        id: cell_row.id,
        name: cell_row.name,
        input_dataframe,
        input_dataset,
        result_dataframe,
        created_at: cell_row.created_at,
        created_by: cell_row.created_by,
        updated_at: cell_row.updated_at,
        updated_by: cell_row.updated_by,
    };

    Ok(cell_dto)
}

pub async fn get_cell_metadata_by_id(
    pool: &PgPool,
    cell_id: uuid::Uuid,
) -> Result<Option<Cell>, Error> {
    // Fetch the cell metadata
    let cell_row = sqlx::query!(
        r#"
        SELECT
            c.id,
            c.name,
            c.cell_type,
            c.input_dataframe_id,
            c.input_dataset_id,
            c.result_dataframe_id,
            c.first_transformation_id,
            c.created_at,
            c.created_by,
            c.updated_at,
            c.updated_by
        FROM cell c
        WHERE c.id = $1
        "#,
        cell_id
    )
    .fetch_optional(pool) // Fetch optional result
    .await?;

    // If no cell is found, return None
    if cell_row.is_none() {
        return Ok(None);
    }

    let cell_row = cell_row.unwrap(); // unwrap once and reuse

    // Map the Record to the Cell struct
    let cell = Cell {
        id: cell_row.id,
        name: cell_row.name,
        cell_type: cell_row.cell_type,
        input_dataframe_id: cell_row.input_dataframe_id,
        input_dataset_id: cell_row.input_dataset_id,
        result_dataframe_id: cell_row.result_dataframe_id,
        created_at: cell_row.created_at,
        created_by: cell_row.created_by,
        updated_at: cell_row.updated_at,
        updated_by: cell_row.updated_by,
        first_transformation_id: cell_row.first_transformation_id,
    };

    Ok(Some(cell)) // Return the Cell wrapped in Some
}

pub async fn create_cell(pool: &PgPool, cell: &Cell) -> Result<Cell, Error> {
    // Insert the cell into the database
    let inserted_cell = sqlx::query_as!(
        Cell,
        r#"
        INSERT INTO cell (
            id, name, first_transformation_id, input_dataframe_id, 
            input_dataset_id, result_dataframe_id, cell_type, 
            created_at, created_by, updated_at, updated_by
        ) 
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
        )
        RETURNING 
            id, name, first_transformation_id, input_dataframe_id, 
            input_dataset_id, result_dataframe_id, cell_type, 
            created_at, created_by, updated_at, updated_by
        "#,
        cell.id,
        cell.name,
        cell.first_transformation_id,
        cell.input_dataframe_id,
        cell.input_dataset_id,
        cell.result_dataframe_id,
        cell.cell_type,
        cell.created_at,
        cell.created_by,
        cell.updated_at,
        cell.updated_by,
    )
    .fetch_one(pool)
    .await?;

    Ok(inserted_cell)
}
