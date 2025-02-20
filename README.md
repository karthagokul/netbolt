# NetBolt 
## Ultrafast Network Speed Test Tool Written in Rust

NetBolt is a high-performance command-line tool that measures network download and upload speeds using asynchronous HTTP requests. It is built with Rust for speed, efficiency, and reliability.

## 🌟 Features
- **Download Speed Test:** Measures download speed using a 100MB file from Cloudflare.
- **Upload Speed Test:** Measures upload speed using Postman Echo.
- **Cross-Platform:** Works on Windows, macOS, and Linux.

## Installation
### Prerequisites
- Ensure [Rust](https://www.rust-lang.org/) is installed.

```bash
# Clone the repository
git clone https://github.com/karthagokul/netbolt.git
cd netbolt

# Build the project
cargo build --release

# Run the tool
cargo run
```
## Usage
```bash
# Basic usage
cargo run

# Run release version for better performance
./target/release/netbolt
```
## Dependencies
- **tokio:** For asynchronous execution
- **reqwest:** For HTTP requests
- **indicatif:** For progress bars
- **chrono:** For time measurement
- **futures:** For asynchronous streaming

## Project Structure
```plaintext
netbolt
├── Cargo.toml
└── src
    ├── main.rs
    └── speed_test.rs
```
## Contribution
Contributions are welcome! Feel free to open an issue or submit a pull request.

1. Fork the repository.
2. Create a new branch.
3. Make your changes.
4. Submit a pull request.

