use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version)] // Set available option `-V`.
struct Cli {
    search_term: Option<Vec<String>>,

    #[arg(short, long)]
    id: Option<i32>,

    #[arg(short, long, value_enum)]
    format: Option<Format>,
}

#[derive(Clone, Debug, ValueEnum)]
enum Format {
    /// Show all contact info.
    Long,
    /// Show a summary of the contact info.
    Short,
}

fn main() {
    let cli = Cli::parse();

    let search_term = cli.search_term.as_ref().unwrap().join(" ");
    println!("search term: {}", search_term);
    println!("id: {:?}", cli.id);
    println!("format: {:?}", cli.format);
    match cli.format {
        Some(Format::Long) => {
            println!("Long format");
        }
        Some(Format::Short) => {
            println!("Short format");
        }
        None => println!("Default value"),
    }
}
