# Events

- Events are immutable records with type, payload, and origin.
- Ordering is deterministic per tab and per input source.
- Event handling is single-threaded per tab context.
- Event metadata includes causal chain IDs for traceability.
