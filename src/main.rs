//! `rustop` is a terminal-based system monitor written in Rust. It shows:
//! - Per-thread CPU load
//! - Combined CPU utilization
//! - Memory usage
//! - GPU utilization (NVIDIA NVML)
//! - GPU memory usage
//! - GPU temperature
//!
//! ## Docs and README maintenance
//! - Build docs locally: `scripts/build_docs.sh` (outputs to `target/doc`).
//! - Regenerate README from crate docs (requires `cargo-readme`): `scripts/update_readme.sh`.
//!
//! ## License
//! Licensed under the GNU General Public License v3.0 or later (GPL-3.0-or-later).

mod app;
mod cpu;
mod gpu;
mod ui;
mod utilities;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}
