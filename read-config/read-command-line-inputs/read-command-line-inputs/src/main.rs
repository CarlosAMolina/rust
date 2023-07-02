use clap::Parser;

// Docs comments will be shown when the user uses the help command.
/// Q&A web service API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Each field can be transformed automatically in a CLI argument with the short and long keywords, as well as set a default value in case the option is not specified when starting the application.
    /// URL for the postgres database
    #[clap(short, long, default_value = "localhost")]
    host_database: String,
    /// PORT number for the database connection
    #[clap(long, default_value = "8080")]
    port_database: u16,
}

fn main() {
    let args = Args::parse();
    println!(
        "{}",
        format!(
            "Dabase host: {}. Dabase port: {}",
            args.host_database, args.port_database
        )
    )
}
