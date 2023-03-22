
mod config;
mod errors;

#[tokio::main]
async fn main() -> Result<(), errors::Error> {
    dotenv::dotenv().ok();
    let config = config::Config::new().expect("Invalid configuration");

    println!("Hello, world!");

    Ok(())
}
