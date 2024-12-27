use crate::{
    dto::cell::{AddCellToPositionRequest, CreateCellRequest},
    models::cell::Cell,
    repositories::cell_repository,
};
use actix_web::{
    web::{self},
    Error, HttpRequest, HttpResponse,
};
use log::{info, warn};
use sqlx::PgPool;
use uuid::Uuid;

use super::transformation::apply_transformation;

pub async fn get_cells(pool: web::Data<PgPool>, _req: HttpRequest) -> Result<HttpResponse, Error> {
    // Query the dataset
    let cells = cell_repository::get_cells(&pool).await.map_err(|e| {
        // Convert the error to an Actix-compatible error
        actix_web::error::ErrorInternalServerError(format!("Failed to retrieve cells: {}", e))
    })?;

    // Return the cells as a JSON response
    Ok(HttpResponse::Ok().json(cells))
}

pub async fn get_cell_by_id(
    pool: web::Data<PgPool>,
    id: web::Path<Uuid>,
    _req: HttpRequest,
) -> Result<HttpResponse, Error> {
    info!("Starting to retrieve cell by id: {}", id);
    let cell_id = id.into_inner();
    // Query the dataset
    let cell = cell_repository::get_cell_by_id(&pool, cell_id)
        .await
        .map_err(|e| {
            // Convert the error to an Actix-compatible error
            actix_web::error::ErrorInternalServerError(format!("Failed to retrieve cell: {}", e))
        })?;

    info!("Successfully retrieved cell");
    // Return the cells as a JSON response
    Ok(HttpResponse::Ok().json(cell))
}

pub async fn create_cell(
    pool: web::Data<PgPool>,
    payload: web::Json<CreateCellRequest>,
) -> Result<HttpResponse, Error> {
    info!("Creating a new cell");
    let new_cell = Cell::new(
        Uuid::new_v4(),
        None,
        payload.input_dataframe_id,
        payload.input_dataset_id,
        None,
        payload.cell_type.clone(),
        payload.cell_order.clone(),
        payload.name.clone(),
        None,
        None,
        None,
    );

    let created_cell = cell_repository::create_cell(&pool, &new_cell)
        .await
        .map_err(|e| {
            warn!("Failed to create cell into database: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Failed to create cell: {}", e))
        })?;

    Ok(HttpResponse::Ok().json(created_cell))
}

pub async fn add_cell_at_position(
    pool: web::Data<PgPool>,
    payload: web::Json<AddCellToPositionRequest>,
) -> Result<HttpResponse, Error> {
    info!("Adding a new cell");
    let new_cell = Cell::new(
        Uuid::new_v4(),
        None,
        payload.cell.input_dataframe_id,
        payload.cell.input_dataset_id,
        None,
        payload.cell.cell_type.clone(),
        payload.cell.cell_order.clone(),
        payload.cell.name.clone(),
        None,
        None,
        None,
    );

    let created_cell = cell_repository::add_cell_at_position(
        &pool,
        payload.reference_cell_id,
        payload.cell_order,
        new_cell,
    )
    .await
    .map_err(|e| {
        warn!("Failed to add cell into database: {}", e);
        actix_web::error::ErrorInternalServerError(format!("Failed to add cell: {}", e))
    })?;

    Ok(HttpResponse::Ok().json(created_cell))
}

pub fn cell_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cells")
            .route("", web::get().to(get_cells))
            .route("", web::post().to(create_cell))
            .route(
                "/add-cell-to-position",
                web::post().to(add_cell_at_position),
            )
            .route("/{id}", web::get().to(get_cell_by_id))
            .route(
                "/{cell_id}/apply-transformation",
                web::post().to(apply_transformation),
            ),
    );
}
