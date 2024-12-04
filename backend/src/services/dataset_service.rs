use log::{error, info};
use reqwest::Client;
use std::{error::Error, path::PathBuf};
use tokio::fs;
use uuid::Uuid;

pub async fn upload_to_supabase(
    file_name: String,
    data: Vec<u8>,
) -> Result<String, Box<dyn Error>> {
    // Retrieve Supabase configuration from environment variables
    let supabase_url =
        std::env::var("SUPABASE_URL").map_err(|_| "Environment variable SUPABASE_URL not found")?;
    let supabase_api_key = std::env::var("SUPABASE_API_KEY")
        .map_err(|_| "Environment variable SUPABASE_API_KEY not found")?;
    let bucket_name = std::env::var("SUPABASE_FILE_BUCKET_NAME")
        .map_err(|_| "Environment variable SUPABASE_FILE_BUCKET_NAME not found")?;

    // Define the endpoint URL
    let endpoint = format!(
        "{}/storage/v1/object/{}/{}",
        supabase_url, bucket_name, file_name
    );

    // Create the HTTP client
    let client = Client::new();

    // Make the request to upload the file
    let response = client
        .post(&endpoint)
        .bearer_auth(supabase_api_key)
        .header("Content-Type", "application/octet-stream")
        .body(data)
        .send()
        .await?;

    // Check the response status
    if response.status().is_success() {
        // Return the URL of the uploaded file
        let file_url = format!(
            "{}/storage/v1/object/public/{}/{}",
            supabase_url, bucket_name, file_name
        );
        Ok(file_url)
    } else {
        // Return an error if the upload failed
        let error_message = response.text().await?;
        Err(format!("Failed to upload file: {}", error_message).into())
    }
}

pub async fn download_dataset(dataset_url: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    info!(
        "Starting to download dataset for dataset_url: {}",
        dataset_url
    );
    let client = Client::new();

    let supabase_api_key = std::env::var("SUPABASE_API_KEY")
        .map_err(|_| "Environment variable SUPABASE_API_KEY not found")?;

    // Create a temporary file path for storing the dataset
    let temp_file_name = format!("dataset_{}.csv", Uuid::new_v4());
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join(temp_file_name);

    // Send a GET request to download the dataset
    let response = client
        .get(dataset_url)
        .bearer_auth(supabase_api_key)
        .header("Content-Type", "application/octet-stream")
        .send()
        .await?;

    // Extract the HTTP status before consuming the response
    let status = response.status();

    if !status.is_success() {
        let body = response.text().await?; // Consume the response here
        error!(
            "Failed to download dataset. HTTP Status: {}. Response body: {}",
            status, body
        );
        return Err(Box::from(format!(
            "Failed to download dataset. HTTP Status: {}",
            status
        )));
    }

    // Write the response to the file
    let content = response.bytes().await?;
    fs::write(&temp_path, &content).await?;

    Ok(temp_path)
}
