# Async Model (tokio)

## Task Domains

- UI input, layout, renderer, VM, IO, persistence.
- One executor per domain with explicit queueing rules.

## Communication

- Bounded channels per domain to preserve determinism.
- Message envelopes include causal chain IDs.
- Backpressure handled by domain-specific drop policies.

## Cancellation

- All long-running operations are cancellable.
- Cancellation propagates via scoped tokens.

## Main Loop

- No blocking on the main event loop.
- `tokio::select!` patterns enforce priority tiers.
