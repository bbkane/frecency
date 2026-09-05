use crate::queries;
use clap::Args;
use jiff::Timestamp;
use rusqlite::Transaction;
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

pub fn add(tx: &mut Transaction<'_>, args: AddArgs) -> Result<(), Box<dyn Error>> {
    let id = queries::InsertOrUpdateItem::builder()
        .item(&args.key)
        .base_score(args.base_score)
        .create_time(args.create_time.as_second())
        .update_time(args.update_time.as_second())
        .build()
        .query_one(tx)?
        .id;

    queries::InsertIntoAccessLog::builder()
        .item_id(id)
        .access_time(args.create_time.as_second())
        .build()
        .execute(tx)?;

    Ok(())
}

#[derive(Args)]
pub struct QueryArgs {
    #[arg(long, default_value_t = 500)]
    pub limit: i64,

    #[arg(long, default_value_t = String::from(""))]
    pub prefix: String,

    #[arg(long, default_value_t = String::from("\t"))]
    pub sep: String,
}
pub fn query(tx: &mut Transaction<'_>, args: QueryArgs) -> Result<(), Box<dyn Error>> {
    // TODO: how do I generate something better than column_1 as a name?
    let results = queries::QuerySelectFromItemFrecency::builder()
        .column_1(Some(&args.prefix))
        .limit(args.limit)
        .build()
        .query_many(tx)?;

    for res in results {
        let (item, frecency_score) = (res.item, res.frecency_score);
        let sep = &args.sep;
        println!("{frecency_score}{sep}{item}");
    }

    Ok(())
}
