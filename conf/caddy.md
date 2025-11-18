# Caddy Configuration

Caddy is a modern web server with automatic HTTPS. It's simple to configure and handles SSL certificates automatically
via Let's Encrypt.

## Prerequisites

- Caddy 2.x installed
- DNS records pointing to your server
- Ports 80 and 443 open

## Basic Configuration

Create or edit your Caddyfile (usually `/etc/caddy/Caddyfile`):

### IPv4 Configuration

```caddy
ipv4.example.com {
    # Automatic HTTPS with Let's Encrypt
    
    # Reverse proxy to IPv4 service
    reverse_proxy localhost:7111 {
        # Pass client IP
        header_up X-Forwarded-For {remote_host}
    }
    
    # Security headers
    header {
        # Prevent clickjacking
        X-Frame-Options "DENY"
        
        # Prevent MIME type sniffing
        X-Content-Type-Options "nosniff"
        
        # XSS protection
        X-XSS-Protection "1; mode=block"
        
        # Referrer policy
        Referrer-Policy "strict-origin-when-cross-origin"
        
        # HSTS
        Strict-Transport-Security "max-age=31536000; includeSubDomains; preload"
        
        # CSP
        Content-Security-Policy "default-src 'none'; frame-ancestors 'none'"
    }
    
    # Logging
    log {
        output file /var/log/caddy/ipv4-api.log
        format json
    }
}
```

### IPv6 Configuration

```caddy
ipv6.example.com {
    # Automatic HTTPS with Let's Encrypt
    
    # Reverse proxy to IPv6 service
    reverse_proxy localhost:7112 {
        # Pass client IP
        header_up X-Forwarded-For {remote_host}
    }
    
    # Security headers
    header {
        X-Frame-Options "DENY"
        X-Content-Type-Options "nosniff"
        X-XSS-Protection "1; mode=block"
        Referrer-Policy "strict-origin-when-cross-origin"
        Strict-Transport-Security "max-age=31536000; includeSubDomains; preload"
        Content-Security-Policy "default-src 'none'; frame-ancestors 'none'"
    }
    
    # Logging
    log {
        output file /var/log/caddy/ipv6-api.log
        format json
    }
}
```

## Combined Configuration

If you prefer a single Caddyfile for both:

```caddy
# IPv4 endpoint
ipv4.example.com {
    reverse_proxy localhost:7111 {
        header_up X-Forwarded-For {remote_host}
    }
    
    header {
        X-Frame-Options "DENY"
        X-Content-Type-Options "nosniff"
        X-XSS-Protection "1; mode=block"
        Referrer-Policy "strict-origin-when-cross-origin"
        Strict-Transport-Security "max-age=31536000; includeSubDomains; preload"
        Content-Security-Policy "default-src 'none'; frame-ancestors 'none'"
    }
    
    log {
        output file /var/log/caddy/ipv4-api.log
        format json
    }
}

# IPv6 endpoint
ipv6.example.com {
    reverse_proxy localhost:7112 {
        header_up X-Forwarded-For {remote_host}
    }
    
    header {
        X-Frame-Options "DENY"
        X-Content-Type-Options "nosniff"
        X-XSS-Protection "1; mode=block"
        Referrer-Policy "strict-origin-when-cross-origin"
        Strict-Transport-Security "max-age=31536000; includeSubDomains; preload"
        Content-Security-Policy "default-src 'none'; frame-ancestors 'none'"
    }
    
    log {
        output file /var/log/caddy/ipv6-api.log
        format json
    }
}
```

## Advanced Configuration

### With Rate Limiting

Caddy has built-in rate limiting through the `rate_limit` directive:

```caddy
ipv4.example.com {
    # Rate limit: 60 requests per minute
    rate_limit {
        zone ip_api {
            key {remote_host}
            events 60
            window 1m
        }
    }
    
    reverse_proxy localhost:7111 {
        header_up X-Forwarded-For {remote_host}
    }
    
    header {
        X-Frame-Options "DENY"
        X-Content-Type-Options "nosniff"
        X-XSS-Protection "1; mode=block"
        Referrer-Policy "strict-origin-when-cross-origin"
        Strict-Transport-Security "max-age=31536000; includeSubDomains; preload"
        Content-Security-Policy "default-src 'none'; frame-ancestors 'none'"
    }
}
```

### With CORS

If you need to allow cross-origin requests:

```caddy
ipv4.example.com {
    reverse_proxy localhost:7111 {
        header_up X-Forwarded-For {remote_host}
    }
    
    header {
        # CORS headers
        Access-Control-Allow-Origin "https://yourdomain.com"
        Access-Control-Allow-Methods "GET, OPTIONS"
        Access-Control-Allow-Headers "User-Agent, X-Requested-With"
        
        # Security headers
        X-Frame-Options "DENY"
        X-Content-Type-Options "nosniff"
        X-XSS-Protection "1; mode=block"
        Referrer-Policy "strict-origin-when-cross-origin"
        Strict-Transport-Security "max-age=31536000; includeSubDomains; preload"
        Content-Security-Policy "default-src 'none'; frame-ancestors 'none'"
    }
}
```

## Applying Configuration

After editing your Caddyfile:

```bash
# Validate configuration
caddy validate --config /etc/caddy/Caddyfile

# Reload Caddy (without downtime)
sudo systemctl reload caddy

# Or restart if needed
sudo systemctl restart caddy

# Check status
sudo systemctl status caddy

# View logs
sudo journalctl -u caddy -f
```

## Testing

```bash
# Test IPv4 endpoint
curl https://ipv4.example.com/

# Test IPv6 endpoint
curl https://ipv6.example.com/

# Test with verbose output
curl -v https://ipv4.example.com/
```

## Troubleshooting

### Check Caddy logs

```bash
sudo journalctl -u caddy --no-pager | tail -50
```

### Check API service logs

```bash
sudo journalctl -u ipv4-api --no-pager | tail -50
sudo journalctl -u ipv6-api --no-pager | tail -50
```

### Verify ports are listening

```bash
sudo ss -tulpn | grep -E ':(7111|7112)'
```

### Test local connection

```bash
curl http://localhost:7111/
curl http://localhost:7112/
```

## Benefits of Using Caddy

- **Automatic HTTPS**: Let's Encrypt certificates are obtained and renewed automatically
- **Simple configuration**: Clean, easy-to-read syntax
- **Built-in security**: Sensible defaults out of the box
- **HTTP/2 and HTTP/3**: Modern protocol support by default
- **Zero downtime reloads**: Configuration changes without service interruption