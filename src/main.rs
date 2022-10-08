use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author = "Engin Diri", version, long_about = None)]
/// A very, very simple Hello World application
struct Args {
    #[clap(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Greet someone
    Greet {
        /// Name of the person to greet
        #[clap(default_value = "Unknown")]
        name: String,
    },
}

fn main() {
    let args = Args::parse();
    match args.cmd {
        Commands::Greet { name } => println!("Hello, {}!", name),
    }
}
