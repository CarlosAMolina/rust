use dotenv;
use std::env;

fn main() {
    // Initialize the .env file via the dotenv crate.
    dotenv::dotenv().ok();
    if let Err(_) = env::var("HOST") {
        panic!("HOST key not set");
    }
    if let Err(_) = env::var("PORT") {
        panic!("PORT key not set");
    }
    let host = std::env::var("HOST").unwrap();
    let port = std::env::var("PORT")
        .ok()
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(8080))
        .unwrap();
    println!(
        "{}",
        format!("Dabase host: {}. Dabase port: {}", host, port)
    )
}
