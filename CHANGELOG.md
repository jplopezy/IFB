# Changelog

## Migration Notes

- **Toolchain pin:** Rust toolchain is pinned in `rust-toolchain.toml`.
- **LibAFL pin:** Dependencies are locked in `Cargo.lock` (workspace root). Update pins with `cargo update` and commit the lockfile.
- **Build scripts:** Use `scripts/setup.sh` and `scripts/build_release.sh` for the reproducible path.
