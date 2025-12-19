#!/usr/bin/env bash
# Release script for CVLR workspace
# This script uses cargo-release to manage releases for all crates in the workspace

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Change to project root
cd "$PROJECT_ROOT"

# Check if cargo-release is installed
if ! command -v cargo-release &> /dev/null; then
    echo -e "${RED}Error: cargo-release is not installed${NC}"
    echo "Install it with: cargo install cargo-release"
    exit 1
fi

# Function to print usage
usage() {
    cat << EOF
Usage: $0 [OPTIONS] [VERSION_TYPE]

Release all crates in the CVLR workspace using cargo-release.

VERSION_TYPE (optional):
    patch     - Bump patch version (default: 0.4.1 -> 0.4.2)
    minor     - Bump minor version (default: 0.4.1 -> 0.5.0)
    major     - Bump major version (default: 0.4.1 -> 1.0.0)
    <version> - Set specific version (e.g., 0.4.2)

OPTIONS:
    --execute         - Execute the release command
    --no-publish      - Skip publishing to crates.io
    --no-push         - Skip pushing to remote repository
    --no-tag          - Skip creating git tag
    --allow-dirty     - Allow release with uncommitted changes
    --help            - Show this help message

Examples:
    $0 --execute patch              # Release patch version
    $0 --execute minor              # Release minor version
    $0 patch                        # Preview patch release
    $0 --execute 0.5.0              # Release specific version
EOF
}

# Parse arguments
EXECUTE=false
NO_PUBLISH=false
NO_PUSH=false
NO_TAG=false
ALLOW_DIRTY=false
VERSION_TYPE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --execute)
            EXECUTE=true
            shift
            ;;
        --no-publish)
            NO_PUBLISH=true
            shift
            ;;
        --no-push)
            NO_PUSH=true
            shift
            ;;
        --no-tag)
            NO_TAG=true
            shift
            ;;
        --allow-dirty)
            ALLOW_DIRTY=true
            shift
            ;;
        --help|-h)
            usage
            exit 0
            ;;
        patch|minor|major)
            VERSION_TYPE="$1"
            shift
            ;;
        *)
            # Check if it's a version number
            if [[ "$1" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
                VERSION_TYPE="$1"
            else
                echo -e "${RED}Error: Unknown option or invalid version: $1${NC}"
                usage
                exit 1
            fi
            shift
            ;;
    esac
done

# Default to patch if no version type specified
if [ -z "$VERSION_TYPE" ]; then
    VERSION_TYPE="patch"
fi

# Build cargo-release command
RELEASE_CMD="cargo release --workspace"

# Add version type
if [[ "$VERSION_TYPE" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    RELEASE_CMD="$RELEASE_CMD $VERSION_TYPE"
else
    RELEASE_CMD="$RELEASE_CMD $VERSION_TYPE"
fi

# Add options
if [ "$EXECUTE" = true ]; then
    # Execute without confirmation prompt
    RELEASE_CMD="$RELEASE_CMD --execute"
else
    echo -e "${YELLOW}Running in dry-run mode (no changes will be made)${NC}"
fi

if [ "$NO_PUBLISH" = true ]; then
    RELEASE_CMD="$RELEASE_CMD --no-publish"
fi

if [ "$NO_PUSH" = true ]; then
    RELEASE_CMD="$RELEASE_CMD --no-push"
fi

if [ "$NO_TAG" = true ]; then
    RELEASE_CMD="$RELEASE_CMD --no-tag"
fi

if [ "$ALLOW_DIRTY" = true ]; then
    RELEASE_CMD="$RELEASE_CMD --allow-dirty"
fi

# Show what will be done
echo -e "${GREEN}Preparing to release all crates in workspace...${NC}"
echo "Version type: $VERSION_TYPE"
echo "Command: $RELEASE_CMD"
echo ""

# Check git status if not allowing dirty
if [ "$ALLOW_DIRTY" = false ] && [ "$EXECUTE" = true ]; then
    if ! git diff-index --quiet HEAD --; then
        echo -e "${RED}Error: Working directory is not clean${NC}"
        echo "Commit or stash your changes first, or use --allow-dirty"
        exit 1
    fi
fi

# Run the release command
echo -e "${GREEN}Running cargo-release...${NC}"
eval "$RELEASE_CMD"

if [ "$EXECUTE" = true ]; then
    echo -e "${GREEN}Release completed successfully!${NC}"
else
    echo -e "${YELLOW}Dry-run completed. No changes were made.${NC}"
fi

