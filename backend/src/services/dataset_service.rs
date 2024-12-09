use reqwest::Client;
use std::error::Error;

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
