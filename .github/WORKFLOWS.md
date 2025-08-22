# GitHub Actions Workflows Documentation

This document explains the complete CI/CD setup for the V6 load testing tool, including how the workflows integrate with the local build scripts.

## Overview

The V6 project uses a **unified build system** where the same scripts used for local development are also used in CI/CD. This ensures consistency and reproducibility across all environments.

### Key Components

- **`build-all.sh`**: Main multi-target build script
- **`quick-build.sh`**: Simplified build script for development
- **`Makefile`**: Convenient wrapper targets
- **GitHub Actions**: Automated CI/CD using the build scripts

## Workflows

### 1. CI Workflow (`.github/workflows/ci.yml`)

**Purpose**: Basic continuous integration for every push/PR

**Triggers**:
- Push to `main`, `master`, `develop`
- Pull requests to `main`, `master`, `develop`

**Jobs**:
- **Test**: Code formatting, linting, unit tests
- **Build**: Cross-platform build verification using `build-all.sh`

**Matrix Strategy**:
```yaml
matrix:
  include:
    - os: ubuntu-latest
      targets: "x86_64-unknown-linux-gnu"
    - os: macos-latest  
      targets: "aarch64-apple-darwin"
    - os: windows-latest
      targets: "x86_64-pc-windows-msvc"
```

**Build Command**:
```bash
./build-all.sh --install-targets x86_64-unknown-linux-gnu
```

### 2. Build Workflow (`.github/workflows/build.yml`)

**Purpose**: Comprehensive multi-platform builds for main branch and tags

**Triggers**:
- Push to `main`, `master`
- Tags matching `v*`
- Pull requests to `main`, `master`
- Manual dispatch

**Jobs**:
- **Test**: Same as CI workflow
- **Build**: Full cross-platform matrix using `build-all.sh`
- **Release**: Creates GitHub releases for tags
- **Docker**: Builds and pushes container images

**Build Matrix**:
```yaml
matrix:
  include:
    - os: ubuntu-latest
      targets: "x86_64-unknown-linux-gnu x86_64-unknown-linux-musl aarch64-unknown-linux-gnu"
    - os: macos-latest
      targets: "x86_64-apple-darwin aarch64-apple-darwin"
    - os: windows-latest
      targets: "x86_64-pc-windows-msvc"
```

**Build Command**:
```bash
./build-all.sh --install-targets x86_64-unknown-linux-gnu x86_64-unknown-linux-musl aarch64-unknown-linux-gnu
```

### 3. Release Workflow (`.github/workflows/release.yml`)

**Purpose**: Production release builds with optimized binaries and artifacts

**Triggers**:
- Tags matching `v*`
- Manual dispatch with tag input

**Jobs**:
- **Create Release**: Creates draft GitHub release
- **Build Release**: Multi-platform optimized builds with archives
- **Finalize Release**: Publishes release with all artifacts

**Features**:
- Optimized release builds
- Automatic archive creation (`.tar.gz`, `.zip`)
- SHA256 checksums
- Binary stripping for smaller sizes
- Draft → Published release flow

**Build Command**:
```bash
./build-all.sh --install-targets --clean x86_64-unknown-linux-gnu aarch64-apple-darwin
```

### 4. Build Script Test Workflow (`.github/workflows/test-build-script.yml`)

**Purpose**: Validates build scripts work correctly across platforms

**Triggers**:
- Push to `main`, `master`, `develop`
- Pull requests
- Manual dispatch

**Jobs**:
- **Test Build Script**: Tests `build-all.sh` functionality
- **Smoke Test**: Basic binary execution tests
- **Verify Script Consistency**: Syntax and compatibility checks
- **Makefile Test**: Tests Makefile integration

**Tests**:
- Script help and list-targets commands
- Single target builds
- Binary execution verification
- Cross-platform compatibility

## Build Script Integration

### How Workflows Use build-all.sh

All workflows use the same `build-all.sh` script with different parameters:

#### Development Builds (CI)
```bash
./build-all.sh --install-targets x86_64-unknown-linux-gnu
```
- Single target for fast feedback
- Basic verification builds

#### Feature Builds (Build Workflow)
```bash
./build-all.sh --install-targets x86_64-unknown-linux-gnu x86_64-unknown-linux-musl aarch64-unknown-linux-gnu
```
- Multiple targets per platform
- Comprehensive coverage

#### Release Builds (Release Workflow)
```bash
./build-all.sh --install-targets --clean x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu
```
- Clean builds from scratch
- Optimized for distribution
- Includes archives and checksums

### Platform-Specific Setup

#### Linux (Ubuntu)
```yaml
- name: Install cross-compilation tools
  run: |
    sudo apt-get update
    sudo apt-get install -y musl-tools gcc-aarch64-linux-gnu
    cargo install cross --git https://github.com/cross-rs/cross
```

#### macOS
```yaml
- name: Install cross-compilation tools
  run: |
    if [[ ${BASH_VERSION%%.*} -lt 4 ]]; then
      brew install bash
    fi
```

#### Windows
```yaml
- name: Install cross-compilation tools
  run: |
    cargo install cross --git https://github.com/cross-rs/cross
```

## Supported Targets

| Platform | Targets | Notes |
|----------|---------|-------|
| **Linux** | `x86_64-unknown-linux-gnu`<br>`x86_64-unknown-linux-musl`<br>`aarch64-unknown-linux-gnu` | GNU libc, musl libc, ARM64 |
| **macOS** | `x86_64-apple-darwin`<br>`aarch64-apple-darwin` | Intel, Apple Silicon |
| **Windows** | `x86_64-pc-windows-msvc` | MSVC toolchain |

## Artifacts

### CI Artifacts
- **Retention**: 7 days
- **Purpose**: Development verification
- **Contents**: Basic binaries

### Build Artifacts  
- **Retention**: 7 days
- **Purpose**: Feature testing
- **Contents**: Multi-platform binaries

### Release Artifacts
- **Retention**: Permanent (GitHub releases)
- **Purpose**: Distribution
- **Contents**: 
  - Optimized binaries
  - Compressed archives (`.tar.gz`, `.zip`)
  - SHA256 checksums
  - Release notes

## Docker Integration

### Multi-Platform Builds
```yaml
platforms: linux/amd64,linux/arm64
```

### Registry
- **Location**: GitHub Container Registry (`ghcr.io`)
- **Images**: 
  - `ghcr.io/owner/v6:latest`
  - `ghcr.io/owner/v6:v1.0.0`
  - `ghcr.io/owner/v6:sha-abc123`

### Build Context
Uses the same `Dockerfile` that works with locally built binaries.

## Caching Strategy

### Rust Cache
```yaml
- uses: Swatinem/rust-cache@v2
  with:
    key: ${{ matrix.platform }}
```

### Docker Cache
```yaml
cache-from: type=gha
cache-to: type=gha,mode=max
```

## Triggering Builds

### Automatic Triggers

#### Every Push/PR
- CI workflow runs
- Basic build verification

#### Main Branch
- Full build workflow
- Docker image builds
- Artifact uploads

#### Version Tags
```bash
git tag v1.0.0
git push origin v1.0.0
```
- Release workflow runs
- Optimized builds
- GitHub release creation
- Docker image tagging

### Manual Triggers

#### Workflow Dispatch
Available on all workflows via GitHub UI or CLI:

```bash
gh workflow run build.yml
gh workflow run release.yml -f tag=v1.0.0
```

## Local Development Sync

### Same Commands
What runs in CI can be run locally:

```bash
# CI command:
./build-all.sh --install-targets x86_64-unknown-linux-gnu

# Local equivalent:
./build-all.sh x86_64-unknown-linux-gnu
```

### Same Scripts
- `build-all.sh` works identically
- `Makefile` provides same targets
- Docker builds use same `Dockerfile`

### Same Outputs
- Binary locations are consistent
- Archive formats match
- Checksums use same algorithm

## Troubleshooting

### Common Issues

#### Build Script Not Executable
```yaml
- name: Make build script executable
  if: matrix.os != 'windows-latest'
  run: chmod +x build-all.sh
```

#### Windows Bash Compatibility
```yaml
- name: Build (Windows)
  run: bash build-all.sh --install-targets x86_64-pc-windows-msvc
```

#### Cross-Compilation Failures
- Check target installation
- Verify cross-compilation tools
- Review `cross` tool setup

### Debugging Workflows

#### Artifact Inspection
```bash
# Download artifacts locally
gh run download <run-id>
```

#### Build Script Testing
```bash
# Test script locally with same parameters
./build-all.sh --install-targets x86_64-unknown-linux-gnu

# Check what CI sees
./build-all.sh --list-targets
```

#### Verbose Output
Most workflows include `--verbose` flags for debugging.

## Best Practices

### 1. Consistency
- Use same scripts locally and in CI
- Keep build commands identical
- Test changes locally first

### 2. Efficiency
- Cache dependencies aggressively
- Use matrix builds for parallelization
- Minimize redundant builds

### 3. Reliability
- Test build scripts separately
- Use fail-fast: false for comprehensive testing
- Include smoke tests for binaries

### 4. Security
- Use official actions from trusted sources
- Pin action versions
- Minimize token permissions

## Extending the Workflows

### Adding New Targets
1. Update `build-all.sh` with new target
2. Test locally: `./build-all.sh new-target`
3. Add to workflow matrix if needed
4. Update documentation

### Adding New Platforms
1. Add to workflow matrix:
   ```yaml
   - os: new-os
     targets: "new-target"
     platform: new-platform
   ```
2. Add platform-specific setup steps
3. Test workflow on new platform

### Custom Build Steps
Extend the build process by modifying `build-all.sh` rather than workflow files to maintain consistency.

## Monitoring

### Success Metrics
- Build success rate per platform
- Artifact upload success
- Release creation success
- Docker image push success

### Failure Analysis
- Check workflow logs
- Verify artifact contents
- Test build scripts locally
- Review target compatibility

This documentation should be updated when workflows are modified to keep it current with the actual implementation.