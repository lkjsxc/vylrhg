# Async Model (tokio)

- Task domains: UI, layout, renderer, VM, IO.
- Communication via bounded channels to preserve determinism.
- All long-running operations are cancellable.
- No blocking on the main event loop; uses `tokio::select!` patterns.
