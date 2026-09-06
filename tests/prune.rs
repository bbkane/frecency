use insta_cmd::{assert_cmd_snapshot, get_cargo_bin};
use std::{fs, path::Path, process::Command};
use tempfile::TempDir;

const ZERO_TIME: &str = "0001-01-01T00:00:00Z";
const ONE_TIME: &str = "0001-01-01T01:00:00Z";
const TWO_TIME: &str = "0001-01-01T02:00:00Z";
const LATE_TIME: &str = "0001-01-25T00:00:00Z";
const NOW_TIME: &str = "0001-02-10T00:00:00Z";

fn command(bin: &Path, db_path: &Path) -> Command {
    let mut command = Command::new(bin);
    command.arg("--db-path").arg(db_path);
    command
}

fn run_success(command: &mut Command) {
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn add(bin: &Path, db_path: &Path, key: &str, base_score: &str, time: &str) {
    run_success(command(bin, db_path).args([
        "add",
        "--key",
        key,
        "--base-score",
        base_score,
        "--create-time",
        time,
        "--update-time",
        time,
    ]));
}

fn test_db(name: &str) -> (TempDir, std::path::PathBuf) {
    let tmp_dir = TempDir::with_prefix("frecency-").unwrap();
    let db_path = tmp_dir.path().join(format!("{name}.db"));
    (tmp_dir, db_path)
}

#[test]
fn test_prune_validation() {
    let bin = get_cargo_bin("frecency");
    let (_tmp_dir, db_path) = test_db("test_prune_validation");

    assert_cmd_snapshot!(
        "01_no_selection_filter",
        command(&bin, &db_path).args(["prune", "--now", NOW_TIME])
    );
    assert_cmd_snapshot!(
        "02_boolean_requires_value",
        command(&bin, &db_path).args(["prune", "--missing"])
    );
    assert_cmd_snapshot!(
        "03_missing_false_is_not_filter",
        command(&bin, &db_path).args(["prune", "--missing", "false"])
    );
    assert_cmd_snapshot!(
        "04_empty_prefix",
        command(&bin, &db_path).args(["prune", "--prefix", ""])
    );
    assert_cmd_snapshot!(
        "05_negative_days",
        command(&bin, &db_path).args(["prune", "--not-accessed-for", "-1"])
    );
}

#[test]
fn test_prune_database_filters() {
    let bin = get_cargo_bin("frecency");
    let (_tmp_dir, db_path) = test_db("test_prune_database_filters");

    add(&bin, &db_path, "/prune/alpha", "1", ZERO_TIME);
    add(&bin, &db_path, "/prune/alpha", "1", ONE_TIME);
    add(&bin, &db_path, "/prune/beta", "20", ZERO_TIME);
    add(&bin, &db_path, "/keep/gamma", "1", ZERO_TIME);

    assert_cmd_snapshot!(
        "06_combined_filters_dry_run",
        command(&bin, &db_path).args([
            "prune",
            "--score-below",
            "10",
            "--prefix",
            "/prune/",
            "--created-before",
            ONE_TIME,
            "--now",
            NOW_TIME,
            "--dry-run",
            "true",
        ])
    );
    assert_cmd_snapshot!(
        "07_query_after_dry_run",
        command(&bin, &db_path).args(["query", "--now", NOW_TIME])
    );
    assert_cmd_snapshot!(
        "08_combined_filters_delete",
        command(&bin, &db_path).args([
            "prune",
            "--score-below",
            "10",
            "--prefix",
            "/prune/",
            "--created-before",
            ONE_TIME,
            "--now",
            NOW_TIME,
            "--dry-run",
            "false",
        ])
    );
    assert_cmd_snapshot!(
        "09_query_after_combined_filters",
        command(&bin, &db_path).args(["query", "--now", NOW_TIME])
    );
}

#[test]
fn test_prune_time_filters() {
    let bin = get_cargo_bin("frecency");
    let (_tmp_dir, db_path) = test_db("test_prune_time_filters");

    add(&bin, &db_path, "/stale", "0", ZERO_TIME);
    add(&bin, &db_path, "/fresh", "0", LATE_TIME);

    assert_cmd_snapshot!(
        "10_last_access_and_update_filters",
        command(&bin, &db_path).args([
            "prune",
            "--not-accessed-for",
            "30",
            "--updated-before",
            TWO_TIME,
            "--now",
            NOW_TIME,
            "--dry-run",
            "false",
        ])
    );
    assert_cmd_snapshot!(
        "11_query_after_time_filters",
        command(&bin, &db_path).args(["query", "--now", NOW_TIME])
    );
    assert_cmd_snapshot!(
        "12_no_matches",
        command(&bin, &db_path).args(["prune", "--updated-before", ZERO_TIME, "--now", NOW_TIME,])
    );
}

#[test]
fn test_prune_missing() {
    let bin = get_cargo_bin("frecency");
    let (tmp_dir, db_path) = test_db("test_prune_missing");
    let existing_key = tmp_dir.path().join("existing");
    fs::write(&existing_key, "exists").unwrap();
    let missing_key = "/tmp/frecency-prune-test-path-that-does-not-exist";

    add(
        &bin,
        &db_path,
        existing_key.to_str().unwrap(),
        "0",
        ZERO_TIME,
    );
    add(&bin, &db_path, missing_key, "0", ZERO_TIME);

    assert_cmd_snapshot!(
        "13_missing_paths",
        command(&bin, &db_path).args([
            "prune",
            "--missing",
            "true",
            "--now",
            NOW_TIME,
            "--dry-run",
            "false",
        ])
    );

    let output = command(&bin, &db_path)
        .args([
            "query",
            "--prefix",
            existing_key.to_str().unwrap(),
            "--now",
            NOW_TIME,
        ])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains(existing_key.to_str().unwrap()));

    #[cfg(unix)]
    {
        let broken_link = tmp_dir.path().join("broken-link");
        std::os::unix::fs::symlink(tmp_dir.path().join("missing-target"), &broken_link).unwrap();
        add(
            &bin,
            &db_path,
            broken_link.to_str().unwrap(),
            "0",
            ZERO_TIME,
        );
        assert_cmd_snapshot!(
            "14_broken_symlink_is_missing",
            command(&bin, &db_path).args([
                "prune",
                "--missing",
                "true",
                "--quiet",
                "true",
                "--now",
                NOW_TIME,
                "--dry-run",
                "false",
            ])
        );

        let output = command(&bin, &db_path)
            .args([
                "query",
                "--prefix",
                broken_link.to_str().unwrap(),
                "--now",
                NOW_TIME,
            ])
            .output()
            .unwrap();
        assert!(output.status.success());
        assert!(output.stdout.is_empty());
    }
}

#[test]
fn test_prune_quiet() {
    let bin = get_cargo_bin("frecency");
    let (_tmp_dir, db_path) = test_db("test_prune_quiet");

    add(&bin, &db_path, "/quiet-delete", "0", ZERO_TIME);
    assert_cmd_snapshot!(
        "15_quiet_delete",
        command(&bin, &db_path).args([
            "prune",
            "--prefix",
            "/quiet-delete",
            "--quiet",
            "true",
            "--now",
            NOW_TIME,
            "--dry-run",
            "false",
        ])
    );
    assert_cmd_snapshot!(
        "16_query_after_quiet_delete",
        command(&bin, &db_path).args(["query", "--now", NOW_TIME])
    );

    add(&bin, &db_path, "/quiet-dry-run", "0", ZERO_TIME);
    assert_cmd_snapshot!(
        "17_quiet_dry_run",
        command(&bin, &db_path).args([
            "prune",
            "--prefix",
            "/quiet-dry-run",
            "--quiet",
            "true",
            "--dry-run",
            "true",
            "--now",
            NOW_TIME,
        ])
    );
    assert_cmd_snapshot!(
        "18_query_after_quiet_dry_run",
        command(&bin, &db_path).args(["query", "--now", NOW_TIME])
    );
}
