#!/usr/bin/env bash
set -euo pipefail

if ! command -v mdbook >/dev/null 2>&1; then
  echo "mdBook not found. Install with: cargo install mdbook" >&2
  exit 1
fi

mdbook build
