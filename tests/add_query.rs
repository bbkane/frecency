use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};
use std::process::Command;
use tempfile::TempDir;

const ZERO_TIME: &str = "0001-01-01T00:00:00Z";
const ONE_TIME: &str = "0001-01-01T01:00:00Z";
const TWO_TIME: &str = "0001-01-01T02:00:00Z";

#[test]
fn test_add_query() {
    let tmp_dir = TempDir::with_prefix("frecency-").unwrap().keep();
    let db_path = tmp_dir.join("test_add_query.db");
    eprintln!("database preserved at: {}", db_path.display());

    let bin = get_cargo_bin("frecency");

    assert_cmd_snapshot!(
        "add",
        Command::new(&bin)
            .args([
                "add",
                "--key",
                "key",
                "--base-score",
                "10",
                "--create-time",
                ZERO_TIME,
                "--update-time",
                ONE_TIME,
                "--db-path",
            ])
            .arg(&db_path)
    );

    assert_cmd_snapshot!(
        "query_after_add",
        Command::new(&bin)
            .args(["query", "--now", TWO_TIME, "--db-path"])
            .arg(&db_path)
    );
}
