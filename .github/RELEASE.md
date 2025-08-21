# Release Process

This document outlines the process for creating releases of V6.

## Pre-Release Checklist

- [ ] All tests are passing on main branch
- [ ] Documentation is up to date
- [ ] `CHANGELOG.md` has been updated with new features/fixes
- [ ] Version number in `Cargo.toml` reflects the new release
- [ ] All dependencies are up to date and security-audited
- [ ] Integration tests pass with real-world scenarios

## Release Steps

### 1. Prepare the Release

```bash
# Ensure you're on the main branch and up to date
git checkout main
git pull origin main

# Run a final test suite
cargo test
cargo clippy -- -D warnings
cargo fmt --check

# Update version in Cargo.toml if needed
# Update CHANGELOG.md with release notes
```

### 2. Create and Push Tag

```bash
# Create annotated tag
git tag -a v1.0.0 -m "Release v1.0.0"

# Push tag to trigger release workflow
git push origin v1.0.0
```

### 3. Monitor Release Build

- Watch the [GitHub Actions](https://github.com/your-username/your-repo/actions) for the release workflow
- Ensure all platform builds complete successfully
- Verify that the release is created with all artifacts

### 4. Post-Release Tasks

- [ ] Test download and installation of released binaries
- [ ] Update documentation if needed
- [ ] Announce release in relevant channels
- [ ] Update any dependent projects

## Release Types

### Patch Release (v1.0.1)
- Bug fixes
- Security patches
- Documentation updates
- No breaking changes

### Minor Release (v1.1.0)
- New features
- Enhancements
- Deprecations (with backward compatibility)
- No breaking changes

### Major Release (v2.0.0)
- Breaking changes
- Major new features
- API changes
- Architectural changes

## Emergency Releases

For critical security fixes or major bugs:

1. Create a hotfix branch from the latest release tag
2. Apply minimal fix
3. Create emergency release following same process
4. Merge hotfix back to main branch

## Rollback Procedure

If a release has critical issues:

1. Mark the GitHub release as a pre-release
2. Create a new patch release with fixes
3. Update documentation to recommend the fixed version

## Artifacts

Each release creates the following artifacts:

### Binaries
- `v6-linux-x86_64.tar.gz` - Linux x86_64
- `v6-linux-aarch64.tar.gz` - Linux ARM64
- `v6-linux-x86_64-musl.tar.gz` - Linux x86_64 (musl)
- `v6-macos-x86_64.tar.gz` - macOS Intel
- `v6-macos-aarch64.tar.gz` - macOS Apple Silicon
- `v6-windows-x86_64.zip` - Windows x86_64

### Checksums
- SHA256 checksums for all binary archives

### Docker Images
- `ghcr.io/your-username/v6:latest`
- `ghcr.io/your-username/v6:v1.0.0`
- `ghcr.io/your-username/v6:1.0`
- `ghcr.io/your-username/v6:1`

## Troubleshooting

### Build Failures

If a platform build fails:
1. Check the GitHub Actions logs
2. Fix the issue in a new commit
3. Create a new tag for the fixed version

### Missing Artifacts

If some artifacts are missing from the release:
1. Re-run the failed jobs in GitHub Actions
2. Or delete the tag, fix issues, and create a new tag

### Docker Build Issues

If Docker builds fail:
1. Check Docker workflow logs
2. Test Docker build locally
3. Fix Dockerfile or dependencies as needed

## Testing Releases

Before marking a release as stable:

```bash
# Test binary download and execution
curl -L https://github.com/your-username/your-repo/releases/download/v1.0.0/v6-linux-x86_64.tar.gz | tar xz
./v6-linux-x86_64 --help

# Test Docker image
docker run --rm ghcr.io/your-username/v6:v1.0.0 --help

# Test with real load test scenario
./v6-linux-x86_64 init test.js
./v6-linux-x86_64 run test.js
```

## Release Notes Template

```markdown
## v1.0.0 - YYYY-MM-DD

### Added
- New feature descriptions

### Changed  
- Changes to existing features

### Deprecated
- Features marked for removal

### Removed
- Removed features

### Fixed
- Bug fixes

### Security
- Security improvements
```
