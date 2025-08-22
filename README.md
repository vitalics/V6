# V6

[![CI](https://github.com/your-username/your-repo/workflows/CI/badge.svg)](https://github.com/your-username/your-repo/actions/workflows/ci.yml)
[![Release](https://github.com/your-username/your-repo/workflows/Release/badge.svg)](https://github.com/your-username/your-repo/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

V6 - modern load testing tool using V8 and inspired by K6

## Installation

### Download Pre-built Binaries

Pre-built binaries are available for download from the [releases page](https://github.com/your-username/your-repo/releases). We provide binaries for:

- **Linux**: x86_64, aarch64 (including musl variants)
- **macOS**: x86_64 (Intel), aarch64 (Apple Silicon)  
- **Windows**: x86_64

#### Quick Install (Linux/macOS)

```bash
# Download and install the latest release
curl -L https://github.com/your-username/your-repo/releases/latest/download/v6-linux-x86_64.tar.gz | tar xz
sudo mv v6-linux-x86_64 /usr/local/bin/v6

# Or for macOS
curl -L https://github.com/your-username/your-repo/releases/latest/download/v6-macos-x86_64.tar.gz | tar xz
sudo mv v6-macos-x86_64 /usr/local/bin/v6
```

#### Manual Installation

1. Download the appropriate binary for your platform from the releases page
2. Extract the archive: `tar -xzf v6-*.tar.gz` (Linux/macOS) or unzip (Windows)
3. Add the binary to your PATH or move it to `/usr/local/bin/` (Unix) or a directory in your PATH (Windows)

### Build from Source

To build from source, you need Rust 1.75 or later:

```bash
git clone https://github.com/your-username/your-repo.git
cd your-repo
cargo build --release
```

The binary will be available at `target/release/v6`.

### Docker

You can also run V6 using Docker:

```bash
# Pull the latest image
docker pull ghcr.io/your-username/v6:latest

# Run a load test
docker run --rm -v $(pwd):/tests ghcr.io/your-username/v6:latest run /tests/your-test.js
```

### Build Script Integration

The CI/CD system is built around the same scripts you use locally:

- **`build-all.sh`**: Used by GitHub Actions for consistent cross-platform builds
- **`quick-build.sh`**: Available for rapid local development
- **`Makefile`**: Provides convenient targets that wrap the build scripts

This ensures that builds are reproducible and consistent between local development and CI environments.

## Usage

```bash
# Initialize a new test file
v6 init test.js

# Run a load test
v6 run test.js

# Run with custom parameters
v6 run test.js --iterations 1000 --duration 30.0 --vus 10
```

## CI/CD

This project uses GitHub Actions for continuous integration and releases, leveraging the `build-all.sh` script for consistent builds:

### Workflows

- **CI Pipeline** (`.github/workflows/ci.yml`): Runs tests, linting, and cross-platform builds on every push
- **Build Pipeline** (`.github/workflows/build.yml`): Creates multi-platform binaries using `build-all.sh`
- **Release Pipeline** (`.github/workflows/release.yml`): Full release builds with archives and checksums
- **Build Script Testing** (`.github/workflows/test-build-script.yml`): Tests the build scripts across platforms

### GitHub Actions Integration

The workflows use the same `build-all.sh` script you can run locally:

```yaml
# Example from our CI
- name: Install targets and build
  run: |
    ./build-all.sh --install-targets x86_64-unknown-linux-gnu aarch64-apple-darwin
```

### Supported Platforms in CI

- **Linux**: x86_64, ARM64 (both GNU and musl)
- **macOS**: Intel x86_64, Apple Silicon ARM64  
- **Windows**: x86_64 (MSVC)

### Creating Releases

To create a new release, simply push a git tag starting with `v`:

```bash
git tag v1.0.0
git push origin v1.0.0
```

This automatically:
- Runs the full test suite
- Builds binaries for all platforms using `build-all.sh`
- Creates compressed archives with checksums
- Generates a GitHub release with all artifacts
- Builds and pushes Docker images

### Local vs CI Consistency

Both local development and CI use the same build scripts:

```bash
# What you run locally:
./build-all.sh --clean

# What CI runs:
./build-all.sh --install-targets --clean x86_64-unknown-linux-gnu aarch64-apple-darwin
```

This will automatically:
- Build binaries for all supported platforms
- Create compressed archives with checksums
- Generate a GitHub release with all artifacts
- Build and push Docker images to GitHub Container Registry

### Development

```bash
# Clone the repository
git clone https://github.com/your-username/your-repo.git
cd your-repo

# Run tests
cargo test

# Build for development
cargo build

# Run with cargo
cargo run -- --help
```
