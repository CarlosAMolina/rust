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

    let format;

    if let Some(id) = cli.id {
        println!("Init id: {}", id);
        format = "long";
    } else {
        let search_term = cli.search_term.as_ref().unwrap().join(" ");
        println!("Init search term {}", search_term);
        format = match cli.format {
            Some(Format::Long) => {
                "long"
            }
            _ => {
                "short"
            }
        };
    }
    println!("Format: {}", format);
}
