# Makefile for V6 multi-target builds
# Provides convenient targets for building binaries across multiple platforms

.PHONY: all clean install-targets help test lint fmt build-native build-cross build-docker
.PHONY: linux macos windows musl release archives checksums

# Project configuration
PROJECT_NAME := v6
VERSION := $(shell grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
BUILD_DIR := target/release-builds

# Target definitions
LINUX_TARGETS := x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu
MACOS_TARGETS := x86_64-apple-darwin aarch64-apple-darwin
WINDOWS_TARGETS := x86_64-pc-windows-msvc x86_64-pc-windows-gnu
MUSL_TARGETS := x86_64-unknown-linux-musl aarch64-unknown-linux-musl
ALL_TARGETS := $(LINUX_TARGETS) $(MACOS_TARGETS) $(WINDOWS_TARGETS) $(MUSL_TARGETS)

# Default target
all: build-all

# Help target
help:
	@echo "V6 Build System"
	@echo "==============="
	@echo ""
	@echo "Targets:"
	@echo "  all              Build for all supported targets (default)"
	@echo "  build-all        Build for all targets using build script"
	@echo "  linux            Build for Linux targets"
	@echo "  macos            Build for macOS targets"
	@echo "  windows          Build for Windows targets"
	@echo "  musl             Build for musl targets"
	@echo "  native           Build for current platform only"
	@echo "  release          Build optimized release binary"
	@echo ""
	@echo "Development:"
	@echo "  test             Run all tests"
	@echo "  test-cross       Run cross-platform tests"
	@echo "  lint             Run clippy linter"
	@echo "  fmt              Format code"
	@echo "  check            Quick syntax check"
	@echo ""
	@echo "Setup:"
	@echo "  install-targets  Install all Rust targets"
	@echo "  install-cross    Install cross-compilation tool"
	@echo "  setup            Install targets and cross tool"
	@echo ""
	@echo "Packaging:"
	@echo "  archives         Create archives of built binaries"
	@echo "  checksums        Create SHA256 checksums"
	@echo "  package          Build, archive, and checksum"
	@echo ""
	@echo "Cleanup:"
	@echo "  clean            Clean all build artifacts"
	@echo "  clean-target     Clean cargo target directory"
	@echo ""
	@echo "Docker:"
	@echo "  docker-build     Build Docker image"
	@echo "  docker-test      Run tests in Docker"
	@echo ""
	@echo "Variables:"
	@echo "  PROJECT_NAME=$(PROJECT_NAME)"
	@echo "  VERSION=$(VERSION)"
	@echo "  BUILD_DIR=$(BUILD_DIR)"

# Main build targets
build-all:
	@echo "Building all targets..."
	./build-all.sh

build-clean:
	@echo "Clean building all targets..."
	./build-all.sh --clean

# Platform-specific builds
linux:
	@echo "Building Linux targets..."
	V8_FROM_SOURCE=1 ./build-all.sh $(LINUX_TARGETS)

macos:
	@echo "Building macOS targets..."
	V8_FROM_SOURCE=1 ./build-all.sh $(MACOS_TARGETS)

windows:
	@echo "Building Windows targets..."
	V8_FROM_SOURCE=1 ./build-all.sh $(WINDOWS_TARGETS)

musl:
	@echo "Building musl targets..."
	V8_FROM_SOURCE=1 ./build-all.sh $(MUSL_TARGETS)

# Native build
native:
	@echo "Building for native target..."
	cargo build --release

release: native

# Individual target builds (using cargo directly)
build-linux-x64:
	cargo build --release --target x86_64-unknown-linux-gnu

build-linux-arm64:
	cargo build --release --target aarch64-unknown-linux-gnu

build-macos-x64:
	cargo build --release --target x86_64-apple-darwin

build-macos-arm64:
	cargo build --release --target aarch64-apple-darwin

build-windows-x64:
	cargo build --release --target x86_64-pc-windows-msvc

build-musl-x64:
	cargo build --release --target x86_64-unknown-linux-musl

# Cross-compilation builds
build-cross:
	@if command -v cross >/dev/null 2>&1; then \
		echo "Building with cross..."; \
		for target in $(ALL_TARGETS); do \
			echo "Building $$target with cross..."; \
			cross build --release --target $$target || echo "Failed to build $$target"; \
		done; \
	else \
		echo "Cross not installed. Install with: make install-cross"; \
		exit 1; \
	fi

# Docker builds
build-docker:
	@if command -v docker >/dev/null 2>&1; then \
		echo "Building with Docker..."; \
		docker run --rm -v "$$(pwd)":/workspace -w /workspace rust:1.75 \
			bash -c "cargo build --release"; \
	else \
		echo "Docker not available"; \
		exit 1; \
	fi

docker-build:
	docker build -t $(PROJECT_NAME):latest .

# Testing
test:
	cargo test --verbose

test-release:
	cargo test --release --verbose

test-cross:
	./test-linux.sh

test-docker:
	./test-linux.sh docker

test-native:
	./test-linux.sh native

test-quality:
	./test-linux.sh quality

# Code quality
check:
	cargo check

lint:
	cargo clippy -- -D warnings

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

# Setup targets
install-targets:
	@echo "Installing Rust targets..."
	@for target in $(ALL_TARGETS); do \
		echo "Installing $$target..."; \
		rustup target add $$target; \
	done

install-cross:
	@echo "Installing cross..."
	cargo install cross --git https://github.com/cross-rs/cross

setup: install-targets install-cross
	@echo "Setup complete!"

# Packaging
archives:
	@echo "Creating archives..."
	@cd $(BUILD_DIR) && \
	for binary in $(PROJECT_NAME)-*; do \
		if [ -f "$$binary" ]; then \
			case "$$binary" in \
				*.exe) \
					archive="$${binary%.exe}.zip"; \
					echo "Creating $$archive..."; \
					zip -q "$$archive" "$$binary" ;; \
				*) \
					archive="$$binary.tar.gz"; \
					echo "Creating $$archive..."; \
					tar -czf "$$archive" "$$binary" ;; \
			esac; \
		fi; \
	done

checksums:
	@echo "Creating checksums..."
	@cd $(BUILD_DIR) && \
	for file in *.tar.gz *.zip; do \
		if [ -f "$$file" ]; then \
			echo "Creating checksum for $$file..."; \
			if command -v sha256sum >/dev/null 2>&1; then \
				sha256sum "$$file" > "$$file.sha256"; \
			elif command -v shasum >/dev/null 2>&1; then \
				shasum -a 256 "$$file" > "$$file.sha256"; \
			fi; \
		fi; \
	done

package: build-all archives checksums
	@echo "Package complete! Check $(BUILD_DIR) for artifacts."

# Cleanup
clean:
	rm -rf $(BUILD_DIR)
	cargo clean

clean-target:
	rm -rf target/

# Development workflow
dev-setup: setup
	@echo "Development environment ready!"

dev-test: fmt-check lint test
	@echo "All development checks passed!"

dev-build: dev-test native
	@echo "Development build complete!"

# CI simulation
ci-test: fmt-check lint test test-cross
	@echo "CI tests complete!"

ci-build: ci-test build-all
	@echo "CI build complete!"

# Release workflow
release-prep: clean ci-test
	@echo "Release preparation complete!"

release-build: release-prep build-all archives checksums
	@echo "Release build complete!"

# Quick targets for common operations
quick-linux:
	cargo build --release --target x86_64-unknown-linux-gnu

quick-macos:
	cargo build --release --target x86_64-apple-darwin

quick-windows:
	cross build --release --target x86_64-pc-windows-msvc

# Show build status
status:
	@echo "Project: $(PROJECT_NAME) v$(VERSION)"
	@echo "Build directory: $(BUILD_DIR)"
	@echo ""
	@echo "Available targets:"
	@for target in $(ALL_TARGETS); do \
		if rustup target list --installed | grep -q "$$target"; then \
			echo "  ✓ $$target (installed)"; \
		else \
			echo "  ✗ $$target (not installed)"; \
		fi; \
	done
	@echo ""
	@echo "Tools:"
	@if command -v cross >/dev/null 2>&1; then \
		echo "  ✓ cross ($(shell cross --version 2>/dev/null | head -n1))"; \
	else \
		echo "  ✗ cross (not installed)"; \
	fi
	@if command -v docker >/dev/null 2>&1 && docker info >/dev/null 2>&1; then \
		echo "  ✓ docker (running)"; \
	elif command -v docker >/dev/null 2>&1; then \
		echo "  ⚠ docker (not running)"; \
	else \
		echo "  ✗ docker (not installed)"; \
	fi
	@echo ""
	@if [ -d "$(BUILD_DIR)" ]; then \
		echo "Built artifacts:"; \
		ls -la $(BUILD_DIR)/ 2>/dev/null || echo "  (none)"; \
	else \
		echo "No build artifacts found."; \
	fi

# Version bump helpers
version-patch:
	@current_version=$$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'); \
	new_version=$$(echo $$current_version | awk -F. '{print $$1"."$$2"."$$3+1}'); \
	sed -i.bak "s/version = \"$$current_version\"/version = \"$$new_version\"/" Cargo.toml; \
	rm -f Cargo.toml.bak; \
	echo "Version bumped from $$current_version to $$new_version"

version-minor:
	@current_version=$$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'); \
	new_version=$$(echo $$current_version | awk -F. '{print $$1"."$$2+1".0"}'); \
	sed -i.bak "s/version = \"$$current_version\"/version = \"$$new_version\"/" Cargo.toml; \
	rm -f Cargo.toml.bak; \
	echo "Version bumped from $$current_version to $$new_version"

version-major:
	@current_version=$$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/'); \
	new_version=$$(echo $$current_version | awk -F. '{print $$1+1".0.0"}'); \
	sed -i.bak "s/version = \"$$current_version\"/version = \"$$new_version\"/" Cargo.toml; \
	rm -f Cargo.toml.bak; \
	echo "Version bumped from $$current_version to $$new_version"
