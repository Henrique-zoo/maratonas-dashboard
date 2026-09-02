FROM oven/bun:1.3.14-alpine AS builder

WORKDIR /app/frontend

COPY frontend/package.json frontend/bun.lock ./
RUN bun install --frozen-lockfile

COPY frontend/app ./app
COPY frontend/vite.config.js ./
RUN bun run build

FROM nginx:1.30.4-alpine

RUN apk add --no-cache openssl

COPY infra/ngnix.conf /etc/nginx/conf.d/default.conf
COPY --chmod=755 infra/generate-local-certificate.sh /docker-entrypoint.d/40-generate-local-certificate.sh
COPY --from=builder /app/frontend/dist /usr/share/nginx/html

EXPOSE 80 443
