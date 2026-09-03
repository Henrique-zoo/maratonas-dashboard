# Local HTTPS and HTTP/2

The `web` service accepts HTTPS on port 443 and redirects port 80 to HTTPS. Nginx terminates TLS and negotiates HTTP/2 with clients. The internal connection from Nginx to Axum also uses HTTP/2 without TLS (h2c), within the isolated Compose network.

On its first start, the container generates a self-signed certificate for `localhost`. The `tls-certs` Compose volume preserves that certificate across container recreation. Browsers will display a trust warning because the certificate is intended only for local development.

For a public deployment, replace `/etc/nginx/certs/tls.crt` and `/etc/nginx/certs/tls.key` with a certificate and private key issued for the deployed hostname. The private key must not be committed to the repository.
