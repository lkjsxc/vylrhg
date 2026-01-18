# Boundaries

## Isolation

- Each tab has a dedicated VM context and heap.
- Effects are explicitly authorized per capability set.

## Messaging

- All subsystem communication is via explicit messages.
- Messages are versioned by schema rather than implicit fields.

## Persistence

- Tab state and layout trees are the only persisted runtime artifacts.
- Render cache and transient effects are not persisted.
