Ran:

```
cargo new frecency
cd frecency
goreleaser init
```



Requires `zig` for cross compilation:

```
brew install zig
```

Testing:

Install `cargo-insta`:

```bash
cargo install cargo-insta
```

Run snapshot tests:

```bash
cargo insta test
```

Inspect every `.snap.new` file, then accept the snapshots without opening the review TUI:

```bash
cargo insta accept
```

Or (if not an agent), use the TUI to do this manually:

```bash
cargo insta review
```

Delete snapshots that are no longer referenced by tests:

```bash
cargo insta test --unreferenced delete
```