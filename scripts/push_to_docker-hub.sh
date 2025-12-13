#!/bin/bash

# Exit on any error
set -e

source ../.env


# Read cargo.toml to get the version
VERSION=$(grep '^version =' ../Cargo.toml | head -1 | awk -F'"' '{print $2}')

echo "Preparing to push Docker image..."
echo "Image Name: $IMAGE_NAME"
echo "Version: $VERSION"

echo "Log in to Docker Hub..."
docker login

echo "Building Docker image..."
docker build --no-cache -t $IMAGE_NAME:$VERSION ../

echo "Tagging Docker image..."
docker tag $IMAGE_NAME:$VERSION $DOCKER_LOGIN_USERNAME/$IMAGE_NAME:$VERSION

echo "Pushing Docker image to Docker Hub..."
docker push $DOCKER_LOGIN_USERNAME/$IMAGE_NAME:$VERSION