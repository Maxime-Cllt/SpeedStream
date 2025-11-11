#!/bin/bash

# Exit on any error
set -e

source ../.env

# Set variables for versioning
VERSION="1.0"

echo "Log in to Docker Hub..."
docker login

echo "Building Docker image..."
docker build -t $IMAGE_NAME:$VERSION ../

echo "Tagging Docker image..."
docker tag $IMAGE_NAME:$VERSION $DOCKER_LOGIN_USERNAME/$IMAGE_NAME:$VERSION

echo "Pushing Docker image to Docker Hub..."
docker push $DOCKER_LOGIN_USERNAME/$IMAGE_NAME:$VERSION