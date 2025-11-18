# Security Policy

## Supported Versions

We release patches for security vulnerabilities for the following versions:

| Version | Supported          |
|---------|--------------------|
| 2.0.x   | :white_check_mark: |
| 1.0.x   | :x:                |

## Security Features

The IP API includes several security features by default:

### Built-in Protection

- **Rate Limiting** - 60 requests per minute per IP address (configurable)
- **Input Validation** - IP addresses and user agents are validated and sanitized
- **Security Headers** - All responses include security headers (CSP, X-Frame-Options, etc.)
- **Request Timeouts** - 30-second timeout prevents slowloris attacks
- **Non-root Execution** - Designed to run as unprivileged user
- **Minimal Dependencies** - Small attack surface with few external dependencies

### Configuration Security

- **Environment Variables** - Sensitive config via environment, not files
- **No Default Credentials** - No authentication system (use reverse proxy)
- **Resource Limits** - Configurable rate limits and timeouts
- **DNS Cache Poisoning Protection** - Short TTL (5 minutes) on DNS cache

### Deployment Security

- **Systemd Hardening** - Service files include security restrictions
- **Docker Security** - Runs as non-root user (UID 1000)
- **Reverse Proxy** - Designed to run behind SSL/TLS termination
- **Read-only Filesystem** - Can run with read-only root filesystem

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

### How to Report

If you discover a security vulnerability, please use one of these methods:

1. **GitHub Security Advisories** (Preferred)  
   Use GitHub's [private security advisory feature](https://github.com/xFanexx/ip-api/security/advisories/new) to
   report the vulnerability privately.

2. **Direct Contact**  
   [Email the maintainers directly](mailto:security@keirahopkins.co.uk) or create a private security advisory.

### What to Include

Please include as much of the following information as possible:

- **Type of vulnerability** (e.g., DoS, XSS, injection, etc.)
- **Full paths of source files** related to the vulnerability
- **Location of affected code** (tag/branch/commit or direct URL)
- **Step-by-step instructions** to reproduce the issue
- **Proof-of-concept or exploit code** (if possible)
- **Impact assessment** - what an attacker could achieve
- **Potential fixes** - if you have suggestions

### Response Timeline

- **Initial Response**: Within 48 hours
- **Vulnerability Assessment**: Within 7 days
- **Fix Development**: Depends on severity and complexity
- **Security Advisory**: Published with fix or mitigation

### Disclosure Policy

- We follow **coordinated disclosure** practices
- We will work with you to understand and address the issue
- We ask that you give us reasonable time to fix the issue before public disclosure
- We will credit you in the security advisory (unless you prefer to remain anonymous)

### Severity Levels

We use the following severity levels:

- **Critical** - Remote code execution, authentication bypass
- **High** - Privilege escalation, significant data exposure
- **Medium** - DoS attacks, less severe data exposure
- **Low** - Minor information disclosure, low-impact issues

## Security Best Practices

### For Deployment

1. **Always run behind a reverse proxy** with SSL/TLS termination
2. **Use a firewall** to restrict access to necessary ports only
3. **Keep the system updated** - update both the OS and application regularly
4. **Run as non-root user** - use the provided systemd service files
5. **Monitor logs** - watch for unusual patterns or attacks
6. **Set resource limits** - prevent resource exhaustion attacks
7. **Use strong rate limits** - adjust `RATE_LIMIT_REQUESTS` based on your needs
8. **Enable security headers** in your reverse proxy (already included in the app)

### For Reverse Proxy Configuration

```nginx
# Example NGINX security configuration
add_header Strict-Transport-Security "max-age=31536000; includeSubDomains; preload" always;
add_header X-Frame-Options "DENY" always;
add_header X-Content-Type-Options "nosniff" always;
add_header X-XSS-Protection "1; mode=block" always;
add_header Referrer-Policy "strict-origin-when-cross-origin" always;

# Rate limiting
limit_req_zone $binary_remote_addr zone=api:10m rate=60r/m;
limit_req zone=api burst=10 nodelay;

# Request size limits
client_max_body_size 1k;
client_body_buffer_size 1k;
```

### For Docker Deployment

```yaml
# Example docker-compose.yml security settings
services:
  ip-api:
    # ... other config ...
    security_opt:
      - no-new-privileges:true
    cap_drop:
      - ALL
    read_only: true
    tmpfs:
      - /tmp
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 256M
```

## Known Issues

We maintain a list of known security issues in
our [Security Advisories](https://github.com/xFanexx/ip-api/security/advisories).

Currently, there are no known security vulnerabilities in version 2.0.0.

## Security Updates

Security updates are released as patch versions (e.g., 2.0.1, 2.0.2) and announced via:

1. [GitHub Security Advisories](https://github.com/xFanexx/ip-api/security/advisories)
2. [GitHub Releases](https://github.com/xFanexx/ip-api/releases)
3. [CHANGELOG.md](CHANGELOG.md)

## Compliance

This project follows security best practices including:

- **OWASP Top 10** - Protection against common web vulnerabilities
- **CWE/SANS Top 25** - Mitigation of most dangerous software errors
- **Rust Security Guidelines** - Following Rust secure coding practices

## Security Audits

We welcome security audits and reviews. If you're interested in conducting a security audit, please contact us.

Previous audits:

- None yet - this is the first major release

## License

The security policy is part of the IP API project and is licensed under EUPL-1.2.

## Contact

For security concerns, please use:

- **Private disclosure**: GitHub Security Advisories (preferred method)
- **General questions**: [GitHub Discussions](https://github.com/xFanexx/ip-api/discussions)

---

*Last updated: 2025-11-18*