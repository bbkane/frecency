use std::path::PathBuf;

fn main() -> rusqlite::Result<()> {
    let path = PathBuf::from("db/tmp.db");
    let _ = std::fs::remove_file(&path);
    let _conn = frecency::db::connect(&path)?;
    println!("Created {path:?}");
    Ok(())
}
