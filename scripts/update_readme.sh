#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo-readme >/dev/null 2>&1; then
    echo "cargo-readme is required. Install with: cargo install cargo-readme" >&2
    exit 1
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

cargo readme > README.md
echo "README.md refreshed from crate documentation."
