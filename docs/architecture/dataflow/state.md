# State

- State is versioned by monotonically increasing revision IDs.
- State diffs are typed patches, not arbitrary mutations.
- Renderer consumes the latest committed state snapshot.
- Persistence stores only committed state snapshots.
