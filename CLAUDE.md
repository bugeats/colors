# colors

OKLCH color palette generator. Outputs hex and RGB via `--json` or a terminal preview table.

## Architecture

Two layers with a hard boundary between them:

- **Chord** (`src/chord.rs`) — Pure data type: `{l, c, h}` in unit scale (0–1). Composable modifiers (`dim`, `rotate`, `interp`, etc.) operate entirely in unit space. Zero external dependencies. Knows nothing about color spaces, sRGB, or the `palette` crate.

- **Backends** (`src/backends.rs`) — Render types (`OklchHex`, `OklchRgb`) that convert Chord to concrete color representations. All `palette` crate interaction (Oklch construction, sRGB gamut clamping, byte conversion) lives here. Each backend `impl From<Chord>` and `impl Display`. `Chord::render::<T>()` dispatches via `From<Chord>`.

- `src/main.rs` — palette definitions, JSON/table output, CLI entry point
- `flake.nix` — builds the binary and a `json` derivation that captures `--json` output

## Build

`/nix-build` (default) builds the binary. `/nix-build json` produces `colors.json`.

## Current Focus

Chord/backend separation is complete. Next natural targets: extract palette definitions from `main.rs`, or add new backends (e.g. ANSI escape, CSS custom properties).
