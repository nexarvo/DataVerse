use chrono::NaiveDateTime;
use serde_json::Value;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::{
    dto::{
        cell::{CellDTO, CellSQLInputsModal}, dataframe::DataframeMetadataDTO, datasets::DatasetMetadataDTO,
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
            c.cell_type,
            c.cell_order,
            c.input_dataframe_id,
            c.input_dataset_id,
            c.result_dataframe_id,
            c.created_at,
            c.created_by,
            c.updated_at,
            c.updated_by
        FROM cell c
        ORDER BY c.cell_order ASC
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
            cell_order: cell_row.cell_order,
            cell_type: cell_row.cell_type,
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
            c.cell_type,
            c.cell_order,
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
        cell_order: cell_row.cell_order,
        cell_type: cell_row.cell_type,
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
            c.cell_order,
            c.input_dataframe_id,
            c.input_dataset_id,
            c.result_dataframe_id,
            c.first_transformation_id,
            c.inputs,
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
        cell_order: cell_row.cell_order,
        inputs: cell_row.inputs.map(|json_value| {
            serde_json::from_value(json_value).unwrap_or_else(|_| None)
        }).flatten(),
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
            input_dataset_id, result_dataframe_id, cell_type, cell_order, inputs, 
            created_at, created_by, updated_at, updated_by
        ) 
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13
        )
        RETURNING 
            id, name, first_transformation_id, input_dataframe_id, 
            input_dataset_id, result_dataframe_id, cell_type, cell_order, inputs,
            created_at, created_by, updated_at, updated_by
        "#,
        cell.id,
        cell.name,
        cell.first_transformation_id,
        cell.input_dataframe_id,
        cell.input_dataset_id,
        cell.result_dataframe_id,
        cell.cell_type,
        cell.cell_order,
        cell.inputs,
        cell.created_at,
        cell.created_by,
        cell.updated_at,
        cell.updated_by,
    )
    .fetch_one(pool)
    .await?;

    Ok(inserted_cell)
}

pub async fn add_cell_at_position(
    pool: &PgPool,
    reference_cell_id: Option<Uuid>, // The reference cell for insertion (if any)
    cell_order: i32,                 // The desired order of the new cell
    new_cell: Cell,                  // The new cell to be added
) -> Result<Cell, sqlx::Error> {
    // Fetch all cells and sort by cell_order to determine position
    let cells: Vec<Cell> = sqlx::query_as::<_, Cell>("SELECT * FROM cell ORDER BY cell_order")
        .fetch_all(pool)
        .await?;

    // Determine the appropriate order for the new cell
    let new_cell_order = if let Some(reference_id) = reference_cell_id {
        // Find the reference cell's order
        let reference_cell = cells.iter().find(|c| c.id == reference_id);
        if let Some(reference) = reference_cell {
            let reference_order = reference.cell_order.unwrap_or(0); // Default to 0 if `cell_order` is None

            // If inserting after the reference cell
            if Some(cell_order) == reference.cell_order {
                // If user gives the same order, calculate an average for new cell
                let next_cell = cells
                    .iter()
                    .find(|c| c.cell_order == Some(reference_order + 1));
                if let Some(next) = next_cell {
                    let next_order = next.cell_order.unwrap_or(0); // Default to 0 if `cell_order` is None

                    let avg_order = (reference_order + next_order) / 2; // Now you can safely add the values
                    avg_order
                } else {
                    // If there is no next cell, insert it as the next after the reference
                    reference.cell_order.unwrap_or(0) + 1
                }
            } else {
                cell_order // The order specified by the user
            }
        } else {
            // If no reference cell, insert at the provided order or the end
            cell_order
        }
    } else {
        // If no reference cell, insert at the provided order or the end
        cell_order
    };

    // Update cell orders of cells that will be moved down (in case of new cell before/after)
    sqlx::query!(
        "UPDATE cell SET cell_order = cell_order + 1 WHERE cell_order >= $1",
        new_cell_order
    )
    .execute(pool)
    .await?;

    // Insert the new cell at the correct order and return it as a `Cell` type
    let inserted_cell: Cell = sqlx::query_as!(
        Cell, 
        "INSERT INTO cell (id, name, input_dataframe_id, input_dataset_id, cell_type, cell_order, first_transformation_id, result_dataframe_id, created_at, created_by, updated_at, updated_by
        , inputs)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13) 
         RETURNING id, name, input_dataframe_id, input_dataset_id, cell_type, cell_order, first_transformation_id, result_dataframe_id, created_at, created_by, updated_at, updated_by, inputs",
        new_cell.id,
        new_cell.name,
        new_cell.input_dataframe_id,
        new_cell.input_dataset_id,
        new_cell.cell_type,
        new_cell_order,
        Option::<Uuid>::None, // first_transformation_id
        Option::<Uuid>::None, // result_dataframe_id
        Option::<NaiveDateTime>::None, // created_at
        Option::<Uuid>::None, // created_by
        Option::<NaiveDateTime>::None, // updated_at
        Option::<Uuid>::None,  // updated_by
        Option::<serde_json::Value>::None // inputs
    )
    .fetch_one(pool)
    .await?;

    Ok(inserted_cell)
}
