#!/bin/bash

# Génère un token API sécurisé de 64 caractères
# Utilise /dev/urandom pour la génération cryptographiquement sûre

generate_token() {
    local length=${1:-64}

    # Génère un token avec des caractères alphanumériques (a-z, A-Z, 0-9)
    # Utilise /dev/urandom qui est cryptographiquement sûr
    token=$(LC_ALL=C tr -dc 'a-zA-Z0-9' < /dev/urandom | head -c "$length")

    echo "$token"
}

# Afficher le token
TOKEN=$(generate_token 64)
echo "Token API généré (64 caractères):"
echo "$TOKEN"