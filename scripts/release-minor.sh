#!/usr/bin/env bash
# Quick script to release a minor version
# Usage: ./scripts/release-minor.sh [--dry-run]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
"$SCRIPT_DIR/release.sh" minor "$@"

