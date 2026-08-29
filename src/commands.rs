use crate::queries;
use clap::Args;
use jiff::Timestamp;
use rusqlite::Connection;
use std::error::Error;

#[derive(Args)]
pub struct AddArgs {
    #[arg(long)]
    pub key: String,

    #[arg(long, default_value_t = 0)]
    pub base_score: i64,

    #[arg(long, default_value_t = Timestamp::now())]
    pub create_time: Timestamp,

    #[arg(long, default_value_t = Timestamp::now())]
    pub update_time: Timestamp,
}

pub fn add(conn: &mut Connection, args: AddArgs) -> Result<(), Box<dyn Error>> {
    let tx = conn.transaction()?;

    let row = queries::InsertOrUpdateItem::builder()
        .item(&args.key)
        .base_score(args.base_score)
        .create_time(args.create_time.as_second())
        .update_time(args.update_time.as_second())
        .build()
        .query_one(&tx)?;

    let id = row.id;
    queries::InsertIntoAccessLog::builder()
        .item_id(id)
        .access_time(args.create_time.as_second())
        .build()
        .execute(&tx)?;

    tx.commit()?;

    Ok(())
}
