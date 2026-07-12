FROM node:24-alpine AS builder

WORKDIR /app/frontend

COPY frontend/package*.json ./
RUN npm ci

COPY frontend/app ./app
COPY frontend/vite.config.js ./
RUN npm run build

FROM nginx:1.27-alpine

COPY infra/ngnix.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/frontend/dist /usr/share/nginx/html

EXPOSE 80
