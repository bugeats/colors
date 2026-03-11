# colors

OKLCH color palette generator. Outputs hex and RGB via `--json` or a terminal preview table.

## Architecture

Two layers with hard boundaries:

- **Chord** (`src/chord.rs`) — `{point: Color, interval: Color}` where `Color = Vector3<f64>` in `[l, c, h]` unit space (0–1). Point is the centroid; interval is a spread vector. `From<Color>` initializes with default interval `[0.5, 0, 0]`. Three extractors: `top` (point + interval/2), `bottom` (point - interval/2), `middle` (point). Builder `set_interval` overrides the spread. Semantic modifiers shift the point (`active`, `rotate`, `desaturated`, `set_lit/sat/hue`), or collapse the interval (`faint`). Uses `nalgebra::Vector3` for all arithmetic.

- **Backends** (`src/backends.rs`) — `ThemeRgb` converts `Color` to sRGB bytes via `palette` crate. `From<Color>` for conversion, `Display` for hex output, `From<ThemeRgb> for anstyle::Color` for terminal styling.

- **Helix** (`src/helix.rs`) — `--helix` backend. `Node` tree (Section/Branch/Leaf) with cascading `Style`. `Style` has const builders (`fg`, `bg`, `underline`, `modifiers`) and `merge` (child overrides parent per-field). `node_style!` macro delegates style builders to Node variants. `leaf()`, `branch()`, `section()` const fn constructors. `emit_node` depth-first walker cascades inherited styles and serializes to inline TOML via `toml::Value`. Complete scope inventory in `docs/helix-scopes.md`.

- `src/main.rs` — palette definitions (two root Chords from Color, all others derived via Chord operations), output dispatch (`--json`, `--helix`, default table), CLI entry point
- `flake.nix` — builds the binary and a `json` derivation that captures `--json` output

## Build

`/nix-build` (default) builds the binary. `/nix-build json` produces `colors.json`.

## Current Focus

Node tree and style cascade are in place. Palette emission is decoupled from the theme tree — palette generation from the tree is a follow-up. Current THEME covers a working subset of scopes; complete inventory is in `docs/helix-scopes.md`, ready for full coverage pass. Scope assignments are reasonable defaults pending eyeball tuning.

Terminal preview table uses `anstyle::Style` for typed ANSI output.
