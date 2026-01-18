# Vim-like Operation

## Modes

- Modal input: normal, insert, visual, command.
- Mode transitions are explicit events.

## Keymaps

- Keymaps resolved via context layers: global > tab > tile.
- Keymaps are deterministic and immutable per frame.

## Commands

- Commands are declarative and queued into the event bus.
- Command execution is serialized per tab.
