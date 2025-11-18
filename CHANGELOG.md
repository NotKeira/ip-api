# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2025-11-18

### Added
- Health check endpoint (`/health`) for monitoring and load balancers
- Metrics endpoint (`/metrics`) for usage statistics and performance monitoring
- Headers endpoint (`/headers`) for debugging proxy configurations
- Version endpoint (`/version`) for API version information
- IP lookup endpoint (`/lookup?ip=<address>`) for arbitrary IP lookups
- Plain text response format support via `?format=text` query parameter
- DNS response caching with configurable TTL (default 5 minutes)
- Environment variable configuration for all settings
- Structured JSON logging with tracing
- Request logging middleware with timing information
- Comprehensive security enhancements:
    - Rate limiting (60 requests per minute per IP)
    - Security headers (CSP, X-Frame-Options, etc.)
    - Input validation and sanitization
    - Request timeout middleware (30 seconds)
- Unit tests for security utilities
- Modular codebase architecture with separate modules
- Caddy reverse proxy configuration documentation
- NGINX reverse proxy configuration documentation
- Systemd service files for easy deployment
- `.env.example` with configuration options
- Enhanced build script with colors and better UX

### Changed
- **BREAKING**: License changed from MIT to EUPL-1.2
- **BREAKING**: Codebase completely refactored into modules
- Updated all dependencies to latest versions
- Improved README with comprehensive documentation
- Enhanced Apache configuration examples
- Optimized Cargo.toml with release profile settings

### Fixed
- Local time format now excludes timezone suffix
- Cargo.lock now properly tracked in version control
- .idea files now properly ignored

### Security
- Added IP address validation and sanitization
- Added user agent validation (length and control character checks)
- Added security headers to all responses
- Implemented rate limiting to prevent abuse
- Added request timeouts to prevent slowloris attacks

## [1.0.0] - 2025-10-27

### Added
- Initial release
- Basic IP information endpoint
- IPv4 and IPv6 support via separate ports
- Reverse DNS lookup
- User agent detection
- Multiple timestamp formats (Unix, UTC, Local)
- Apache reverse proxy configuration example

[2.0.0]: https://github.com/xFanexx/ip-api/compare/v1.0.0...v2.0.0
[1.0.0]: https://github.com/xFanexx/ip-api/releases/tag/v1.0.0