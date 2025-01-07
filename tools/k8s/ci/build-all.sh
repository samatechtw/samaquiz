#!/bin/sh

# Print each command and exit on error
set -e

echo "[INFO] Bulding local docker images..."

# Include env set in pr-commit.yaml
docker build -t samaquiz-api.prod -f backend/samaquiz-api/Dockerfile --build-arg="S3_SECRET_ACCESS_KEY=dev" --build-arg="SENDGRID_API_KEY=dev" --target=prod .
docker build -t api-test-helper.prod -f backend/api-test-helper/Dockerfile --target=prod .
docker build -t db-app.prod -f backend/db-app/Dockerfile --target=prod .
