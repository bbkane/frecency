use clap::Args;

use jiff::Timestamp;

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
