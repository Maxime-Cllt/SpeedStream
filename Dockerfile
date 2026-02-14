# syntax=docker/dockerfile:1

# --- Stage 1: Chef (Préparation du cache) ---
FROM rust:1.93-slim AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- Stage 2: Builder ---
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Installation des libs de build + mold (linker rapide)
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev mold clang \
    && rm -rf /var/lib/apt/lists/*
ENV RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=mold"

# Cook des dépendances avec cache persistant du registre Cargo
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo chef cook --release --recipe-path recipe.json

COPY . .

# Build du binaire avec cache persistant du target
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo build --release --bin SpeedStream \
    && cp target/release/SpeedStream /app/speedstream-bin

# On "nettoie" le binaire des symboles de debug (gain de place massif)
RUN strip /app/speedstream-bin

# --- Stage 3: Runtime (Le plus léger et sécurisé possible) ---
FROM debian:bookworm-slim AS runtime

# On crée l'utilisateur avant d'installer quoi que ce soit
RUN groupadd -r speedstream && useradd -r -g speedstream speedstream

# Installation des libs de RUNTIME uniquement
# libpq5 et ca-certificates sont essentiels pour Postgres et le SSL
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
RUN chown speedstream:speedstream /app

# Copie du binaire propre
COPY --from=builder --chown=speedstream:speedstream /app/speedstream-bin /app/speedstream

USER speedstream

# Port configurable via variable d'environnement (défaut: 8080)
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080
EXPOSE ${SERVER_PORT}

# On utilise le binaire directement
ENTRYPOINT ["./speedstream"]
