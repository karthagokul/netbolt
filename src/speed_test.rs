use reqwest::blocking::Client;
use chrono::Utc;
use std::error::Error;

const DOWNLOAD_URL: &str = "https://speed.hetzner.de/100MB.bin"; // Test file for download
const UPLOAD_URL: &str = "https://httpbin.org/post"; // For testing upload

/// Measure download speed in Mbps
pub async fn download_speed() -> Result<f64, Box<dyn Error>> {
    let start_time = Utc::now();
    let response = reqwest::get(DOWNLOAD_URL).await?.bytes().await?;
    let end_time = Utc::now();

    let duration = (end_time - start_time).num_milliseconds() as f64 / 1000.0;
    let size_mb = response.len() as f64 / (1024.0 * 1024.0); // Bytes to Megabytes
    let speed_mbps = (size_mb * 8.0) / duration; // Megabits per second

    Ok(speed_mbps)
}

/// Measure upload speed in Mbps
pub async fn upload_speed() -> Result<f64, Box<dyn Error>> {
    let data = vec![0u8; 10 * 1024 * 1024]; // 10MB of data to upload
    let client = reqwest::Client::new(); // Async client

    

    let start_time = Utc::now();
    let response = client.post(UPLOAD_URL)
    .body(data)
    .send()
    .await?;
    let end_time = Utc::now();

    if response.status().is_success() {
        let duration = (end_time - start_time).num_milliseconds() as f64 / 1000.0;
        let size_mb = 10.0; // 10MB uploaded
        let speed_mbps = (size_mb * 8.0) / duration; // Megabits per second
        Ok(speed_mbps)
    } else {
        Err("Upload failed".into())
    }
}
