# API Documentation

Complete API reference for the IP API service.

## Quick Links

- [OpenAPI Specification](../openapi.yaml)
- [Swagger UI](https://editor.swagger.io/) - Paste the openapi.yaml content
- [Redoc](https://redocly.github.io/redoc/) - Alternative API documentation viewer

## Base URLs

- **IPv4**: `https://ipv4.example.com`
- **IPv6**: `https://ipv6.example.com`
- **Local**: `http://localhost:7111` (IPv4) or `http://localhost:7112` (IPv6)

## Authentication

No authentication required. The API is rate-limited to 60 requests per minute per IP address.

## Response Formats

All endpoints support JSON by default. Some endpoints also support plain text format.

### Content Negotiation

Use the `Accept` header or `format` query parameter:

```bash
# JSON (default)
curl https://ipv4.example.com/

# Plain text via Accept header
curl -H "Accept: text/plain" https://ipv4.example.com/

# Plain text via query parameter
curl https://ipv4.example.com/?format=text
```

## Endpoints

### GET /

Get information about the client's IP address.

**Request:**

```bash
curl https://ipv4.example.com/
```

**Query Parameters:**

- `format` (optional): Response format (`json`, `text`, `plain`, `txt`)

**Response (JSON):**

```json
{
  "IP": "203.0.113.42",
  "rDNS": "example.com",
  "User-Agent": "curl/7.68.0",
  "Unix-Timestamp": 1732040095,
  "UTC-Time": "2025-11-18 17:54:55 UTC",
  "Local-Time": "2025-11-18 17:54:55"
}
```

**Response (Plain Text):**

```
IP: 203.0.113.42
rDNS: example.com
User-Agent: curl/7.68.0
Unix-Timestamp: 1732040095
UTC-Time: 2025-11-18 17:54:55 UTC
Local-Time: 2025-11-18 17:54:55
```

---

### GET /lookup

Look up information about any IP address.

**Request:**

```bash
curl "https://ipv4.example.com/lookup?ip=8.8.8.8"
```

**Query Parameters:**

- `ip` (required): IP address to look up (IPv4 or IPv6)

**Response:**

```json
{
  "IP": "8.8.8.8",
  "rDNS": "dns.google",
  "User-Agent": null,
  "Unix-Timestamp": 1732040095,
  "UTC-Time": "2025-11-18 17:54:55 UTC",
  "Local-Time": "2025-11-18 17:54:55"
}
```

**Error Responses:**

- `400 Bad Request`: Invalid IP address format

---

### GET /health

Health check endpoint for monitoring.

**Request:**

```bash
curl https://ipv4.example.com/health
```

**Response:**

```json
{
  "status": "healthy",
  "timestamp": 1732040095,
  "uptime_seconds": 86400
}
```

---

### GET /metrics

API usage statistics.

**Request:**

```bash
curl https://ipv4.example.com/metrics
```

**Response:**

```json
{
  "total_requests": 15420,
  "successful_requests": 15380,
  "failed_requests": 40,
  "uptime_seconds": 86400,
  "timestamp": 1732040095
}
```

---

### GET /headers

View all request headers (debugging).

**Request:**

```bash
curl https://ipv4.example.com/headers
```

**Response:**

```json
{
  "headers": {
    "host": "ipv4.example.com",
    "user-agent": "curl/7.68.0",
    "x-forwarded-for": "203.0.113.42",
    "x-real-ip": "203.0.113.42",
    "accept": "*/*"
  }
}
```

---

### GET /version

API version information.

**Request:**

```bash
curl https://ipv4.example.com/version
```

**Response:**

```json
{
  "version": "2.0.0",
  "name": "ip-api",
  "authors": [
    "xFanexx",
    "NotKeira"
  ],
  "repository": "https://github.com/xFanexx/ip-api",
  "rust_edition": "2024"
}
```

## Rate Limiting

- **Limit**: 60 requests per minute per IP address
- **Response**: `429 Too Many Requests`
- **Retry**: Wait for the time window to reset

The rate limit applies to all endpoints globally per IP address.

## Security Headers

All responses include security headers:

- `X-Content-Type-Options: nosniff`
- `X-Frame-Options: DENY`
- `X-XSS-Protection: 1; mode=block`
- `Referrer-Policy: strict-origin-when-cross-origin`
- `Content-Security-Policy: default-src 'none'; frame-ancestors 'none'`
- `Strict-Transport-Security: max-age=31536000; includeSubDomains` (HTTPS only)

## Error Handling

### HTTP Status Codes

- `200 OK`: Successful request
- `400 Bad Request`: Invalid input (e.g., malformed IP address)
- `408 Request Timeout`: Request took too long to process
- `429 Too Many Requests`: Rate limit exceeded
- `500 Internal Server Error`: Server error

### Error Response Format

Errors return appropriate HTTP status codes. Some endpoints may return JSON error objects:

```json
{
  "error": "Invalid IP address format"
}
```

## Examples

### Shell/cURL

```bash
# Get your IP
curl https://ipv4.example.com/

# Get your IP in plain text
curl https://ipv4.example.com/?format=text

# Look up an IP
curl "https://ipv4.example.com/lookup?ip=1.1.1.1"

# Check health
curl https://ipv4.example.com/health

# View metrics
curl https://ipv4.example.com/metrics
```

### Python

```python
import requests

# Get your IP
response = requests.get('https://ipv4.example.com/')
data = response.json()
print(f"Your IP: {data['IP']}")
print(f"rDNS: {data['rDNS']}")

# Look up an IP
response = requests.get('https://ipv4.example.com/lookup', params={'ip': '8.8.8.8'})
data = response.json()
print(f"Google DNS: {data['rDNS']}")
```

### JavaScript/Node.js

```javascript
// Get your IP
fetch('https://ipv4.example.com/')
    .then(response => response.json())
    .then(data => {
        console.log('Your IP:', data.IP);
        console.log('rDNS:', data.rDNS);
    });

// Look up an IP
fetch('https://ipv4.example.com/lookup?ip=8.8.8.8')
    .then(response => response.json())
    .then(data => console.log('Google DNS:', data.rDNS));
```

### Go

```go
package main

import (
    "encoding/json"
    "fmt"
    "net/http"
)

type IPResponse struct {
    IP            string  `json:"IP"`
    RDNS          *string `json:"rDNS"`
    UserAgent     *string `json:"User-Agent"`
    UnixTimestamp int64   `json:"Unix-Timestamp"`
    UTCTime       string  `json:"UTC-Time"`
    LocalTime     string  `json:"Local-Time"`
}

func main() {
    resp, err := http.Get("https://ipv4.example.com/")
    if err != nil {
        panic(err)
    }
    defer resp.Body.Close()

    var data IPResponse
    json.NewDecoder(resp.Body).Decode(&data)
    
    fmt.Printf("Your IP: %s\n", data.IP)
    if data.RDNS != nil {
        fmt.Printf("rDNS: %s\n", *data.RDNS)
    }
}
```

## Performance

- **DNS Caching**: Reverse DNS lookups are cached for 5 minutes (configurable)
- **Request Timeout**: 30 seconds (configurable)
- **Average Response Time**: < 50ms (without DNS lookup), < 200ms (with DNS lookup)

## Support

- **Issues**: [GitHub Issues](https://github.com/xFanexx/ip-api/issues)
- **Documentation**: [README.md](../README.md)
- **Contributing**: [CONTRIBUTING.md](../CONTRIBUTING.md)