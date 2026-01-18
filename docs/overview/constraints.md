# Constraints

- All user-facing logic runs inside the VM; no implicit side channels.
- Renderer and layout are pure over immutable state snapshots.
- Only explicit, typed effects may cause IO, file, or UI operations.
- Assembly compilation must be deterministic and reproducible.
- Resource usage must be bounded per tab by configurable budgets.
