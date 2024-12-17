use crate::repositories::cell_repository::get_cell_metadata_by_id;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn check_and_create_dataframe_id(
    pool: &PgPool,
    cell_id: Uuid,
) -> Result<Uuid, sqlx::Error> {
    // Fetch the cell metadata from the database
    let cell = get_cell_metadata_by_id(pool, cell_id).await?;

    // Check if the cell exists
    match cell {
        Some(cell_row) => {
            // Check if the cell has a result_dataframe_id (optional)
            if let Some(result_dataframe_id) = cell_row.result_dataframe_id {
                // If the result_dataframe_id exists, return it
                Ok(result_dataframe_id)
            } else {
                // If result_dataframe_id does not exist, generate a new UUID
                let new_uuid = Uuid::new_v4();
                Ok(new_uuid)
            }
        }
        None => {
            // If no cell was found, generate and return a new UUID
            let new_uuid = Uuid::new_v4();
            Ok(new_uuid)
        }
    }
}
