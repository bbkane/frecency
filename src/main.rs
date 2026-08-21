use std::error::Error;

use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};

#[allow(warnings)]
mod queries;

#[derive(Args)]
struct Add {
    #[arg(long)]
    key: String,
}

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
    Add(Add),

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

            let mut conn = rusqlite::Connection::open_in_memory()?;
            frecency::migrate(&mut conn)?;

            let authors = queries::ListAuthors.query_many(&conn)?;
            println!("authors before insert: {}", authors.len());

            let affected_rows = queries::CreateAuthor::builder()
                .name("Brian Kernighan")
                .bio(Some("Co-author of The C Programming Language"))
                .build()
                .execute(&conn)?;
            println!("inserted rows: {affected_rows}");

            let author = queries::GetAuthor::builder()
                .id(conn.last_insert_rowid())
                .build()
                .query_one(&conn)?;

            println!("author: {} {:?}", author.name, author.bio);
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
