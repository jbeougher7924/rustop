# rustop

[![CI](https://github.com/jbeougher7924/rustop/actions/workflows/ci.yml/badge.svg)](https://github.com/jbeougher7924/rustop/actions/workflows/ci.yml)
[![Docs](https://img.shields.io/badge/docs-GitHub%20Pages-blue)](https://jbeougher7924.github.io/rustop)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://github.com/jbeougher7924/rustop/blob/main/LICENSE)

`rustop` is a terminal-based system monitor written in Rust. It shows:
- Per-thread CPU load
- Combined CPU utilization
- Memory usage
- GPU utilization (NVIDIA NVML)
- GPU memory usage
- GPU temperature

### Usage
- Requirements: Rust toolchain; NVIDIA drivers/NVML for GPU stats (CPU/RAM work without GPU support).
- Run: `cargo run --release`
- Quit: press `F10` or `q`/`Q`.
- Docs: <https://jbeougher7924.github.io/rustop>

### Docs and README maintenance
- Build docs locally: `scripts/build_docs.sh` (outputs to `target/doc`).
- Regenerate README from crate docs (requires `cargo-readme`): `scripts/update_readme.sh`. CI and tests fail if the README is stale.

### Contributing
- See CONTRIBUTING.md for how to develop and test changes.
- Please review the Code of Conduct before participating.

### License
Licensed under the GNU General Public License v3.0 (GPL-3.0).

License: GPL-3.0-or-later
