#!/bin/bash
set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Helper functions
print_header() {
    echo -e "${BLUE}════════════════════════════════════════${NC}"
    echo -e "${CYAN}$1${NC}"
    echo -e "${BLUE}════════════════════════════════════════${NC}"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_info() {
    echo -e "${CYAN}ℹ${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    print_error "Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

print_header "Building Rust IP API"

# Check for existing target directory
if [ -d "target/release" ]; then
  print_info "Cleaning previous build artifacts..."
  cargo clean --release
fi

# Build optimized release binary
print_info "Building release binary..."
if cargo build --release; then
  print_success "Build completed successfully!"
else
  print_error "Build failed!"
  exit 1
fi

# Check if binary exists
if [ ! -f "target/release/ip-api" ]; then
  print_error "Binary not found at target/release/ip-api"
  exit 1
fi

# Get binary size
BINARY_SIZE=$(du -h target/release/ip-api | cut -f1)
BINARY_SIZE_BYTES=$(stat -f%z target/release/ip-api 2>/dev/null || stat -c%s target/release/ip-api 2>/dev/null)

echo ""
print_header "Build Information"
echo -e "${CYAN}Binary:${NC} target/release/ip-api"
echo -e "${CYAN}Size:${NC} ${BINARY_SIZE} (${BINARY_SIZE_BYTES} bytes)"
echo ""

# Show optimization details
print_info "Optimization settings:"
echo "  • Size optimization: z"
echo "  • LTO: enabled"
echo "  • Strip: enabled"
echo "  • Panic: abort"
echo ""

# Deployment instructions
print_header "Deployment Instructions"
echo ""
echo -e "${YELLOW}1. Copy binary to system:${NC}"
echo "   sudo cp target/release/ip-api /usr/local/bin/"
echo ""
echo -e "${YELLOW}2. Set up systemd services:${NC}"
echo "   sudo cp ipv4-api.service /etc/systemd/system/"
echo "   sudo cp ipv6-api.service /etc/systemd/system/"
echo "   sudo systemctl daemon-reload"
echo ""
echo -e "${YELLOW}3. Enable and start services:${NC}"
echo "   sudo systemctl enable ipv4-api ipv6-api"
echo "   sudo systemctl start ipv4-api ipv6-api"
echo ""
echo -e "${YELLOW}4. Check status:${NC}"
echo "   sudo systemctl status ipv4-api"
echo "   sudo systemctl status ipv6-api"
echo ""

# Check if systemd service files exist
if [ ! -f "ipv4-api.service" ] || [ ! -f "ipv6-api.service" ]; then
    print_warning "Systemd service files not found in current directory"
    print_info "Create ipv4-api.service and ipv6-api.service before deploying"
fi

print_success "All done!"