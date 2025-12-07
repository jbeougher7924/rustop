# Contributing to rustop

Thanks for your interest in improving rustop! Contributions are welcome. Please follow these guidelines:

## Getting started
- Install a recent Rust toolchain (stable).
- Clone the repo and run `cargo test` to ensure the baseline is green.
- For GPU features, ensure NVIDIA drivers/NVML are available on your system.

## Development workflow
- Format and lint: `cargo fmt` and `cargo clippy -- -D warnings`.
- Run tests: `cargo test`.
- Keep the README in sync: `scripts/update_readme.sh` (requires `cargo-readme`).
- If you install `cargo-deny`, run `cargo deny check` to verify dependencies.

## Commit and PR guidelines
- Keep changes focused; prefer small, single-purpose PRs.
- Include tests when fixing bugs or adding features.
- Update documentation and README when behavior or requirements change.
- Adhere to the Code of Conduct.

## Questions or issues
If you’re unsure about an approach, open an issue or draft PR to discuss before investing lots of time. Thank you for contributing!
