# vylrhg

Minimal vim-like TUI text editor (MVP).

## Features (MVP)

- Open a file from CLI: `vylrhg path/to/file`
- Modes: Normal / Insert / Command-line (`:`)
- Save / quit commands: `:w`, `:w <path>`, `:q`, `:q!`, `:wq`, `:e <path>`, `:e! <path>`
- Basic cursor movement: `h j k l`, `0`, `$`, `gg`, `G` (arrows also work)

## Run

Local:

```bash
cargo run -- path/to/file
```

Docker Compose (interactive TUI):

```bash
docker compose run --rm vylrhg /work/path/to/file
```

Build release:

```bash
cargo build --release
./target/release/vylrhg path/to/file
```

## Notes

- Text files are treated as UTF-8. Non-UTF-8 files will show an error.
- Newlines are normalized to `\n` in memory.
