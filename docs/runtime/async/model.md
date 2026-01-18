# Async Model

- Each domain has its own executor and queue.
- Domains communicate via typed message envelopes.
- Execution order is deterministic within a domain.
