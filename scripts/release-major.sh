#!/usr/bin/env bash
# Quick script to release a major version
# Usage: ./scripts/release-major.sh [--dry-run]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
"$SCRIPT_DIR/release.sh" major "$@"

