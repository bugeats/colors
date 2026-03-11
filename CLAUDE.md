# colors

OKLCH color palette generator. Outputs hex and RGB via `--json` or a terminal preview table.

## Architecture

Two layers with hard boundaries:

- **Chord** (`src/chord.rs`) — `{point: Color, interval: Color}` where `Color = Vector3<f64>` in `[l, c, h]` unit space (0–1). Point is the centroid; interval is a spread vector. `From<Color>` initializes with default interval `[0.5, 0, 0]`. Three extractors: `top` (point + interval/2), `bottom` (point - interval/2), `middle` (point). Builder `set_interval` overrides the spread. Semantic modifiers shift the point (`dim`, `light`, `rotate`, `ansi`), collapse the interval (`faint`, `tint`), or narrow it (`soften`). Uses `nalgebra::Vector3` for all arithmetic.

- **Backends** (`src/backends.rs`) — Render types (`OklchHex`, `OklchRgb`) that convert `Color` to concrete color representations. All `palette` crate interaction lives here. Each backend `impl From<Color>` and `impl Display`.

- `src/main.rs` — palette definitions (two root Chords from Color, all others derived via Chord operations), JSON/table output, CLI entry point
- `flake.nix` — builds the binary and a `json` derivation that captures `--json` output

## Build

`/nix-build` (default) builds the binary. `/nix-build json` produces `colors.json`.

## Current Focus

Chord architecture is landed but color values are placeholder approximations — all 58 output names are wired but need eyeball tuning. Modifier constants (`dim` -0.199, `light` +0.16, `faint` +0.078, `tint` +0.046, `soften` -0.09, `ansi` -0.075) carried over from the old model and may need adjustment for the new symmetric semantics.
