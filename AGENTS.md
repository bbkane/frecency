# AGENTS.md

Please read the following and make a plan first for me to review before implementing

## Project workflow

- Rust CLI backed by SQLite and generated `sqlc` bindings.
- Edit SQL sources under `db/queries/` and `db/migrations/`; do not hand-edit `src/queries.rs`.
- After changing SQL, regenerate bindings and database docs from the repository root:

  ```sh
  cd db && go generate ./...
  ```

## Snapshot tests

- CLI integration tests live under `tests/` and use `insta-cmd`.
- Use fixed timestamps, including `query --now`, so output is deterministic.
- Avoid the interactive `cargo insta review` TUI. Run snapshot tests, inspect every `.snap.new` file, then accept reviewed snapshots from the repository root:

  ```sh
  cargo insta test
  cargo insta accept
  ```

- Commit accepted `.snap` files; do not commit `.snap.new` files.

## Validation

```sh
cargo fmt -- --check
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```
