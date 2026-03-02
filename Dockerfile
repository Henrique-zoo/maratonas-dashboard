# ── Estágio 1: build ──────────────────────────────────────────────────────────
FROM rust:1.91 AS builder

WORKDIR /app

# Instala dependências de sistema necessárias para compilar (ex: libpq para postgres)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Truque para cachear dependências: copia só o Cargo.toml e Cargo.lock primeiro
# e compila um main.rs dummy, aproveitando o cache de layers do Docker
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/md_backend*

# Agora copia o código real e compila de verdade
# Apenas o código do projeto será compilado, pois o Cargo verificará os timestamps dos artefatos e perceberá que as dependências já estão compiladas e atualizadas
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

# ── Estágio 2: runtime ────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Instala só as libs de runtime (não o compilador)
RUN apt-get update && apt-get install -y \
    libssl3 \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copia apenas o binário compilado
COPY --from=builder /app/target/release/md_backend .

EXPOSE 8000

CMD ["./md_backend"]