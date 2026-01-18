# Syntax

## Structure

- Assembly is line-oriented with explicit block regions.
- No attributes; metadata is encoded as typed headers.

## Control Flow

- All labels are typed; jumps require type-compatible labels.
- Control flow uses explicit block delimiters.

## Macros

- Macros are hygienic and expanded before type checking.
- Macro expansion is deterministic and stable.
