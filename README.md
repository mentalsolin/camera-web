## Https Camera

## Start

1. Build/Download latest release
2. Generate certificates
3. Start the camera app
4. Available on https://ip-address:8081

## Generating certificates

```sh
openssl req -newkey rsa:2048 -nodes -keyout server.key -x509 -days 365 -out server.crt
```

## Building

### Debian

```bash
sudo apt install build-essential pkg-config libssl-dev
```
