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

Then after adding a test you need to run

```bash
cargo insta test
```

```bash
cargo insta review
```

to accept the snapshot