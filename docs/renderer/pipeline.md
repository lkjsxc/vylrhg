# Pipeline

## Stages

- Parse markup -> build node tree -> resolve modules.
- Build render tree from markup + state.
- Compute layout -> build render graph -> emit render ops.

## Properties

- Render ops are deterministic and replayable.
- Render graph is immutable per frame.
- Layout outputs are cached across frames when unchanged.
