# frecency

A small CLI to calculate [frecency](https://en.wikipedia.org/wiki/Frecency). Intended for Neovim integration with a mini.pick picker.

TODO: GIF demo

# Install

- [Homebrew](https://brew.sh/): `brew install --cask bbkane/tap/frecency`
- [Scoop](https://scoop.sh/):

```
scoop bucket add bbkane https://github.com/bbkane/scoop-bucket
scoop install bbkane/frecency                 
```

- Download Mac/Linux/Windows executable: [GitHub releases](https://github.com/bbkane/frecency/releases)
- Build with [goreleaser](https://goreleaser.com/) after cloning: ` goreleaser release --snapshot --clean`

# Dev notes

Requires `zig` for cross compilation:

```
brew install zig
```

## Snapshot Testing

Install `cargo-insta`:

```bash
cargo install cargo-insta
```

Run snapshot tests:

```bash
cargo insta test
```

Agents: inspect every `.snap.new` file, then accept the snapshots without opening the review TUI:

```bash
cargo insta accept
```

Humans: use the TUI to review snapshots:

```bash
cargo insta review
```

Delete snapshots that are no longer referenced by tests:

```bash
cargo insta test --unreferenced delete
```
