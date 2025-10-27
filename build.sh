#!/bin/bash
set -e

echo "Building Rust IP API..."

# Build optimized release binary
cargo build --release

# Show binary size
ls -lh target/release/ip-api

echo ""
echo "Build complete!"
echo ""
echo "Binary: target/release/ip-api"
echo "Size: $(du -h target/release/ip-api | cut -f1)"
echo ""
echo "To deploy:"
echo "sudo cp ipv4-api.service /etc/systemd/system/"
echo "sudo cp ipv6-api.service /etc/systemd/system/"
echo "sudo systemctl daemon-reload"
echo "sudo systemctl enable ipv4-api ipv6-api"
echo "sudo systemctl restart ipv4-api ipv6-api"
echo ""
echo "Check status:"
echo "sudo systemctl status ipv4-api"
echo "sudo systemctl status ipv6-api"
