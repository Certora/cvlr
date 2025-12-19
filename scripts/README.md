# Release Scripts

This directory contains scripts for managing releases of all crates in the CVLR workspace using `cargo-release`.

## Prerequisites

Install `cargo-release` if you haven't already:

```bash
cargo install cargo-release
```

## Configuration

The release process is configured in `release.toml` at the project root. This file defines:
- Which crates to release (all workspace members)
- Whether to update CHANGELOG.md
- Whether to commit, tag, and push changes
- Whether to publish to crates.io
- Pre-release replacements to update workspace dependency versions in `Cargo.toml`

## Usage

### Main Release Script

The main script is `release.sh`. It supports various options:

```bash
# Release a patch version (0.4.1 -> 0.4.2)
./scripts/release.sh patch

# Release a minor version (0.4.1 -> 0.5.0)
./scripts/release.sh minor

# Release a major version (0.4.1 -> 1.0.0)
./scripts/release.sh major

# Release a specific version
./scripts/release.sh 0.5.0

# Dry-run to preview changes
./scripts/release.sh patch --dry-run

# Release without publishing to crates.io
./scripts/release.sh patch --no-publish

# Release without pushing to remote
./scripts/release.sh patch --no-push

# Release without creating git tag
./scripts/release.sh patch --no-tag

# Allow release with uncommitted changes
./scripts/release.sh patch --allow-dirty
```

### Quick Release Scripts

For convenience, there are shortcut scripts:

```bash
# Patch release
./scripts/release-patch.sh

# Minor release
./scripts/release-minor.sh

# Major release
./scripts/release-major.sh
```

All quick scripts support the same options as the main script:

```bash
./scripts/release-patch.sh --dry-run
./scripts/release-minor.sh --no-publish
```

## Release Process

The release process will:

1. **Bump versions** - Update version numbers in all crate `Cargo.toml` files
2. **Update CHANGELOG.md** - Add a new release entry (if configured)
3. **Commit changes** - Create a commit with the version bump and changelog
4. **Create git tag** - Tag the release (e.g., `v0.4.2`)
5. **Push to remote** - Push commits and tags to the remote repository
6. **Publish to crates.io** - Publish all crates to crates.io (if enabled)

## Pre-Release Checklist

Before releasing, make sure:

- [ ] All tests pass: `cargo test --workspace`
- [ ] Code is formatted: `cargo fmt --all`
- [ ] Linting passes: `cargo clippy --workspace`
- [ ] CHANGELOG.md is updated with changes for the new version
- [ ] Working directory is clean (or use `--allow-dirty` if needed)
- [ ] You have permissions to publish to crates.io
- [ ] You have push access to the repository

## Troubleshooting

### "cargo-release is not installed"
Install it with: `cargo install cargo-release`

### "Working directory is not clean"
Commit or stash your changes, or use `--allow-dirty` flag

### "Publish failed"
Make sure you're logged in to crates.io: `cargo login`

### "Tag already exists"
The version may have already been released. Check existing tags: `git tag -l`

## Customization

To customize the release process, edit `release.toml` at the project root. You can:

- Enable/disable changelog updates
- Configure pre-release or post-release commands
- Customize per-package settings
- Adjust commit message format
- Configure tag format

For more information, see the [cargo-release documentation](https://github.com/crate-ci/cargo-release).

