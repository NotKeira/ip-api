# Self-Hostable IP API

[![License: EUPL-1.2](https://img.shields.io/badge/License-EUPL--1.2-blue.svg)](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)

A lightweight Rust-based API that returns your IP address, reverse DNS, user agent, and timestamp information. Built for self-hosting with proper IPv4 and IPv6 support.

## Features

- Fast and efficient - written in Rust with Axum
- Separate endpoints for IPv4 and IPv6
- Automatic reverse DNS lookups
- Multiple timestamp formats (Unix, UTC, local time)
- Small binary size (~1-2 MB)
- No runtime dependencies

## Response Example

```json
{
  "IP": "9.9.9.9",
  "Local-Time": "2025-11-18 12:12:03",
  "UTC-Time": "2025-11-18 10:12:03 UTC",
  "Unix-Timestamp": 1760782323,
  "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:144.0) Gecko/20100101 Firefox/144.0",
  "rDNS": "dns.quad9.net"
}
```

## Installation

### Requirements

- Rust 1.85.0 or later
- Cargo

### Building

Clone and build:

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

The compiled binary will be at `target/release/ip-api`.

## Usage

Run the service with a specified port:

```bash
# IPv4 endpoint
./target/release/ip-api --port 7111

# IPv6 endpoint
./target/release/ip-api --port 7112
```

Port 7112 binds to `[::]` for IPv6, while other ports bind to `0.0.0.0` for IPv4.

### Testing

```bash
curl http://localhost:7111/
curl http://localhost:7112/
```

## Deployment

### Systemd Services

Create service files for both IPv4 and IPv6 instances.

**IPv4:** `/etc/systemd/system/ipv4-api.service`
```ini
[Unit]
Description=IP API Service (IPv4)
After=network.target

[Service]
Type=simple
User=www-data
ExecStart=/usr/local/bin/ip-api --port 7111
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

**IPv6:** `/etc/systemd/system/ipv6-api.service`
```ini
[Unit]
Description=IP API Service (IPv6)
After=network.target

[Service]
Type=simple
User=www-data
ExecStart=/usr/local/bin/ip-api --port 7112
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable ipv4-api ipv6-api
sudo systemctl start ipv4-api ipv6-api
```

### Reverse Proxy

For production use, put this behind a reverse proxy with SSL. Check out [conf/apache-vhost.md](conf/apache-vhost.md) for Apache configuration examples.

## API Documentation

### Endpoint

`GET /`

Returns JSON with client information.

### Response Fields

| Field | Type | Description |
|-------|------|-------------|
| `IP` | string | Client IP address |
| `rDNS` | string/null | Reverse DNS hostname |
| `User-Agent` | string/null | User agent string |
| `Unix-Timestamp` | integer | Unix timestamp (seconds) |
| `UTC-Time` | string | UTC time (`YYYY-MM-DD HH:MM:SS UTC`) |
| `Local-Time` | string | Server local time |

### Headers

When behind a reverse proxy:
- `X-Forwarded-For` - Real client IP
- `User-Agent` - Client user agent

## Configuration

This service is meant to run behind a reverse proxy that sets `X-Forwarded-For`. Without a proxy, it uses the direct connection IP.

Port 7112 binds to IPv6 (`[::]`), everything else binds to IPv4 (`0.0.0.0`).

## Performance

Release builds are optimized for size and speed:
- Size optimization enabled
- Link-time optimization (LTO) enabled
- Stripped binary
- Panic = abort

Binary size is usually 1-2 MB depending on your platform.

## Development

Run in dev mode:
```bash
cargo run -- --port 7111
```

Format code:
```bash
cargo fmt
```

Lint:
```bash
cargo clippy
```

Run tests:
```bash
cargo test
```

## Contributing

Contributions welcome! Make sure your code:
- Follows Rust conventions
- Includes docs where appropriate
- Passes `cargo fmt` and `cargo clippy`
- Uses American English

## License

Licensed under EUPL-1.2 (European Union Public License v1.2).

- [LICENSE](LICENSE) - English
- [LICENSE-de](LICENSE-de) - German

More info: https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12

## Authors

- xFanexx
- NotKeira

## Questions?

Open an issue on [GitHub](https://github.com/xFanexx/ip-api).