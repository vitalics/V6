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

This project uses GitHub Actions for continuous integration and releases:

- **CI Pipeline**: Runs tests, linting, and builds on every push
- **Release Pipeline**: Creates cross-platform binaries and Docker images on tagged releases
- **Supported Platforms**: Linux, macOS, Windows (x86_64 and ARM64 where applicable)

### Creating Releases

To create a new release, simply push a git tag starting with `v`:

```bash
git tag v1.0.0
git push origin v1.0.0
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
