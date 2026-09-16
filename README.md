# skwd-iced-layershell

Minimal downstream `iced_layershell` fixes used by Skwd Wall. The crate name
and public API remain compatible with `iced_layershell` 0.19.1 and Iced 0.14.

This is not an upstream release. Prefer upstream unless you need the fixes
listed below.

## Baseline

- Package: `iced_layershell 0.19.1`
- Packaged source SHA-256:
  `51b59e8af02bf4c51db2ee5d130293e6e1e5ac4ae9f74a1e57990a19221aa0d5`
- Upstream project: <https://github.com/waycrate/exwlshelleventloop>

The package archive does not contain Cargo VCS metadata, so its digest is the
immutable provenance key. This source was reconstructed from that exact
package.

## Patch set

- Return layer-shell construction failures through `iced_layershell::Error`
  instead of panicking.
- Carry asynchronous compositor construction failures out of the callback and
  return them after the event loop exits.
- Recreate `Lost` surfaces even at zero size, reconfigure `Outdated` surfaces,
  and request a new frame after either nonzero-sized recovery.
- Retry transient `Timeout` and `Other` presentation errors when the surface
  has nonzero dimensions. Zero-sized outputs defer recovery until a later
  configure event instead of entering a redraw loop.
- Report the Wayland output containing each layer-shell window through
  `iced_layershell::output_name`. Remove the cached name when the window closes.

The output query includes the published `layershellev` 0.19.1 source with one
accessor for its existing output-name cache. Its archive digest and source
changes are recorded in `OUTPUT-PROVENANCE.md`. Wall schedules animation
updates before the draw boundary.

## Build and verify

```sh
git clone https://github.com/liixini/skwd-iced-layershell.git
cd skwd-iced-layershell
cargo fmt --check
cargo check --all-targets --no-default-features
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --no-default-features
sh scripts/check-unsafe.sh
```

Consumers should pin the published commit used by their Skwd Wall checkout:

```toml
[patch.crates-io]
iced_layershell = { git = "https://github.com/liixini/skwd-iced-layershell.git", rev = "<40-character-commit>" }
```

Do not use a branch-only Cargo dependency.

## Unsafe policy

The crate denies unsafe Rust by default. Three narrowly allowed operations own
the lifetime-extension and native-display invariants required by the upstream
event loop. Each allowance is statement-local and carries a `SAFETY:`
rationale. `unsafe-baseline.txt` is the exact source inventory; its guard
rejects any unreviewed addition, removal, or relocation. The imported
`layershellev` source adds seven unchanged upstream raw-handle borrows to that
inventory.

## License

The upstream MIT license is preserved in `LICENSE`. The dependency notice is
`iced_layershell 0.19.1 — Copyright (c) 2023 DecodeTalkers — MIT.` Package
author metadata additionally names Decodertalkers and Aakash Sen Sharma.
