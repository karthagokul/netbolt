/*!
 * NetBolt - Ultrafast Network Speed Test Tool written in Rust
 * 
 * Description: 
 * This program measures the download and upload speeds of a network 
 * using HTTP requests and displays the results with progress bars.
 * 
 * Author: Gokul Kartha <kartha.gokul@gmail.com>
 * Version: 1.0.0
 * License: MIT
 * Repository: https://github.com/karthagokul/netbolt
 *
 * Copyright (c) 2024 Gokul Kartha
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights 
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies 
 * of the Software, and to permit persons to whom the Software is furnished to do so, 
 * subject to the following conditions:
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, 
 * INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR 
 * PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE 
 * FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, 
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

 mod speed_test;

 #[tokio::main]
 async fn main() {
     // Retrieve version, name, and author information from Cargo.toml using environment variables
     let version = env!("CARGO_PKG_VERSION");     // Version from Cargo.toml
     let name = env!("CARGO_PKG_NAME");           // Package name from Cargo.toml
     let authors = env!("CARGO_PKG_AUTHORS");     // Authors from Cargo.toml
 
     println!("\nWelcome to {} Version {}", name, version);
     println!("Developed by: {}", authors);
     println!("\nMeasuring network speed, please wait...");
 
     // Variables to store speed test results
     let download_result = speed_test::download_speed().await;
     let upload_result = speed_test::upload_speed().await;
 
     // Display results as a summary
     
     println!("");
 
     // Output download speed or error
     match download_result {
         Ok(speed) => println!("\n✅ Download Speed: {:.2} Mbps", speed),
         Err(e) => println!("\n❗ Download Speed Test Failed: {}", e),
     }
 
     // Output upload speed or error
     match upload_result {
         Ok(speed) => println!("✅ Upload Speed: {:.2} Mbps", speed),
         Err(e) => println!("❗ Upload Speed Test Failed: {}", e),
     }
 
     println!("");
 }
 