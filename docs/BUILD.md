# Build & Reproducibility

## Toolchain pin

IFB pins the Rust toolchain in `rust-toolchain.toml` to avoid API drift.

To update it:
1. Edit `rust-toolchain.toml`.
2. Run `rustup toolchain install <version>`.
3. Regenerate the lockfile (`cargo update` or `cargo generate-lockfile`).

## Dependency pinning

Dependencies are locked in `Cargo.lock` (workspace root). To update pins:

```bash
cargo update
```

Commit the updated `Cargo.lock` with the code changes that require it.

## Build steps

```bash
./scripts/setup.sh
./scripts/build_release.sh
```

### libcurl requirements

The default target links against libcurl. `scripts/setup.sh` checks for the dev package via `pkg-config`. If you need a custom build, set:

- `IFB_STATIC_LIB_DIR`
- `IFB_STATIC_LIBS` (comma-separated)
- `IFB_INCLUDE_DIR`

Optional:
- `IFB_LINK_ASAN=1` to link ASan runtime when the target library is ASan-built.
