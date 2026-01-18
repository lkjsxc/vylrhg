# Docker

- Use docker compose for build and run.
- Command: docker compose up --build
- Data directory: bind mount ./data to /data (read/write).
- Default environment: DATA_DIR=/data
- Logs: /data/boot.log
