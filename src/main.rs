mod speed_test;

#[tokio::main]
async fn main() {
    println!("🌐 Network Speed Tester in Rust 🌐");

    match speed_test::download_speed().await {
        Ok(speed) => println!("⬇️ Download Speed: {:.2} Mbps", speed),
        Err(e) => println!("❗ Error measuring download speed: {}", e),
    }

    match speed_test::upload_speed().await {
        Ok(speed) => println!("⬆️ Upload Speed: {:.2} Mbps", speed),
        Err(e) => println!("❗ Error measuring upload speed: {}", e),
    }
}
