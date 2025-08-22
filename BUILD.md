# Build Guide for V6

This guide covers all methods to build V6 binaries for multiple platforms and architectures.

## Table of Contents

- [Quick Start](#quick-start)
- [Prerequisites](#prerequisites)
- [Build Methods](#build-methods)
- [Platform-Specific Instructions](#platform-specific-instructions)
- [Troubleshooting](#troubleshooting)
- [Advanced Building](#advanced-building)

## Quick Start

### Simple Builds

```bash
# Build for current platform
cargo build --release

# Quick cross-platform builds
./quick-build.sh all

# Or using Make
make all
```

### Using the Build Scripts

```bash
# Comprehensive multi-target build
./build-all.sh

# Build specific targets
./build-all.sh x86_64-unknown-linux-gnu aarch64-apple-darwin

# Install targets and build everything
./build-all.sh --install-targets --clean
```

## Prerequisites

### Required Tools

1. **Rust Toolchain** (1.75+)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup update
   ```

2. **Git**
   ```bash
   # macOS
   xcode-select --install
   
   # Ubuntu/Debian
   sudo apt-get install git
   
   # Windows
   # Download from https://git-scm.com/
   ```

### Optional Tools

1. **Cross-compilation tool**
   ```bash
   cargo install cross --git https://github.com/cross-rs/cross
   ```

2. **Docker** (for cross-platform builds)
   - Download from https://docker.com/
   - Required for `cross` tool and Docker builds

3. **Make** (for convenience targets)
   ```bash
   # Usually pre-installed on Unix systems
   # Windows: Install via Chocolatey or use WSL
   ```

## Build Methods

### Method 1: Native Cargo Build

Build for your current platform:

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Specific target
cargo build --release --target x86_64-unknown-linux-gnu
```

**Pros:**
- Fast compilation
- No additional setup
- Full optimization

**Cons:**
- Single platform only
- May require cross-compilation setup for other targets

### Method 2: Quick Build Script

Use the simple build script for common scenarios:

```bash
# Show available commands
./quick-build.sh help

# Build for current platform
./quick-build.sh native

# Build for specific platforms
./quick-build.sh linux
./quick-build.sh macos
./quick-build.sh windows

# Build everything
./quick-build.sh all
```

### Method 3: Comprehensive Build Script

Use the full-featured build script:

```bash
# Build all supported targets
./build-all.sh

# Build specific targets
./build-all.sh x86_64-unknown-linux-gnu aarch64-apple-darwin

# Clean build with target installation
./build-all.sh --install-targets --clean

# Create archives and checksums
./build-all.sh --clean  # (archives are created by default)

# Skip archives
./build-all.sh --no-archives

# List all available targets
./build-all.sh --list-targets
```

### Method 4: Makefile Targets

Use Make for convenient builds:

```bash
# Show all available targets
make help

# Build everything
make all

# Platform-specific builds
make linux      # All Linux targets
make macos      # All macOS targets  
make windows    # All Windows targets
make musl       # All musl targets

# Development workflow
make setup      # Install targets and tools
make dev-test   # Format, lint, and test
make package    # Build, archive, and checksum

# Quick individual builds
make quick-linux
make quick-macos
make quick-windows
```

### Method 5: Cross-Compilation Tool

Use the `cross` tool for robust cross-compilation:

```bash
# Install cross
cargo install cross --git https://github.com/cross-rs/cross

# Build for different targets
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target aarch64-unknown-linux-gnu
cross build --release --target x86_64-pc-windows-msvc
```

### Method 6: Docker Builds

Use Docker for isolated, reproducible builds:

```bash
# Basic Docker build
docker run --rm -v "$(pwd)":/workspace -w /workspace \
  rust:1.75 cargo build --release

# Multi-target Docker build
docker run --rm -v "$(pwd)":/workspace -w /workspace \
  rust:1.75 bash -c "
    rustup target add x86_64-unknown-linux-gnu &&
    cargo build --release --target x86_64-unknown-linux-gnu
  "

# Using our test script
./test-linux.sh docker
```

## Platform-Specific Instructions

### Building on macOS

#### For macOS Targets
```bash
# Native builds work out of the box
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

#### For Linux Targets
```bash
# Option 1: Use Docker (recommended)
docker run --rm -v "$(pwd)":/workspace -w /workspace \
  rust:1.75 cargo build --release --target x86_64-unknown-linux-gnu

# Option 2: Use cross
cross build --release --target x86_64-unknown-linux-gnu

# Option 3: Install cross-compilation tools (complex)
# Not recommended - use Docker or cross instead
```

#### For Windows Targets
```bash
# Use cross (requires Docker)
cross build --release --target x86_64-pc-windows-msvc

# Or try direct (may fail without proper setup)
cargo build --release --target x86_64-pc-windows-msvc
```

### Building on Linux

#### For Linux Targets
```bash
# Native builds
cargo build --release --target x86_64-unknown-linux-gnu

# For ARM64
sudo apt-get install gcc-aarch64-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu

# For musl
sudo apt-get install musl-tools
cargo build --release --target x86_64-unknown-linux-musl
```

#### For macOS Targets
```bash
# Use cross (requires Docker)
cross build --release --target x86_64-apple-darwin
cross build --release --target aarch64-apple-darwin
```

#### For Windows Targets
```bash
# Use cross (requires Docker)
cross build --release --target x86_64-pc-windows-msvc

# Or install mingw
sudo apt-get install gcc-mingw-w64
cargo build --release --target x86_64-pc-windows-gnu
```

### Building on Windows

#### For Windows Targets
```bash
# Native builds (requires MSVC)
cargo build --release --target x86_64-pc-windows-msvc

# Or with MinGW
cargo build --release --target x86_64-pc-windows-gnu
```

#### For Linux/macOS Targets
```bash
# Use cross with Docker Desktop
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target x86_64-apple-darwin
```

#### WSL2 Option
```bash
# Use WSL2 for Linux-like environment
wsl --install
# Then follow Linux instructions in WSL2
```

## Supported Targets

| Target | Platform | Architecture | Notes |
|--------|----------|--------------|-------|
| `x86_64-unknown-linux-gnu` | Linux | x86_64 | Standard Linux |
| `aarch64-unknown-linux-gnu` | Linux | ARM64 | Linux ARM64 |
| `x86_64-unknown-linux-musl` | Linux | x86_64 | Static linking |
| `aarch64-unknown-linux-musl` | Linux | ARM64 | Static ARM64 |
| `x86_64-apple-darwin` | macOS | Intel | macOS Intel |
| `aarch64-apple-darwin` | macOS | Apple Silicon | macOS ARM64 |
| `x86_64-pc-windows-msvc` | Windows | x86_64 | Windows MSVC |
| `x86_64-pc-windows-gnu` | Windows | x86_64 | Windows MinGW |

## Build Outputs

### Binary Locations

After building, binaries are located at:

```
target/
├── release/
│   └── v6                           # Native binary
├── x86_64-unknown-linux-gnu/
│   └── release/
│       └── v6                       # Linux x86_64 binary
├── aarch64-apple-darwin/
│   └── release/
│       └── v6                       # macOS ARM64 binary
└── x86_64-pc-windows-msvc/
    └── release/
        └── v6.exe                   # Windows binary
```

### Organized Outputs

The build scripts create organized outputs in `target/release-builds/`:

```
target/release-builds/
├── v6-x86_64-unknown-linux-gnu     # Linux binary
├── v6-x86_64-unknown-linux-gnu.tar.gz
├── v6-x86_64-unknown-linux-gnu.tar.gz.sha256
├── v6-aarch64-apple-darwin          # macOS ARM64 binary  
├── v6-aarch64-apple-darwin.tar.gz
├── v6-aarch64-apple-darwin.tar.gz.sha256
├── v6-x86_64-pc-windows-msvc.exe   # Windows binary
├── v6-x86_64-pc-windows-msvc.zip
└── v6-x86_64-pc-windows-msvc.zip.sha256
```

## Troubleshooting

### Common Issues

#### "failed to find tool 'x86_64-linux-gnu-gcc'"

**Problem:** Missing cross-compilation toolchain.

**Solutions:**
```bash
# Use Docker (recommended)
docker run --rm -v "$(pwd)":/workspace -w /workspace \
  rust:1.75 cargo build --release --target x86_64-unknown-linux-gnu

# Or use cross
cross build --release --target x86_64-unknown-linux-gnu
```

#### "aws-lc-sys" build errors

**Problem:** Native dependencies require specific build tools.

**Solutions:**
```bash
# On Ubuntu/Debian
sudo apt-get update
sudo apt-get install build-essential cmake pkg-config libssl-dev

# On macOS
xcode-select --install
brew install cmake pkg-config openssl

# Or use Docker
docker run --rm -v "$(pwd)":/workspace -w /workspace \
  rust:1.75 bash -c "
    apt-get update &&
    apt-get install -y build-essential cmake pkg-config libssl-dev &&
    cargo build --release
  "
```

#### Slow compilation

**Solutions:**
```bash
# Use more parallel jobs
cargo build --release -j 8

# Use sccache for caching
cargo install sccache
export RUSTC_WRAPPER=sccache

# Use release mode for faster runtime
cargo build --release
```

#### Docker not available

**Solutions:**
```bash
# Install Docker Desktop
# macOS: Download from docker.com
# Linux: Use package manager
# Windows: Download Docker Desktop

# Or use native builds where possible
./quick-build.sh native

# Or install cross-compilation tools manually
```

### Platform-Specific Issues

#### macOS

- **Issue:** Old bash version
  ```bash
  # Install newer bash if needed
  brew install bash
  # Use full path: /usr/local/bin/bash ./build-all.sh
  ```

- **Issue:** Xcode license
  ```bash
  sudo xcodebuild -license accept
  ```

#### Linux

- **Issue:** Missing musl tools
  ```bash
  sudo apt-get install musl-tools
  ```

- **Issue:** Missing ARM64 cross-compiler
  ```bash
  sudo apt-get install gcc-aarch64-linux-gnu
  ```

#### Windows

- **Issue:** MSVC not found
  ```bash
  # Install Visual Studio or Build Tools
  # https://visualstudio.microsoft.com/downloads/
  ```

- **Issue:** Long path names
  ```bash
  # Enable long paths in Windows
  # Use WSL2 as alternative
  ```

## Advanced Building

### Custom Build Configurations

#### Optimized Release Build

Add to `Cargo.toml`:
```toml
[profile.release]
lto = true           # Link-time optimization
codegen-units = 1    # Single codegen unit for better optimization
panic = "abort"      # Smaller binaries
strip = true         # Strip debug symbols
```

#### Cross-compilation with Custom Linkers

```bash
# Set custom linker for target
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
cargo build --release --target aarch64-unknown-linux-gnu
```

### Environment Variables

```bash
# Optimization settings
export CARGO_INCREMENTAL=1       # Enable incremental compilation
export RUSTFLAGS="-C target-cpu=native"  # Optimize for current CPU

# Build tool settings
export CROSS_CONTAINER_ENGINE=docker     # Use Docker with cross
export RUSTC_WRAPPER=sccache             # Use sccache for caching

# Build for current target
cargo build --release
```

### CI/CD Integration

The project includes GitHub Actions workflows that automatically:

1. **Test on every push** (`ci.yml`)
2. **Build multi-platform binaries on tags** (`build.yml`)
3. **Create releases with artifacts** (`release.yml`)

To trigger a release build:
```bash
git tag v1.0.0
git push origin v1.0.0
```

### Custom Docker Images

Create a custom build environment:

```dockerfile
FROM rust:1.75
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev \
    musl-tools \
    gcc-aarch64-linux-gnu
WORKDIR /workspace
```

```bash
docker build -t v6-builder .
docker run --rm -v "$(pwd)":/workspace v6-builder cargo build --release
```

## Quick Reference

### Essential Commands

```bash
# Quick builds
cargo build --release                    # Native
./quick-build.sh all                     # All platforms
make package                             # Build + archive

# Cross-compilation
cross build --release --target <TARGET>  # Using cross
docker run --rm -v "$(pwd)":/workspace -w /workspace rust:1.75 cargo build --release

# Comprehensive
./build-all.sh --install-targets --clean # Full multi-target build
make setup && make release-build         # Complete setup and build
```

### Targets Quick Reference

```bash
# Add targets
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-msvc

# Build for targets
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-apple-darwin  
cargo build --release --target x86_64-pc-windows-msvc
```

### File Locations

- **Source:** `src/`
- **Native binary:** `target/release/v6`
- **Cross-compiled:** `target/<TARGET>/release/v6[.exe]`
- **Organized outputs:** `target/release-builds/`
- **Build scripts:** `build-all.sh`, `quick-build.sh`
- **Make targets:** `Makefile`

For more specific help, see:
- `./build-all.sh --help`
- `./quick-build.sh help`
- `make help`
- [TESTING.md](TESTING.md) for testing instructions
- [.github/workflows/](/.github/workflows/) for CI/CD examples