use std::{error::Error, path::PathBuf};

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use frecency::{commands, db};
#[allow(warnings)]
mod queries;

fn default_db_path() -> PathBuf {
    PathBuf::from(std::env::var_os("HOME").expect("HOME environment variable is not set"))
        .join(".config")
        .join("frecency.db")
}

#[derive(Parser)]
struct Cli {
    #[arg(long, global = true, default_value_os_t = default_db_path())]
    db_path: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Add item")]
    Add(commands::AddArgs),
    #[command(about = "List items and frecency scores")]
    Query(commands::QueryArgs),

    // generic commands
    #[command(about = "Print shell completion scripts")]
    Completion { shell: Shell },
    #[command(about = "Print version")]
    Version,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    if let Some(parent) = args.db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut conn = db::connect(&args.db_path).expect("coud not open db");
    let mut tx = conn.transaction()?;

    match args.command {
        Commands::Add(args) => commands::add(&mut tx, args)?,
        Commands::Query(args) => commands::query(&mut tx, args)?,

        // generic commands
        Commands::Completion { shell } => generate(
            shell,
            &mut Cli::command(),
            env!("CARGO_PKG_NAME"),
            &mut std::io::stdout(),
        ),
        Commands::Version => println!("v{}", env!("CARGO_PKG_VERSION")),
    }
    tx.commit()?;
    Ok(())
}
