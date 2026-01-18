# Memory Model

- Heap is per-tab and isolated.
- Stack frames are explicit and typed.
- GC roots include stack frames and global module state.
- No shared heap between tabs.
