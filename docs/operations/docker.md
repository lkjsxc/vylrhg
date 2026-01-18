# Docker

## Usage

- Use docker compose for build and run.
- Command: docker compose up --build

## Data

- Bind mount ./data to /data (read/write).
- Default environment: DATA_DIR=/data

## Logging

- Logs: /data/boot.log
- Logs are rotated by size on startup.
