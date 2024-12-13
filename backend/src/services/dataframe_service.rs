use log::{error, info};
use polars::{frame::DataFrame, io::SerReader};
use reqwest::Client;
use std::{error::Error, fs::File};
use tokio::fs;
use uuid::Uuid;
use polars::io::parquet::ParquetReader;

pub async fn save_dataframe_to_supabase(parquet_file_path: String) -> Result<(Uuid, String), Box<dyn Error>> {
    let dataframe_id = Uuid::new_v4(); // Generate a unique ID for the DataFrame

    let supabase_url =
        std::env::var("SUPABASE_URL").map_err(|_| "Environment variable SUPABASE_URL not found")?;
    let supabase_api_key = std::env::var("SUPABASE_API_KEY")
        .map_err(|_| "Environment variable SUPABASE_API_KEY not found")?;
    let bucket_name = std::env::var("SUPABASE_DATAFRAME_BUCKET_NAME")
        .map_err(|_| "Environment variable SUPABASE_DATAFRAME_BUCKET_NAME not found")?;

    let parquet_data = fs::read(&parquet_file_path).await?; // Read the Parquet file
    let object_path = format!("{}.parquet", dataframe_id);

    // Define the endpoint URL
    let endpoint = format!(
        "{}/storage/v1/object/{}/{}",
        supabase_url, bucket_name, object_path
    );

    // Create the HTTP client
    let client = Client::new();

    // Make the request to upload the file
    let response = client
        .post(&endpoint)
        .bearer_auth(supabase_api_key)
        .header("Content-Type", "application/octet-stream")
        .body(parquet_data)
        .send()
        .await?;

    // Check the response status
    if response.status().is_success() {
        // Step 3: Clean up temporary file
        let _ = fs::remove_file(&parquet_file_path).await;

        Ok((dataframe_id, object_path))
    } else {
        // Return an error if the upload failed
        let error_message = response.text().await?;
        Err(format!("Failed to upload file: {}", error_message).into())
    }
}

// Utility function to download the Parquet file from Supabase
pub async fn download_parquet_from_supabase(dataframe_id: Uuid, file_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    info!("Downloading parquet file");

    let supabase_url =
        std::env::var("SUPABASE_URL").map_err(|_| "Environment variable SUPABASE_URL not found")?;
    let supabase_api_key = std::env::var("SUPABASE_API_KEY")
        .map_err(|_| "Environment variable SUPABASE_API_KEY not found")?;
    let bucket_name = std::env::var("SUPABASE_DATAFRAME_BUCKET_NAME")
        .map_err(|_| "Environment variable SUPABASE_DATAFRAME_BUCKET_NAME not found")?;

    let supabase_file_url = format!(
        "{}/storage/v1/object/{}/{}",
        supabase_url, bucket_name, file_url
    );

    let temp_file_path = format!("/tmp/{}", dataframe_id);
    let file_data = reqwest::get(supabase_file_url).await?.bytes().await?;
    let _ = fs::write(&temp_file_path, file_data).await;

    info!("Successfully downloaded parquet file");

    Ok(temp_file_path)
}

// Utility function to read the Parquet file into a Polars DataFrame
pub fn read_parquet_to_dataframe(file_path: &str) -> Result<DataFrame, Box<dyn std::error::Error>> {
    info!("Read parquet file to dataframe");
    let file = File::open(file_path)?;
    let df = ParquetReader::new(file).finish()?;
    info!("Successfully read parquet file to dataframe");
    Ok(df)
}