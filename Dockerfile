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
# Installation des libs de build
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev \
    && cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin SpeedStream
# On "nettoie" le binaire des symboles de debug (gain de place massif)
RUN strip target/release/SpeedStream

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

# Copie du binaire propre
COPY --from=builder --chown=speedstream:speedstream /app/target/release/SpeedStream /app/speedstream

USER speedstream

# L'astuce pour le port : Axum écoute souvent sur 0.0.0.0:8080
EXPOSE 8080

# On utilise le binaire directement
ENTRYPOINT ["./speedstream"]