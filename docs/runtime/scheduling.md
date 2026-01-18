# Scheduling

## Priority Tiers

- input > layout > render > VM > background.

## Fairness

- Starvation avoidance via time slices and cooperative yielding.
- Per-tab budgets enforce equal opportunity across tabs and tiles.

## Determinism

- Deterministic ordering within each domain queue.
- Scheduling decisions are reproducible under identical input.
