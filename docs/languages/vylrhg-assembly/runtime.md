# Runtime

## VM Instances

- Per-tab VM instance with isolated heap.
- VM instances are created on tab activation.

## Garbage Collection

- Stop-the-world, precise, generational.
- Collection is triggered by per-tab thresholds.

## Foreign Calls

- Limited to UI and renderer capabilities.
- All foreign calls are expressed as effects.

## Scheduling

- Deterministic scheduling through cooperative yields.
- Yield points are explicit in the instruction set.
