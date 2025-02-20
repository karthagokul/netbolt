mod speed_test;

#[tokio::main]
async fn main() {
    // Get version info from Cargo environment variables
    let version = env!("CARGO_PKG_VERSION");     // Version from Cargo.toml
    let name = env!("CARGO_PKG_NAME");           // Package name from Cargo.toml
    let authors = env!("CARGO_PKG_AUTHORS");     // Authors from Cargo.toml

    println!("\nWelcome to {} Version {}", name, version);
    println!("Developed by: {}", authors);
    println!("");

    // Variables to store speed results
    let download_result = speed_test::download_speed().await;
    let upload_result = speed_test::upload_speed().await;

    // Display results as a summary
    println!("");

    match download_result {
        Ok(speed) => println!("\nDownload Speed: {:.2} Mbps", speed),
        Err(e) => println!("\n❗Download Speed Test Failed: {}", e),
    }

    match upload_result {
        Ok(speed) => println!("\nUpload Speed: {:.2} Mbps", speed),
        Err(e) => println!("\n❗Upload Speed Test Failed: {}", e),
    }

    println!("");
    println!("Test Completed. Thank you for using {}!", name);
}
