use std::error::Error;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use frecency::commands;
#[allow(warnings)]
mod queries;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Print version")]
    Version,

    #[command(about = "Say hello")]
    Add(commands::AddArgs),

    #[command(about = "Print shell completion scripts")]
    Completion { shell: Shell },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    match &args.command {
        Commands::Version => {
            println!("v{}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        Commands::Add(args) => {
            println!("Hello there {}!", args.key);

            println!("Update time: {}", args.update_time.as_second());

            let mut conn = rusqlite::Connection::open_in_memory()?;
            frecency::db::migrate(&mut conn)?;

            let items = queries::QueryItems::builder()
                .term(&args.key)
                .build()
                .query_many(&conn)?;
            println!("matching items: {}", items.len());
            Ok(())
        }
        Commands::Completion { shell } => {
            generate(
                *shell,
                &mut Cli::command(),
                env!("CARGO_PKG_NAME"),
                &mut std::io::stdout(),
            );
            Ok(())
        }
    }
}
