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
pub struct UpdateArgs {
    #[arg(long)]
    pub key: String,

    #[arg(long)]
    pub base_score: Option<i64>,

    #[arg(long, default_value_t = Timestamp::now())]
    pub update_time: Timestamp,

    #[arg(long)]
    pub new_key: Option<String>,
}

pub fn update(tx: &mut Transaction<'_>, args: UpdateArgs) -> Result<(), Box<dyn Error>> {
    queries::UpdateItem::builder()
        .new_key(args.new_key.as_deref())
        .base_score(args.base_score)
        .update_time(args.update_time.as_second())
        .key(&args.key)
        .build()
        .query_one(tx)?;

    Ok(())
}

#[derive(Args)]
pub struct DeleteArgs {
    #[arg(long)]
    pub key: String,
}

pub fn delete(tx: &mut Transaction<'_>, args: DeleteArgs) -> Result<(), Box<dyn Error>> {
    queries::DeleteItem::builder()
        .key(&args.key)
        .build()
        .query_one(tx)?;

    Ok(())
}

#[derive(Args)]
pub struct QueryArgs {
    #[arg(long, default_value_t = Timestamp::now())]
    pub now: Timestamp,

    #[arg(long, default_value_t = 500)]
    pub limit: i64,

    #[arg(long, default_value_t = String::from(""))]
    pub prefix: String,

    #[arg(long, default_value_t = String::from("\t"))]
    pub sep: String,
}
pub fn query(tx: &mut Transaction<'_>, args: QueryArgs) -> Result<(), Box<dyn Error>> {
    let results = queries::QuerySelectFromItemFrecency::builder()
        .now(args.now.as_second())
        .prefix(Some(&args.prefix))
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
