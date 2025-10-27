# Self-Hostable IP API
A lightweight API that returns the client IP, hostname (reverse DNS), user agent, and timestamp information. Supports **IPv4/IPv6** via separate endpoints with a reverse proxy.

## Response Example
```json
{
  "IP": "9.9.9.9",
  "Local-Time": "2025-10-18 12:12:03",
  "UTC-Time": "2025-10-18 10:12:03 UTC",
  "Unix-Timestamp": 1760782323,
  "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:144.0) Gecko/20100101 Firefox/144.0",
  "rDNS": "dns.quad9.net"
}
```
