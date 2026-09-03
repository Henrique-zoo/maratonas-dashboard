#!/bin/sh
set -eu

certificate_directory=/etc/nginx/certs
certificate_path="$certificate_directory/tls.crt"
private_key_path="$certificate_directory/tls.key"

if [ -e "$certificate_path" ] || [ -e "$private_key_path" ]; then
    if [ ! -s "$certificate_path" ] || [ ! -s "$private_key_path" ]; then
        echo "The TLS volume contains an incomplete certificate and private-key pair." >&2
        exit 1
    fi

    exit 0
fi

mkdir -p "$certificate_directory"
temporary_directory="$(mktemp -d)"
trap 'rm -rf "$temporary_directory"' EXIT HUP INT TERM

umask 077
openssl req \
    -x509 \
    -nodes \
    -newkey rsa:2048 \
    -sha256 \
    -days 365 \
    -subj "/CN=localhost" \
    -addext "subjectAltName=DNS:localhost,IP:127.0.0.1" \
    -keyout "$temporary_directory/tls.key" \
    -out "$temporary_directory/tls.crt"

mv "$temporary_directory/tls.key" "$private_key_path"
mv "$temporary_directory/tls.crt" "$certificate_path"
chmod 600 "$private_key_path"
chmod 644 "$certificate_path"

echo "Generated a self-signed TLS certificate for local HTTPS."
