mod migrations;

use rusqlite::{Connection, Result};

pub fn migrate(conn: &mut Connection) -> Result<()> {
    migrations::run(conn)
}

pub fn connect(path: &str) -> Result<Connection> {
    let mut conn = Connection::open(path)?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    migrate(&mut conn)?;
    Ok(conn)
}
