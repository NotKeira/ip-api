# NGINX Configuration

NGINX is a high-performance web server and reverse proxy. This guide covers setting up NGINX to proxy requests to the IP
API service.

## Prerequisites

- NGINX installed
- SSL certificates (Let's Encrypt recommended)
- DNS records configured

## Installation

### Ubuntu/Debian

```bash
sudo apt update
sudo apt install nginx certbot python3-certbot-nginx
```

### CentOS/RHEL

```bash
sudo yum install epel-release
sudo yum install nginx certbot python3-certbot-nginx
```

## SSL Certificates

Get SSL certificates using Certbot:

```bash
# For both domains
sudo certbot --nginx -d ipv4.example.com -d ipv6.example.com

# Or individually
sudo certbot --nginx -d ipv4.example.com
sudo certbot --nginx -d ipv6.example.com
```

Certbot will automatically configure NGINX and set up auto-renewal.

## Configuration

### IPv4 Configuration

Create `/etc/nginx/sites-available/ipv4-api.conf`:

```nginx
# Rate limiting zone
limit_req_zone $binary_remote_addr zone=ip_api_ipv4:10m rate=60r/m;

# HTTP - Redirect to HTTPS
server {
    listen 80;
    listen [::]:80;
    server_name ipv4.example.com;
    
    return 301 https://$server_name$request_uri;
}

# HTTPS
server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name ipv4.example.com;
    
    # SSL configuration
    ssl_certificate /etc/letsencrypt/live/ipv4.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/ipv4.example.com/privkey.pem;
    ssl_trusted_certificate /etc/letsencrypt/live/ipv4.example.com/chain.pem;
    
    # Modern SSL configuration
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers 'ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305';
    ssl_prefer_server_ciphers off;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 10m;
    ssl_session_tickets off;
    ssl_stapling on;
    ssl_stapling_verify on;
    
    # Security headers
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains; preload" always;
    add_header X-Frame-Options "DENY" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Referrer-Policy "strict-origin-when-cross-origin" always;
    add_header Content-Security-Policy "default-src 'none'; frame-ancestors 'none'" always;
    
    # Logging
    access_log /var/log/nginx/ipv4-api-access.log;
    error_log /var/log/nginx/ipv4-api-error.log;
    
    # Rate limiting
    limit_req zone=ip_api_ipv4 burst=10 nodelay;
    
    location / {
        # Proxy to backend service
        proxy_pass http://127.0.0.1:7111;
        
        # Proxy headers
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # Proxy timeouts
        proxy_connect_timeout 30s;
        proxy_send_timeout 30s;
        proxy_read_timeout 30s;
        
        # Disable buffering for real-time responses
        proxy_buffering off;
    }
}
```

### IPv6 Configuration

Create `/etc/nginx/sites-available/ipv6-api.conf`:

```nginx
# Rate limiting zone
limit_req_zone $binary_remote_addr zone=ip_api_ipv6:10m rate=60r/m;

# HTTP - Redirect to HTTPS
server {
    listen 80;
    listen [::]:80;
    server_name ipv6.example.com;
    
    return 301 https://$server_name$request_uri;
}

# HTTPS
server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name ipv6.example.com;
    
    # SSL configuration
    ssl_certificate /etc/letsencrypt/live/ipv6.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/ipv6.example.com/privkey.pem;
    ssl_trusted_certificate /etc/letsencrypt/live/ipv6.example.com/chain.pem;
    
    # Modern SSL configuration
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers 'ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305';
    ssl_prefer_server_ciphers off;
    ssl_session_cache shared:SSL:10m;
    ssl_session_timeout 10m;
    ssl_session_tickets off;
    ssl_stapling on;
    ssl_stapling_verify on;
    
    # Security headers
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains; preload" always;
    add_header X-Frame-Options "DENY" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header Referrer-Policy "strict-origin-when-cross-origin" always;
    add_header Content-Security-Policy "default-src 'none'; frame-ancestors 'none'" always;
    
    # Logging
    access_log /var/log/nginx/ipv6-api-access.log;
    error_log /var/log/nginx/ipv6-api-error.log;
    
    # Rate limiting
    limit_req zone=ip_api_ipv6 burst=10 nodelay;
    
    location / {
        # Proxy to backend service
        proxy_pass http://127.0.0.1:7112;
        
        # Proxy headers
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # Proxy timeouts
        proxy_connect_timeout 30s;
        proxy_send_timeout 30s;
        proxy_read_timeout 30s;
        
        # Disable buffering for real-time responses
        proxy_buffering off;
    }
}
```

## Enable Sites

```bash
# Create symbolic links
sudo ln -s /etc/nginx/sites-available/ipv4-api.conf /etc/nginx/sites-enabled/
sudo ln -s /etc/nginx/sites-available/ipv6-api.conf /etc/nginx/sites-enabled/

# Test configuration
sudo nginx -t

# Reload NGINX
sudo systemctl reload nginx
```

## With CORS Support

If you need CORS, add this to the `location /` block:

```nginx
location / {
    # CORS headers
    add_header Access-Control-Allow-Origin "https://yourdomain.com" always;
    add_header Access-Control-Allow-Methods "GET, OPTIONS" always;
    add_header Access-Control-Allow-Headers "User-Agent, X-Requested-With" always;
    
    # Handle preflight requests
    if ($request_method = 'OPTIONS') {
        add_header Access-Control-Allow-Origin "https://yourdomain.com";
        add_header Access-Control-Allow-Methods "GET, OPTIONS";
        add_header Access-Control-Allow-Headers "User-Agent, X-Requested-With";
        add_header Content-Length 0;
        add_header Content-Type text/plain;
        return 204;
    }
    
    proxy_pass http://127.0.0.1:7111;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
}
```

## Advanced Rate Limiting

For more granular control:

```nginx
# Define multiple rate limit zones
limit_req_zone $binary_remote_addr zone=ip_api_strict:10m rate=30r/m;
limit_req_zone $binary_remote_addr zone=ip_api_normal:10m rate=60r/m;

server {
    # ... SSL and other config ...
    
    location / {
        # Apply strict limits to specific paths if needed
        limit_req zone=ip_api_normal burst=10 nodelay;
        
        proxy_pass http://127.0.0.1:7111;
        # ... proxy config ...
    }
}
```

## Monitoring

### Check NGINX status

```bash
sudo systemctl status nginx
```

### View access logs

```bash
sudo tail -f /var/log/nginx/ipv4-api-access.log
sudo tail -f /var/log/nginx/ipv6-api-access.log
```

### View error logs

```bash
sudo tail -f /var/log/nginx/ipv4-api-error.log
sudo tail -f /var/log/nginx/ipv6-api-error.log
```

### Test configuration

```bash
sudo nginx -t
```

### Reload without downtime

```bash
sudo nginx -s reload
```

## Performance Tuning

Add to `/etc/nginx/nginx.conf` in the `http` block:

```nginx
http {
    # Connection settings
    keepalive_timeout 65;
    keepalive_requests 100;
    
    # Buffer settings
    client_body_buffer_size 10K;
    client_header_buffer_size 1k;
    client_max_body_size 8m;
    large_client_header_buffers 2 1k;
    
    # Compression
    gzip on;
    gzip_vary on;
    gzip_proxied any;
    gzip_comp_level 6;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml;
    
    # ... rest of config ...
}
```

## Troubleshooting

### Test backend services

```bash
curl http://localhost:7111/
curl http://localhost:7112/
```

### Check port listeners

```bash
sudo ss -tulpn | grep -E ':(80|443|7111|7112)'
```

### Verify SSL certificates

```bash
sudo certbot certificates
```

### Check NGINX error log

```bash
sudo tail -50 /var/log/nginx/error.log
```

## Security Best Practices

1. Keep NGINX updated
2. Use strong SSL configuration
3. Enable rate limiting
4. Monitor logs regularly
5. Use fail2ban for additional protection
6. Restrict access to sensitive paths
7. Keep SSL certificates up to date