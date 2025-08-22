#!/usr/bin/env bash

# Multi-target build script for V6
# Builds binaries for multiple platforms and architectures

set -e

# Compatible with bash 3.2+ (macOS default)

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Build configuration
PROJECT_NAME="v6"
BUILD_DIR="target/release-builds"
VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')

# Target configurations - using functions instead of associative arrays for compatibility
get_target_description() {
    case "$1" in
        "x86_64-unknown-linux-gnu") echo "Linux x86_64" ;;
        "aarch64-unknown-linux-gnu") echo "Linux ARM64" ;;
        "x86_64-unknown-linux-musl") echo "Linux x86_64 (musl)" ;;
        "aarch64-unknown-linux-musl") echo "Linux ARM64 (musl)" ;;
        "x86_64-apple-darwin") echo "macOS Intel" ;;
        "aarch64-apple-darwin") echo "macOS Apple Silicon" ;;
        "x86_64-pc-windows-msvc") echo "Windows x86_64" ;;
        "x86_64-pc-windows-gnu") echo "Windows x86_64 (GNU)" ;;
        *) echo "Unknown target" ;;
    esac
}

# List of all supported targets
ALL_TARGETS=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu" 
    "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-musl"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-msvc"
    "x86_64-pc-windows-gnu"
)

# Function to check if target is valid
is_valid_target() {
    local target="$1"
    for valid_target in "${ALL_TARGETS[@]}"; do
        if [[ "$target" == "$valid_target" ]]; then
            return 0
        fi
    done
    return 1
}

# Function to print colored output
print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${CYAN}===================================================${NC}"
    echo -e "${CYAN} $1${NC}"
    echo -e "${CYAN}===================================================${NC}"
}

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to install rust targets
install_targets() {
    print_header "Installing Rust Targets"
    
    for target in "${ALL_TARGETS[@]}"; do
        print_info "Installing target: $target"
        if rustup target add "$target"; then
            print_success "Target $target installed"
        else
            print_warning "Failed to install target $target"
        fi
    done
}

# Function to check build requirements
check_requirements() {
    print_header "Checking Build Requirements"
    
    # Check Rust
    if ! command_exists rustc; then
        print_error "Rust is not installed"
        exit 1
    fi
    
    # Check cargo
    if ! command_exists cargo; then
        print_error "Cargo is not installed"
        exit 1
    fi
    
    print_info "Rust version: $(rustc --version)"
    print_info "Cargo version: $(cargo --version)"
    
    # Check cross if available
    if command_exists cross; then
        print_info "Cross tool available: $(cross --version)"
        CROSS_AVAILABLE=true
    else
        print_warning "Cross tool not available. Install with: cargo install cross --git https://github.com/cross-rs/cross"
        CROSS_AVAILABLE=false
    fi
    
    # Check Docker if available
    if command_exists docker && docker info >/dev/null 2>&1; then
        print_info "Docker available"
        DOCKER_AVAILABLE=true
    else
        print_warning "Docker not available or not running"
        DOCKER_AVAILABLE=false
    fi
}

# Function to determine build method for target
get_build_method() {
    local target="$1"
    local host_os=$(uname -s)
    local host_arch=$(uname -m)
    
    # Determine if we can build natively
    case "$host_os" in
        "Darwin")
            case "$target" in
                "x86_64-apple-darwin"|"aarch64-apple-darwin")
                    echo "native"
                    ;;
                *)
                    if [[ "$CROSS_AVAILABLE" == true ]]; then
                        echo "cross"
                    elif [[ "$DOCKER_AVAILABLE" == true ]]; then
                        echo "docker"
                    else
                        echo "skip"
                    fi
                    ;;
            esac
            ;;
        "Linux")
            case "$target" in
                "x86_64-unknown-linux-gnu"|"x86_64-unknown-linux-musl")
                    echo "native"
                    ;;
                *)
                    if [[ "$CROSS_AVAILABLE" == true ]]; then
                        echo "cross"
                    else
                        echo "skip"
                    fi
                    ;;
            esac
            ;;
        *)
            if [[ "$CROSS_AVAILABLE" == true ]]; then
                echo "cross"
            else
                echo "skip"
            fi
            ;;
    esac
}

# Function to build for a specific target
build_target() {
    local target="$1"
    local description=$(get_target_description "$target")
    local method=$(get_build_method "$target")
    
    print_info "Building for $target ($description) using method: $method"
    
    case "$method" in
        "native")
            build_native "$target"
            ;;
        "cross")
            build_cross "$target"
            ;;
        "docker")
            build_docker "$target"
            ;;
        "skip")
            print_warning "Skipping $target - no suitable build method available"
            return 1
            ;;
    esac
}

# Function to build using native cargo
build_native() {
    local target="$1"
    
    print_info "Building $target with native cargo..."
    
    if cargo build --release --target "$target"; then
        copy_binary "$target"
        print_success "Successfully built $target"
        return 0
    else
        print_error "Failed to build $target"
        return 1
    fi
}

# Function to build using cross
build_cross() {
    local target="$1"
    
    print_info "Building $target with cross..."
    
    if cross build --release --target "$target"; then
        copy_binary "$target"
        print_success "Successfully built $target with cross"
        return 0
    else
        print_error "Failed to build $target with cross"
        return 1
    fi
}

# Function to build using Docker
build_docker() {
    local target="$1"
    
    print_info "Building $target with Docker..."
    
    local docker_image="rust:1.75"
    
    if docker run --rm \
        -v "$(pwd)":/workspace \
        -w /workspace \
        "$docker_image" \
        bash -c "
            rustup target add $target &&
            cargo build --release --target $target
        "; then
        copy_binary "$target"
        print_success "Successfully built $target with Docker"
        return 0
    else
        print_error "Failed to build $target with Docker"
        return 1
    fi
}

# Function to copy and rename binary
copy_binary() {
    local target="$1"
    local binary_name="$PROJECT_NAME"
    local extension=""
    
    # Add .exe extension for Windows targets
    case "$target" in
        *windows*)
            extension=".exe"
            ;;
    esac
    
    local source_path="target/$target/release/$binary_name$extension"
    local dest_name="$PROJECT_NAME-$target$extension"
    local dest_path="$BUILD_DIR/$dest_name"
    
    if [[ -f "$source_path" ]]; then
        mkdir -p "$BUILD_DIR"
        cp "$source_path" "$dest_path"
        
        # Make executable on Unix systems
        if [[ "$extension" != ".exe" ]]; then
            chmod +x "$dest_path"
        fi
        
        print_info "Binary copied to: $dest_path"
        
        # Show binary info
        local size=$(du -h "$dest_path" | cut -f1)
        print_info "Binary size: $size"
    else
        print_error "Binary not found at: $source_path"
        return 1
    fi
}

# Function to create archives
create_archives() {
    print_header "Creating Archives"
    
    cd "$BUILD_DIR"
    
    for binary in $PROJECT_NAME-*; do
        if [[ -f "$binary" ]]; then
            case "$binary" in
                *.exe)
                    # Create ZIP for Windows
                    local archive_name="${binary%.exe}.zip"
                    print_info "Creating archive: $archive_name"
                    zip -q "$archive_name" "$binary"
                    ;;
                *)
                    # Create tar.gz for Unix systems
                    local archive_name="$binary.tar.gz"
                    print_info "Creating archive: $archive_name"
                    tar -czf "$archive_name" "$binary"
                    ;;
            esac
        fi
    done
    
    cd - >/dev/null
}

# Function to create checksums
create_checksums() {
    print_header "Creating Checksums"
    
    cd "$BUILD_DIR"
    
    for file in *.tar.gz *.zip; do
        if [[ -f "$file" ]]; then
            print_info "Creating checksum for: $file"
            
            # Create SHA256 checksum
            if command_exists sha256sum; then
                sha256sum "$file" > "$file.sha256"
            elif command_exists shasum; then
                shasum -a 256 "$file" > "$file.sha256"
            else
                print_warning "No SHA256 tool available for $file"
            fi
        fi
    done
    
    cd - >/dev/null
}

# Function to show build summary
show_summary() {
    print_header "Build Summary"
    
    print_info "Project: $PROJECT_NAME v$VERSION"
    print_info "Build directory: $BUILD_DIR"
    
    if [[ -d "$BUILD_DIR" ]]; then
        echo
        print_info "Built artifacts:"
        
        cd "$BUILD_DIR"
        
        for file in *; do
            if [[ -f "$file" ]]; then
                local size=$(du -h "$file" | cut -f1)
                printf "  %-40s %s\n" "$file" "$size"
            fi
        done
        
        cd - >/dev/null
    fi
    
    echo
    print_success "Build completed! Check the '$BUILD_DIR' directory for artifacts."
}

# Function to clean build directory
clean_builds() {
    if [[ -d "$BUILD_DIR" ]]; then
        print_info "Cleaning previous builds..."
        rm -rf "$BUILD_DIR"
    fi
}

# Function to show help
show_help() {
    echo "Usage: $0 [OPTIONS] [TARGETS...]"
    echo
    echo "Build V6 binaries for multiple targets"
    echo
    echo "Options:"
    echo "  --help, -h          Show this help"
    echo "  --clean             Clean build directory before building"
    echo "  --no-archives       Don't create archives"
    echo "  --no-checksums      Don't create checksums"
    echo "  --install-targets   Install Rust targets before building"
    echo "  --list-targets      List available targets"
    echo
    echo "Targets:"
    echo "  If no targets specified, builds for all supported targets"
    echo "  Available targets:"
    for target in "${ALL_TARGETS[@]}"; do
        printf "    %-30s %s\n" "$target" "$(get_target_description "$target")"
    done
    echo
    echo "Examples:"
    echo "  $0                                    # Build all targets"
    echo "  $0 x86_64-unknown-linux-gnu          # Build only Linux x86_64"
    echo "  $0 --clean x86_64-apple-darwin       # Clean and build macOS Intel"
    echo "  $0 --install-targets --clean         # Install targets, clean, and build all"
}

# Main function
main() {
    local targets_to_build=()
    local clean=false
    local create_archives_flag=true
    local create_checksums_flag=true
    local install_targets_flag=false
    
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            --help|-h)
                show_help
                exit 0
                ;;
            --clean)
                clean=true
                shift
                ;;
            --no-archives)
                create_archives_flag=false
                shift
                ;;
            --no-checksums)
                create_checksums_flag=false
                shift
                ;;
            --install-targets)
                install_targets_flag=true
                shift
                ;;
            --list-targets)
                echo "Available targets:"
                for target in "${ALL_TARGETS[@]}"; do
                    printf "  %-30s %s\n" "$target" "$(get_target_description "$target")"
                done
                exit 0
                ;;
            --*)
                print_error "Unknown option: $1"
                exit 1
                ;;
            *)
                # Check if it's a valid target
                if is_valid_target "$1"; then
                    targets_to_build+=("$1")
                else
                    print_error "Unknown target: $1"
                    exit 1
                fi
                shift
                ;;
        esac
    done
    
    # If no targets specified, build all
    if [[ ${#targets_to_build[@]} -eq 0 ]]; then
        targets_to_build=("${ALL_TARGETS[@]}")
    fi
    
    print_header "V6 Multi-Target Build Script"
    
    # Check requirements
    check_requirements
    
    # Install targets if requested
    if [[ "$install_targets_flag" == true ]]; then
        install_targets
    fi
    
    # Clean if requested
    if [[ "$clean" == true ]]; then
        clean_builds
    fi
    
    # Build targets
    print_header "Building Targets"
    
    local successful_builds=0
    local failed_builds=0
    
    for target in "${targets_to_build[@]}"; do
        echo
        if build_target "$target"; then
            ((successful_builds++))
        else
            ((failed_builds++))
        fi
    done
    
    echo
    print_info "Build results: $successful_builds successful, $failed_builds failed"
    
    # Create archives if requested and we have successful builds
    if [[ "$create_archives_flag" == true && $successful_builds -gt 0 ]]; then
        echo
        create_archives
    fi
    
    # Create checksums if requested
    if [[ "$create_checksums_flag" == true && $successful_builds -gt 0 ]]; then
        echo
        create_checksums
    fi
    
    # Show summary
    echo
    show_summary
    
    # Exit with appropriate code
    if [[ $failed_builds -gt 0 ]]; then
        exit 1
    fi
}

# Run main function
main "$@"