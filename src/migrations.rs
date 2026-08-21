use rusqlite::{Connection, Result, params};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "db/migrations/"]
struct MigrationFiles;

pub fn run(conn: &mut Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS migration (
            id INTEGER PRIMARY KEY,
            file_name TEXT NOT NULL UNIQUE,
            migrate_time TEXT NOT NULL
        ) STRICT;",
    )?;

    let mut file_names: Vec<String> = MigrationFiles::iter().map(|f| f.into_owned()).collect();
    file_names.sort();

    for file_name in file_names {
        let already_applied: bool = conn.query_row(
            "SELECT COUNT(*) FROM migration WHERE file_name = ?1",
            params![file_name],
            |row| row.get::<_, i64>(0),
        )? > 0;

        if already_applied {
            continue;
        }

        let file = MigrationFiles::get(&file_name).expect("embedded file missing");
        let sql = std::str::from_utf8(file.data.as_ref()).expect("migration is not valid UTF-8");

        let tx = conn.transaction()?;
        tx.execute_batch(sql)?;
        tx.execute(
            "INSERT INTO migration (file_name, migrate_time) VALUES (?1, datetime('now'))",
            params![file_name],
        )?;
        tx.commit()?;
    }

    Ok(())
}
