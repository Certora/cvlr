#!/usr/bin/env bash
# Quick script to release a patch version
# Usage: ./scripts/release-patch.sh [--dry-run]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
"$SCRIPT_DIR/release.sh" patch "$@"

