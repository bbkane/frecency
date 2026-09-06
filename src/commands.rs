use crate::queries;
use clap::Args;
use jiff::Timestamp;
use rusqlite::Transaction;
use std::{error::Error, io, path::Path};

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
pub struct PruneArgs {
    #[arg(long)]
    pub score_below: Option<f64>,

    #[arg(long, default_value_t = false, action = clap::ArgAction::Set)]
    pub missing: bool,

    #[arg(long)]
    pub prefix: Option<String>,

    #[arg(long)]
    pub not_accessed_for: Option<i64>,

    #[arg(long)]
    pub created_before: Option<Timestamp>,

    #[arg(long)]
    pub updated_before: Option<Timestamp>,

    #[arg(long, default_value_t = Timestamp::now())]
    pub now: Timestamp,

    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    pub dry_run: bool,

    #[arg(long, default_value_t = false, action = clap::ArgAction::Set)]
    pub quiet: bool,
}

pub fn prune(tx: &mut Transaction<'_>, args: PruneArgs) -> Result<(), Box<dyn Error>> {
    let has_selector = args.score_below.is_some()
        || args.missing
        || args.prefix.is_some()
        || args.not_accessed_for.is_some()
        || args.created_before.is_some()
        || args.updated_before.is_some();
    if !has_selector {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "prune requires at least one selection filter",
        )
        .into());
    }
    if args.score_below.is_some_and(|score| !score.is_finite()) {
        return Err(
            io::Error::new(io::ErrorKind::InvalidInput, "--score-below must be finite").into(),
        );
    }
    if args.prefix.as_deref() == Some("") {
        return Err(
            io::Error::new(io::ErrorKind::InvalidInput, "--prefix must not be empty").into(),
        );
    }
    if args.not_accessed_for.is_some_and(|days| days < 0) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "--not-accessed-for must not be negative",
        )
        .into());
    }

    let last_access_before = args
        .not_accessed_for
        .map(|days| {
            days.checked_mul(86_400)
                .and_then(|seconds| args.now.as_second().checked_sub(seconds))
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "--not-accessed-for is too large",
                    )
                })
        })
        .transpose()?;

    let mut candidates = queries::SelectPruneCandidates::builder()
        .score_below(args.score_below)
        .prefix(args.prefix.as_deref())
        .last_access_before(last_access_before)
        .created_before(args.created_before.map(|time| time.as_second()))
        .updated_before(args.updated_before.map(|time| time.as_second()))
        .now(args.now.as_second())
        .build()
        .query_many(tx)?;

    if args.missing {
        candidates.retain(|candidate| !Path::new(&candidate.item).exists());
    }

    let access_count: i64 = candidates
        .iter()
        .map(|candidate| candidate.access_count)
        .sum();

    if !args.dry_run {
        for candidate in &candidates {
            queries::DeleteItemById::builder()
                .id(candidate.id)
                .build()
                .execute(tx)?;
        }
    }

    if !args.quiet {
        if args.dry_run {
            println!("Dry run - run with `--dry-run false` to actually prune");
        }
        if !candidates.is_empty() {
            println!("Deleted:");
            for candidate in &candidates {
                println!("  {}", candidate.item);
            }
        }
        let item_label = if candidates.len() == 1 {
            "item"
        } else {
            "items"
        };
        let access_label = if access_count == 1 {
            "access"
        } else {
            "accesses"
        };
        println!(
            "Summary: deleted {} {item_label} and {access_count} {access_label}",
            candidates.len(),
        );
    }

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
