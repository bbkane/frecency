fn main() -> rusqlite::Result<()> {
    let path = "db/tmp.db";
    let _ = std::fs::remove_file(path);
    let _conn = frecency::db::connect(path)?;
    println!("Created {path}");
    Ok(())
}
