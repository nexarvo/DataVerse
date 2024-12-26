use calamine::Reader;
use csv::ReaderBuilder;
use log::{error, info};
use parquet::basic::{Compression, ConvertedType, Encoding, LogicalType, Type as ParquetType};
use parquet::data_type::{
    BoolType, ByteArray, ByteArrayType, DoubleType, FixedLenByteArray, FixedLenByteArrayType,
    FloatType, Int32Type, Int64Type,
};
use parquet::file::writer::SerializedRowGroupWriter;
use parquet::record::{Row, RowAccessor};
use parquet::schema::types::ColumnPath;
use parquet::{
    file::properties::WriterProperties, file::writer::SerializedFileWriter, schema::types::Type,
};
use polars::prelude::CsvReader;
use polars::prelude::*;
use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use std::io::{Cursor, Write};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tokio::fs;
use uuid::Uuid;

pub async fn upload_to_supabase(
    file_name: String,
    data: Vec<u8>,
) -> Result<String, Box<dyn Error>> {
    info!("Starting to upload file: {}", file_name);

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
        info!("Successfully uploaded file: {}", file_url);
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

fn infer_column_type(values: &[String]) -> ParquetType {
    let mut has_float = false;
    let mut has_int = false;
    let mut has_bool = false;

    for v in values {
        if v.is_empty() {
            continue;
        }
        if v == "true" || v == "false" {
            has_bool = true;
        } else if v.parse::<i64>().is_ok() {
            has_int = true;
        } else if v.parse::<f64>().is_ok() {
            has_float = true;
        } else {
            return ParquetType::BYTE_ARRAY; // Default to string if any value is non-numeric
        }
    }

    if has_bool {
        ParquetType::BOOLEAN
    } else if has_float {
        ParquetType::FLOAT
    } else if has_int {
        ParquetType::INT32
    } else {
        ParquetType::BYTE_ARRAY
    }
}

pub fn convert_csv_to_parquet(file_data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    println!("Converting CSV data to Parquet");

    // Step 1: Parse CSV data into records
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(Cursor::new(file_data));
    let headers = rdr.headers()?.clone();
    let records: Vec<Vec<String>> = rdr
        .records()
        .filter_map(|record| record.ok())
        .map(|record| record.iter().map(|s| s.to_string()).collect())
        .collect();

    // Step 2: Define Parquet schema based on CSV headers
    let fields: Result<Vec<Arc<Type>>, Box<dyn Error>> = headers
        .iter()
        .map(|h| {
            // Safely find the index of the header
            let header_index = headers
                .iter()
                .position(|header| header == h)
                .ok_or_else(|| format!("Header '{}' not found in CSV headers.", h))?;

            // Collect values for the column
            let column_values: Vec<String> = records
                .iter()
                .map(|record| record.get(header_index).unwrap_or(&"".to_string()).clone())
                .collect();

            // Infer the column type
            let parquet_type = infer_column_type(&column_values);

            // Build the Parquet type safely
            Type::primitive_type_builder(h, parquet_type)
                .with_converted_type(if matches!(parquet_type, ParquetType::BYTE_ARRAY) {
                    ConvertedType::UTF8
                } else {
                    ConvertedType::NONE
                }) // Logical type remains UTF8 for strings
                .with_length(-1)
                .with_repetition(parquet::basic::Repetition::OPTIONAL)
                .build()
                .map(Arc::new)
                .map_err(|e| Box::new(e) as Box<dyn Error>)
        })
        .collect();

    // Handle possible errors
    let fields = match fields {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error building fields: {}", e);
            return Err(e); // Or handle it gracefully as needed
        }
    };

    let schema = Type::group_type_builder("schema")
        .with_fields(fields.into_iter().collect())
        .build()
        .unwrap();

    // Step 3: Create Parquet writer with specific encoding
    let mut buffer = Vec::new();
    let writer = Cursor::new(&mut buffer);
    let props = WriterProperties::builder()
        .set_encoding(Encoding::PLAIN) // Use PLAIN encoding for better compatibility
        .set_compression(Compression::SNAPPY) // Optional: Use SNAPPY compression
        .build();

    let mut parquet_writer = SerializedFileWriter::new(writer, Arc::new(schema), Arc::new(props))?;

    // Step 4: Write data column by column
    let mut row_group_writer = parquet_writer.next_row_group()?;

    for col_idx in 0..headers.len() {
        // Retrieve the column writer
        if let Some(mut column_writer) = row_group_writer.next_column()? {
            let parquet_type = infer_column_type(
                &records
                    .iter()
                    .map(|r| r[col_idx].clone())
                    .collect::<Vec<_>>(),
            );

            match parquet_type {
                ParquetType::BOOLEAN => {
                    let row_writer = column_writer.typed::<BoolType>();
                    let column_values: Vec<bool> = records
                        .iter()
                        .map(|record| record[col_idx].parse::<bool>().unwrap_or(false))
                        .collect();
                    let definition_levels = vec![1; column_values.len()];
                    row_writer.write_batch(&column_values, Some(&definition_levels), None)?;
                }
                ParquetType::INT32 => {
                    let row_writer = column_writer.typed::<Int32Type>();
                    let column_values: Vec<i32> = records
                        .iter()
                        .map(|record| record[col_idx].parse::<i32>().unwrap_or(0))
                        .collect();
                    let definition_levels = vec![1; column_values.len()];
                    row_writer.write_batch(&column_values, Some(&definition_levels), None)?;
                }
                ParquetType::INT64 => {
                    let row_writer = column_writer.typed::<Int64Type>();
                    let column_values: Vec<i64> = records
                        .iter()
                        .map(|record| record[col_idx].parse::<i64>().unwrap_or(0))
                        .collect();
                    let definition_levels = vec![1; column_values.len()];
                    row_writer.write_batch(&column_values, Some(&definition_levels), None)?;
                }
                ParquetType::FLOAT => {
                    let row_writer = column_writer.typed::<FloatType>();
                    let column_values: Vec<f32> = records
                        .iter()
                        .map(|record| record[col_idx].parse::<f32>().unwrap_or(0.0))
                        .collect();
                    let definition_levels = vec![1; column_values.len()];
                    row_writer.write_batch(&column_values, Some(&definition_levels), None)?;
                }
                ParquetType::DOUBLE => {
                    let row_writer = column_writer.typed::<DoubleType>();
                    let column_values: Vec<f64> = records
                        .iter()
                        .map(|record| record[col_idx].parse::<f64>().unwrap_or(0.0))
                        .collect();
                    let definition_levels = vec![1; column_values.len()];
                    row_writer.write_batch(&column_values, Some(&definition_levels), None)?;
                }
                ParquetType::BYTE_ARRAY => {
                    let row_writer = column_writer.typed::<ByteArrayType>();
                    let column_values: Vec<ByteArray> = records
                        .iter()
                        .map(|record| ByteArray::from(record[col_idx].as_str()))
                        .collect();
                    let definition_levels = vec![1; column_values.len()];
                    row_writer.write_batch(&column_values, Some(&definition_levels), None)?;
                }
                ParquetType::FIXED_LEN_BYTE_ARRAY => {
                    let row_writer = column_writer.typed::<FixedLenByteArrayType>();
                    let column_values: Vec<FixedLenByteArray> = records
                        .iter()
                        .map(|record| FixedLenByteArray::from(record[col_idx].as_bytes().to_vec()))
                        .collect();
                    let definition_levels = vec![1; column_values.len()];
                    row_writer.write_batch(&column_values, Some(&definition_levels), None)?;
                }
                ParquetType::INT96 => {
                    // INT96 is a timestamp type used in Parquet, you can implement it similarly
                    // However, it requires parsing date/timestamps, so this can be added later as needed
                    // Here we just handle as dummy implementation
                    unimplemented!("INT96 type is not yet implemented");
                }
            }

            column_writer.close()?;
        }
    }

    // Close writers
    row_group_writer.close()?;
    parquet_writer.close()?;

    println!("Successfully converted CSV data to Parquet");
    Ok(buffer)
}

/// Generates metadata for the columns of a dataset (CSV or JSON format).
///
/// This function reads a file's content (CSV or JSON) and infers the schema of the dataset.
/// It extracts the column names and their corresponding data types (e.g., Utf8, Int64, Float64),
/// and returns the metadata as a JSON object.
///
/// # Arguments
///
/// * `file_data` - A byte slice representing the raw data of the file.
/// * `file_name` - The name of the file (used to determine the file type).
///
/// # Returns
///
/// This function returns a `Result` where:
/// * On success, it returns a `serde_json::Value::Object` containing column names as keys and their data types as values.
/// * On failure, it returns a `Box<dyn Error>`, indicating the error that occurred. This could be an unsupported file format or issues during the reading process.
///
/// # Errors
///
/// This function may return an error if:
/// * The file format is not CSV or JSON (based on the file extension).
/// * The file cannot be read properly or has invalid content.
///
/// # Example
///
/// ```rust
/// let file_data = b"Name,Age\nAlice,30\nBob,25";
/// let file_name = "data.csv";
/// let metadata = generate_columns_metadata(file_data, file_name);
/// assert!(metadata.is_ok());
/// let result = metadata.unwrap();
/// assert_eq!(result, serde_json::json!({ "Name": "Utf8", "Age": "Int64" }));
/// ```
///
/// # Supported File Formats
///
/// This function currently supports:
/// * CSV files (.csv)
/// * JSON files (.json)
///
/// # Notes
/// * The CSV format is assumed to have a header row that defines the column names.
/// * The JSON format is assumed to be an array of objects, with each object containing the same set of keys.
///
/// # Panics
/// This function may panic if the file content cannot be parsed into a valid schema or if an unsupported file type is provided.
///
pub fn generate_columns_metadata(
    file_data: &[u8],
    file_name: &str,
) -> Result<Value, Box<dyn Error>> {
    info!(
        "Generating metadata for dataset columns file: {}",
        file_name
    );
    // Determine the file type and read the dataset
    let df = if file_name.ends_with(".csv") {
        // Create a CsvReader instance
        let reader = CsvReader::new(Cursor::new(file_data))
            .infer_schema(Some(100))
            .has_header(true)
            .with_ignore_parser_errors(true);

        // Try reading the file, and handle any UTF-8 errors gracefully
        match reader.finish() {
            Ok(df) => df,
            Err(e) => {
                // Log and return the error without panicking
                return Err(Box::new(e));
            }
        }
    } else if file_name.ends_with(".json") {
        JsonReader::new(Cursor::new(file_data)).finish()?
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Unsupported file format",
        )));
    };

    // Generate metadata
    let metadata = df
        .get_column_names()
        .iter()
        .zip(df.dtypes().iter())
        .map(|(col_name, dtype)| (col_name.to_string(), Value::String(format!("{:?}", dtype))))
        .collect::<serde_json::Map<String, Value>>();

    info!(
        "Successfully generated metadata for dataset columns file: {}",
        file_name
    );
    Ok(Value::Object(metadata))
}
