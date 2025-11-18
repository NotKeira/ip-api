# Self-Hostable IP API

[![License: EUPL-1.2](https://img.shields.io/badge/License-EUPL--1.2-blue.svg)](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)
[![CI](https://github.com/xFanexx/ip-api/actions/workflows/ci.yml/badge.svg)](https://github.com/xFanexx/ip-api/actions/workflows/ci.yml)
[![Release](https://github.com/xFanexx/ip-api/actions/workflows/release.yml/badge.svg)](https://github.com/xFanexx/ip-api/actions/workflows/release.yml)
[![Version](https://img.shields.io/github/v/release/xFanexx/ip-api)](https://github.com/xFanexx/ip-api/releases)

A lightweight Rust-based API that returns your IP address, reverse DNS, user agent, and timestamp information. Built for
self-hosting with proper IPv4 and IPv6 support.

## Features

- **Fast and efficient** - Written in Rust with Axum framework
- **Dual stack support** - Separate endpoints for IPv4 and IPv6
- **Reverse DNS lookups** - Automatic with intelligent caching (5min TTL)
- **Multiple endpoints** - IP info, health checks, metrics, debugging tools
- **Security hardened** - Rate limiting, input validation, security headers
- **Flexible responses** - JSON or plain text output formats
- **Production ready** - Health checks, metrics, structured logging
- **Easy deployment** - Docker support, pre-built binaries, systemd services
- **Fully documented** - OpenAPI spec, comprehensive guides, code examples
- **Small footprint** - Binary size ~1-2 MB, minimal dependencies

## Quick Start

### Docker (Recommended)

```bash
# Using Docker Compose
docker-compose up -d

# Or with Docker directly
docker run -d -p 7111:7111 -e PORT=7111 ip-api:latest
```

See [Docker documentation](docs/DOCKER.md) for more details.

### Pre-built Binaries

Download pre-built binaries from the [releases page](https://github.com/xFanexx/ip-api/releases).

Available for:

- Linux (amd64, arm64)
- macOS (amd64, arm64/Apple Silicon)
- Windows (amd64)

```bash
# Linux/macOS example
wget https://github.com/xFanexx/ip-api/releases/download/v2.0.0/ip-api-linux-amd64.tar.gz
tar -xzf ip-api-linux-amd64.tar.gz
chmod +x ip-api
./ip-api --port 7111
```

### Building from Source

```bash
git clone https://github.com/xFanexx/ip-api.git
cd ip-api
cargo build --release
```

Or use the build script:

```bash
chmod +x build.sh
./build.sh
```

## API Endpoints

| Endpoint                   | Description                  | Docs                               |
|----------------------------|------------------------------|------------------------------------|
| `GET /`                    | Your IP information          | [Details](docs/API.md#get-)        |
| `GET /lookup?ip=<address>` | Look up any IP address       | [Details](docs/API.md#get-lookup)  |
| `GET /health`              | Health check for monitoring  | [Details](docs/API.md#get-health)  |
| `GET /metrics`             | Usage statistics             | [Details](docs/API.md#get-metrics) |
| `GET /headers`             | View request headers (debug) | [Details](docs/API.md#get-headers) |
| `GET /version`             | API version info             | [Details](docs/API.md#get-version) |

**Full API Documentation:**

- [API Reference Guide](docs/API.md)
- [OpenAPI 3.0 Specification](openapi.yaml)
- [Interactive Swagger UI](https://editor.swagger.io/) (paste openapi.yaml)

## Response Example

```json
{
  "IP": "9.9.9.9",
  "rDNS": "dns.quad9.net",
  "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:144.0) Gecko/20100101 Firefox/144.0",
  "Unix-Timestamp": 1732040095,
  "UTC-Time": "2025-11-18 18:01:35 UTC",
  "Local-Time": "2025-11-18 18:01:35"
}
```

**Plain Text Format:**

```bash
curl https://ipv4.example.com/?format=text

IP: 9.9.9.9
rDNS: dns.quad9.net
User-Agent: curl/7.68.0
Unix-Timestamp: 1732040095
UTC-Time: 2025-11-18 18:01:35 UTC
Local-Time: 2025-11-18 18:01:35
```

## Installation

### Requirements

- Rust 1.90.0 or later
- Cargo (comes with Rust)

### Building

```bash
git clone https://github.com/xFanexx/ip-api.git
cd ip-api
cargo build --release
```

The compiled binary will be at `target/release/ip-api`.

### Using Pre-built Binaries

1. Download from [releases page](https://github.com/xFanexx/ip-api/releases)
2. Extract the archive
3. Move binary to your PATH (optional):
   ```bash
   sudo mv ip-api /usr/local/bin/
   ```

## Usage

Run the service with a specified port:

```bash
# IPv4 endpoint
./ip-api --port 7111

# IPv6 endpoint
./ip-api --port 7112
```

**Port Binding:**

- Port 7112 binds to `[::]` for IPv6
- All other ports bind to `0.0.0.0` for IPv4

### Testing Locally

```bash
curl http://localhost:7111/
curl http://localhost:7112/
```

## Configuration

Configure via environment variables (see [`.env.example`](.env.example)):

```bash
# Rate limiting
export RATE_LIMIT_REQUESTS=60        # Max requests per window
export RATE_LIMIT_WINDOW_SECS=60     # Window duration in seconds

# DNS cache
export DNS_CACHE_TTL_SECS=300        # Cache TTL (5 minutes)

# Timeouts
export REQUEST_TIMEOUT_SECS=30       # Request timeout

# Logging
export RUST_LOG=info                 # Log level (trace, debug, info, warn, error)
export LOG_FORMAT=json               # Optional: JSON structured logs
```

All settings have sensible defaults and can be used without configuration.

## Deployment

### Systemd Services

Service files included for easy deployment:

```bash
# Copy binary and service files
sudo cp target/release/ip-api /usr/local/bin/
sudo cp ipv4-api.service ipv6-api.service /etc/systemd/system/

# Enable and start services
sudo systemctl daemon-reload
sudo systemctl enable --now ipv4-api ipv6-api

# Check status
sudo systemctl status ipv4-api ipv6-api
```

### Reverse Proxy Setup

Complete configuration guides available:

- **[Apache](conf/apache-vhost.md)** - Full SSL/TLS setup with Let's Encrypt
- **[NGINX](conf/nginx.md)** - Modern SSL config with rate limiting
- **[Caddy](conf/caddy.md)** - Automatic HTTPS with simple configuration

### Docker Deployment

See the [Docker deployment guide](docs/DOCKER.md) for:

- Docker Compose setup
- Multi-container configurations
- Resource limits and security
- Health checks and monitoring
- Integration with Traefik/NGINX

## Security Features

- **Rate Limiting** - 60 requests/minute per IP (configurable)
- **Input Validation** - IP address and user agent sanitization
- **Security Headers** - CSP, X-Frame-Options, X-XSS-Protection, etc.
- **Request Timeouts** - 30 second timeout to prevent slowloris attacks
- **No Privilege Escalation** - Runs as non-root user
- **Minimal Attack Surface** - Small binary, few dependencies

See [SECURITY.md](SECURITY.md) for security policy and reporting vulnerabilities.

## Performance

- **Response Time**: <50ms (cached DNS), <200ms (fresh DNS lookup)
- **DNS Caching**: 5 minutes TTL (configurable)
- **Request Timeout**: 30 seconds (configurable)
- **Rate Limit**: 60 req/min per IP (configurable)
- **Binary Size**: 1-2 MB (stripped, optimized)
- **Memory Usage**: ~5-10 MB typical

## Development

### Setup

```bash
# Clone and build
git clone https://github.com/xFanexx/ip-api.git
cd ip-api
cargo build

# Run in development mode
cargo run -- --port 7111
```

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Security audit
cargo audit
```

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for:

- Code of conduct
- Development setup
- Coding standards
- Commit message conventions
- Pull request process

**Quick Guidelines:**

- Follow Rust conventions
- Add tests for new features
- Update documentation
- Use conventional commits
- Run `cargo fmt` and `cargo clippy`

## Documentation

- **[README.md](README.md)** - This file (getting started)
- **[CHANGELOG.md](CHANGELOG.md)** - Version history and changes
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Contribution guidelines
- **[SECURITY.md](SECURITY.md)** - Security policy
- **[API Reference](docs/API.md)** - Complete API documentation
- **[OpenAPI Spec](openapi.yaml)** - Machine-readable API spec
- **[Docker Guide](docs/DOCKER.md)** - Docker deployment
- **[Apache Config](conf/apache-vhost.md)** - Apache setup
- **[NGINX Config](conf/nginx.md)** - NGINX setup
- **[Caddy Config](conf/caddy.md)** - Caddy setup

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for detailed version history.

## Roadmap

Planned features for future releases:

- [ ] Prometheus metrics endpoint
- [ ] GeoIP support (optional)
- [ ] ASN information
- [ ] Bulk IP lookup endpoint
- [ ] WebSocket support for real-time updates
- [ ] GraphQL API option

Suggestions? [Open an issue](https://github.com/xFanexx/ip-api/issues)!

## License

Licensed under **EUPL-1.2** (European Union Public License v1.2).

- [LICENSE](LICENSE) - English
- [LICENSE-de](LICENSE-de) - German (Deutsch)

More info: https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12

**Why EUPL?** Strong copyleft protection while maintaining compatibility with other open source licenses (GPL, MPL,
etc.). Available in all 24 official EU languages.

## Authors

- **xFanexx** - Initial work and core development
- **NotKeira** - Enhancements and documentation

See the list of [contributors](https://github.com/xFanexx/ip-api/contributors) who participated in this project.

## Acknowledgments

Built with these amazing open source projects:

- [Rust](https://www.rust-lang.org/) - Systems programming language
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Tokio](https://tokio.rs/) - Async runtime
- [Serde](https://serde.rs/) - Serialization framework

## Support

- **Issues**: [GitHub Issues](https://github.com/xFanexx/ip-api/issues)
- **Discussions**: [GitHub Discussions](https://github.com/xFanexx/ip-api/discussions)
- **Security**: See [SECURITY.md](SECURITY.md)

---

**⭐ If you find this project useful, please consider giving it a star on GitHub!**