#!/bin/bash
set -e

cd /opt/orbitask
git pull
cd backend
docker build -t orbitask .
docker stop orbitask || true
docker rm orbitask || true
docker run -d \
  --name orbitask \
  -v orbitask-data:/data \
  -p 8000:8000 \
  orbitask
