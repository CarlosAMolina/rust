use config::Config;

#[derive(serde::Deserialize)]
struct Args {
    /// URL for the postgres database
    database_host: String,
    /// PORT number for the database connection
    database_port: u16,
}

fn main() {
    // It is not required to specify the .toml extension when parsing the file.
    let config = Config::builder()
        .add_source(config::File::with_name("setup"))
        .build()
        .unwrap();
    let config = config.try_deserialize::<Args>().unwrap();
    println!(
        "{}",
        format!(
            "Dabase host: {}. Dabase port: {}",
            config.database_host, config.database_port
        )
    )
}
