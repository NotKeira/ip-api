## Apache Configuration

### IPv4 Virtual Host

Create a virtual host configuration:

`sudo nano /etc/apache2/sites-available/ipv4.example.com.conf`

Paste the following configuration:

```apache
# =============================================
# IPv4 API Configuration
# =============================================

# IPv4 HTTP -> HTTPS Redirect
<VirtualHost *:80>
    ServerName ipv4.example.com
    RewriteEngine On
    RewriteRule ^ https://%{HTTP_HOST}%{REQUEST_URI} [R=301,L]
</VirtualHost>

# IPv4 HTTPS Proxy
<VirtualHost *:443>
    ServerName ipv4.example.com

    # Proxy configuration
    ProxyPass / http://127.0.0.1:7111/
    ProxyPassReverse / http://127.0.0.1:7111/

    # X-Forwarded-For header to pass client IP
    RequestHeader set X-Forwarded-For expr=%{REMOTE_ADDR}

    # CORS headers for external access
    Header always set Access-Control-Allow-Origin "your-domain"
    Header always set Access-Control-Allow-Methods "GET"
    Header always set Access-Control-Allow-Headers "X-Requested-With"

    # Security Headers
    Header always set Strict-Transport-Security "max-age=31536000; includeSubDomains; preload"
    Header always set X-Frame-Options "DENY"
    Header always set X-Content-Type-Options "nosniff"
    Header always set X-XSS-Protection "1; mode=block"
    Header always set Referrer-Policy "strict-origin-when-cross-origin"

    # SSL configuration
    SSLEngine on
    SSLCertificateFile /path/to/certs/fullchain.pem
    SSLCertificateKeyFile /path/to/certs//privkey.pem

    # Modern SSL configuration
    SSLProtocol all -SSLv3 -TLSv1 -TLSv1.1
    SSLCipherSuite ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384
    SSLHonorCipherOrder off
    SSLSessionTickets off
</VirtualHost>
```

### IPv6 Virtual Host

Create a similar configuration for IPv6:


`sudo nano /etc/apache2/sites-available/ipv6.example.com.conf`

Use the following configuration (note the IPv6 localhost address):

```apache
# =============================================
# IPv6 API Configuration
# =============================================

# IPv6 HTTP -> HTTPS Redirect
<VirtualHost *:80>
    ServerName ipv6.example.com
    RewriteEngine On
    RewriteRule ^ https://%{HTTP_HOST}%{REQUEST_URI} [R=301,L]
</VirtualHost>

# IPv6 HTTPS Proxy
<VirtualHost *:443>
    ServerName ipv6.example.com

    # Proxy configuration
    ProxyPass / http://127.0.0.1:7112/
    ProxyPassReverse / http://127.0.0.1:7112/

    # X-Forwarded-For header to pass client IP
    RequestHeader set X-Forwarded-For expr=%{REMOTE_ADDR}

    # CORS headers for external access
    Header always set Access-Control-Allow-Origin "your-domain"
    Header always set Access-Control-Allow-Methods "GET"
    Header always set Access-Control-Allow-Headers "X-Requested-With"

    # Security Headers
    Header always set Strict-Transport-Security "max-age=31536000; includeSubDomains; preload"
    Header always set X-Frame-Options "DENY"
    Header always set X-Content-Type-Options "nosniff"
    Header always set X-XSS-Protection "1; mode=block"
    Header always set Referrer-Policy "strict-origin-when-cross-origin"

    # SSL configuration
    SSLEngine on
    SSLCertificateFile /path/to/certs/fullchain.pem
    SSLCertificateKeyFile /path/to/certs/fullchain.pem

    # Modern SSL configuration
    SSLProtocol all -SSLv3 -TLSv1 -TLSv1.1
    SSLCipherSuite ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-CHACHA20-POLY1305:ECDHE-RSA-CHACHA20-POLY1305:DHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256-GCM-SHA384
    SSLHonorCipherOrder off
    SSLSessionTickets off
</VirtualHost>
```

### Enable Apache Modules and Sites

```bash
# Enable required Apache modules
sudo a2enmod proxy proxy_http headers ssl

# Enable sites
sudo a2ensite ipv4.example.com
sudo a2ensite ipv6.example.com

# Apply changes
sudo systemctl reload apache2
```
