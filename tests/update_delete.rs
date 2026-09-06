use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};
use std::{path::Path, process::Command};
use tempfile::TempDir;

const ZERO_TIME: &str = "0001-01-01T00:00:00Z";
const ONE_TIME: &str = "0001-01-01T01:00:00Z";
const TWO_TIME: &str = "0001-01-01T02:00:00Z";

fn command(bin: &Path, db_path: &Path) -> Command {
    let mut command = Command::new(bin);
    command.arg("--db-path").arg(db_path);
    command
}

#[test]
fn test_update_delete() {
    let tmp_dir = TempDir::with_prefix("frecency-").unwrap().keep();
    let db_path = tmp_dir.join("test_update_delete.db");
    eprintln!("database preserved at: {}", db_path.display());

    let bin = get_cargo_bin("frecency");

    assert_cmd_snapshot!(
        "01_add",
        command(&bin, &db_path).args([
            "add",
            "--key",
            "key",
            "--create-time",
            ZERO_TIME,
            "--update-time",
            ZERO_TIME,
        ])
    );

    assert_cmd_snapshot!(
        "02_update_score",
        command(&bin, &db_path).args([
            "update",
            "--key",
            "key",
            "--base-score",
            "10",
            "--update-time",
            ONE_TIME,
        ])
    );

    assert_cmd_snapshot!(
        "03_query_after_score_update",
        command(&bin, &db_path).args(["query", "--now", TWO_TIME])
    );

    assert_cmd_snapshot!(
        "04_update_key",
        command(&bin, &db_path).args([
            "update",
            "--key",
            "key",
            "--new-key",
            "new-key",
            "--update-time",
            TWO_TIME,
        ])
    );

    assert_cmd_snapshot!(
        "05_query_after_key_update",
        command(&bin, &db_path).args(["query", "--now", TWO_TIME])
    );

    assert_cmd_snapshot!(
        "06_delete",
        command(&bin, &db_path).args(["delete", "--key", "new-key"])
    );

    assert_cmd_snapshot!(
        "07_query_after_delete",
        command(&bin, &db_path).args(["query", "--now", TWO_TIME])
    );
}
