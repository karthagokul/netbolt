use chrono::Utc;
use reqwest::Client;
use indicatif::{ProgressBar, ProgressStyle};
use futures::StreamExt;
use std::error::Error;

const DOWNLOAD_URL: &str = "https://speed.cloudflare.com/__down?bytes=100000000"; // 100MB file from Cloudflare
const UPLOAD_URL: &str = "https://postman-echo.com/post";


/// Measure download speed in Mbps with progress bar
pub async fn download_speed() -> Result<f64, Box<dyn Error>> {
    let client = Client::new();
    let start_time = Utc::now();

    let response = client.get(DOWNLOAD_URL).send().await?;
    let total_size = response.content_length().unwrap_or(100_000_000); // Default to 100MB if unknown

    // Progress bar setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Stream response and update progress bar
    let mut stream = response.bytes_stream();
    let mut downloaded_bytes = 0;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        downloaded_bytes += chunk.len() as u64;
        pb.set_position(downloaded_bytes);
    }

    pb.finish_with_message("Download complete!");

    let end_time = Utc::now();
    let duration = (end_time - start_time).num_milliseconds() as f64 / 1000.0;

    let size_mb = total_size as f64 / (1024.0 * 1024.0); // Bytes to MB
    let speed_mbps = (size_mb * 8.0) / duration; // Megabits per second

    Ok(speed_mbps)
}

/// Measure upload speed in Mbps with progress bar
pub async fn upload_speed() -> Result<f64, Box<dyn Error>> {
    let data = vec![0u8; 10 * 1024 * 1024]; // 10MB of data to upload
    let client = Client::new();

    let total_size = data.len() as u64;

    // Progress bar setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.yellow/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let start_time = Utc::now();

    // Simulate upload progress (reqwest doesn't provide real streaming progress for uploads)
    let chunk_size = 1024 * 1024; // 1MB
    let mut uploaded_bytes = 0;

    for chunk in data.chunks(chunk_size) {
        uploaded_bytes += chunk.len() as u64;
        pb.set_position(uploaded_bytes);
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await; // Simulate network delay
    }

    let response = client.post(UPLOAD_URL).body(data).send().await?;
    let end_time = Utc::now();

    pb.finish_with_message("Upload complete!");

    if response.status().is_success() {
        let duration = (end_time - start_time).num_milliseconds() as f64 / 1000.0;
        let size_mb = 10.0; // 10MB uploaded
        let speed_mbps = (size_mb * 8.0) / duration;
        Ok(speed_mbps)
    } else {
        Err("Upload failed".into())
    }
}
