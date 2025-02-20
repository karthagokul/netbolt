/*!
 * NetBolt - Ultrafast Network Speed Test Tool written in Rust
 *
 * This module contains the core functionality for measuring download 
 * and upload speeds using asynchronous HTTP requests. Both download 
 * and upload processes display a progress bar using the `indicatif` crate.
 *
 * Functions:
 * - download_speed(): Measures and returns the download speed in Mbps.
 * - upload_speed(): Measures and returns the upload speed in Mbps.
 *
 * Author: Gokul Kartha <kartha.gokul@gmail.com>
 * Version: 1.0.0
 * License: MIT
 */

 use chrono::Utc;  // Used to measure elapsed time
 use reqwest::Client;  // Asynchronous HTTP client for sending requests
 use indicatif::{ProgressBar, ProgressStyle};  // For displaying progress bars
 use futures::StreamExt;  // To stream download data in chunks
 use std::error::Error;  // For error handling
 
 // Constant URLs for download and upload tests
 const DOWNLOAD_URL: &str = "https://speed.cloudflare.com/__down?bytes=100000000"; // 100MB file from Cloudflare
 const UPLOAD_URL: &str = "https://postman-echo.com/post"; // Endpoint for upload test
 
 
 /// Measures the network download speed in Mbps with a progress bar.
 ///
 /// This asynchronous function downloads a 100MB file from Cloudflare, displays
 /// a real-time progress bar using `indicatif`, and calculates the download speed.
 ///
 /// # Returns
 /// - `Ok(f64)` with download speed in Mbps if successful.
 /// - `Err(Box<dyn Error>)` if an error occurs during the download.
 ///
 /// # Example
 /// ```rust
 /// let speed = download_speed().await.unwrap();
 /// println!("Download Speed: {:.2} Mbps", speed);
 /// ```
 pub async fn download_speed() -> Result<f64, Box<dyn Error>> {
     // Create a new HTTP client
     let client = Client::new();
     let start_time = Utc::now(); // Record the start time
 
     // Send GET request to download the file
     let response = client.get(DOWNLOAD_URL).send().await?;
     let total_size = response.content_length().unwrap_or(100_000_000); // Use 100MB if unknown
 
     // Initialize the progress bar with total file size
     let pb = ProgressBar::new(total_size);
     pb.set_style(
         ProgressStyle::default_bar()
             .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
             .unwrap()
             .progress_chars("#>-"),
     );
 
     // Stream the response and update progress bar for each chunk
     let mut stream = response.bytes_stream();
     let mut downloaded_bytes = 0;
 
     while let Some(chunk) = stream.next().await {
         let chunk = chunk?;
         downloaded_bytes += chunk.len() as u64;
         pb.set_position(downloaded_bytes);
     }
 
     pb.finish_with_message("Download complete!"); // Mark progress bar as complete
 
     let end_time = Utc::now(); // Record the end time
     let duration = (end_time - start_time).num_milliseconds() as f64 / 1000.0; // Calculate duration in seconds
 
     // Calculate download speed in Mbps (Megabits per second)
     let size_mb = total_size as f64 / (1024.0 * 1024.0); // Bytes to MB
     let speed_mbps = (size_mb * 8.0) / duration; // Convert MBps to Mbps
 
     Ok(speed_mbps)
 }
 
 
 /// Measures the network upload speed in Mbps with a progress bar.
 ///
 /// This asynchronous function uploads a 10MB payload to `https://postman-echo.com/post`,
 /// simulates the upload progress using a delay, and calculates the upload speed.
 ///
 /// # Returns
 /// - `Ok(f64)` with upload speed in Mbps if successful.
 /// - `Err(Box<dyn Error>)` if an error occurs during the upload.
 ///
 /// # Example
 /// ```rust
 /// let speed = upload_speed().await.unwrap();
 /// println!("Upload Speed: {:.2} Mbps", speed);
 /// ```
 pub async fn upload_speed() -> Result<f64, Box<dyn Error>> {
     // Prepare 10MB of data to be uploaded
     let data = vec![0u8; 10 * 1024 * 1024]; // 10MB
     let client = Client::new();
 
     let total_size = data.len() as u64;
 
     // Initialize the progress bar with total data size
     let pb = ProgressBar::new(total_size);
     pb.set_style(
         ProgressStyle::default_bar()
             .template("{spinner:.green} [{elapsed_precise}] [{bar:40.yellow/blue}] {bytes}/{total_bytes} ({eta})")
             .unwrap()
             .progress_chars("#>-"),
     );
 
     let start_time = Utc::now(); // Record the start time
 
     // Simulate upload progress (since reqwest doesn't support real streaming progress)
     let chunk_size = 1024 * 1024; // 1MB per chunk
     let mut uploaded_bytes = 0;
 
     for chunk in data.chunks(chunk_size) {
         uploaded_bytes += chunk.len() as u64;
         pb.set_position(uploaded_bytes);
         tokio::time::sleep(tokio::time::Duration::from_millis(100)).await; // Simulate delay for progress visualization
     }
 
     // Perform the actual upload request
     let response = client.post(UPLOAD_URL).body(data).send().await?;
     let end_time = Utc::now(); // Record the end time
 
     pb.finish_with_message("Upload complete!"); // Mark progress bar as complete
 
     // Check if the response is successful
     if response.status().is_success() {
         let duration = (end_time - start_time).num_milliseconds() as f64 / 1000.0; // Calculate duration in seconds
         let size_mb = 10.0; // Uploaded 10MB
         let speed_mbps = (size_mb * 8.0) / duration; // Convert MBps to Mbps
         Ok(speed_mbps)
     } else {
         Err("Upload failed".into()) // Return error if upload was not successful
     }
 }
 