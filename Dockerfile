# --- Stage 1: Planner (Optimization du cache des dépendances) ---
FROM rust:1.93-slim AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --- Stage 2: Builder ---
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build des dépendances uniquement (mis en cache tant que Cargo.toml ne change pas)
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev \
    && cargo chef cook --release --recipe-path recipe.json

# Build de l'application
COPY . .
RUN cargo build --release --bin SpeedStream

# Optimisation du binaire (réduction de taille ~20-40%)
RUN strip target/release/SpeedStream

# --- Stage 3: Runtime (Sécurité maximale et poids plume) ---
# Utilisation de Distroless pour une surface d'attaque minimale
FROM gcr.io/distroless/cc-debian12 AS runtime

WORKDIR /app

# Copie des bibliothèques système nécessaires (libpq, etc.) depuis le builder ou via staging
# Note : libpq5 et libssl sont souvent requis dynamiquement
COPY --from=builder /usr/lib/x86_64-linux-gnu/libpq.so* /usr/lib/x86_64-linux-gnu/
COPY --from=builder /usr/lib/x86_64-linux-gnu/libssl.so* /usr/lib/x86_64-linux-gnu/
COPY --from=builder /usr/lib/x86_64-linux-gnu/libcrypto.so* /usr/lib/x86_64-linux-gnu/
COPY --from=builder /usr/lib/x86_64-linux-gnu/liblber-2.5.so* /usr/lib/x86_64-linux-gnu/
COPY --from=builder /usr/lib/x86_64-linux-gnu/libldap-2.5.so* /usr/lib/x86_64-linux-gnu/
COPY --from=builder /usr/lib/x86_64-linux-gnu/libsasl2.so* /usr/lib/x86_64-linux-gnu/
COPY --from=builder /usr/lib/x86_64-linux-gnu/libgnutls.so* /usr/lib/x86_64-linux-gnu/

# Copie du binaire avec droits restreints d'emblée
COPY --from=builder --chown=nonroot:nonroot /app/target/release/SpeedStream /app/speedstream

# Utilisation de l'utilisateur non-root par défaut de distroless (id 65532)
USER nonroot

EXPOSE 8080

# Note: Distroless n'a pas 'curl'. Pour un HEALTHCHECK, il est préférable
# d'utiliser un petit binaire dédié ou de gérer cela via l'orchestrateur (K8s/Docker).
# Si tu as vraiment besoin d'un healthcheck interne, reste sur debian-slim.

ENTRYPOINT ["./speedstream"]