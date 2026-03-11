# colors

OKLCH color palette generator. Outputs hex and RGB via `--json` or a terminal preview table.

## Architecture

Three modules with hard boundaries:

- **Chord** (`src/chord.rs`) — `{point: Color, interval: Color}` where `Color = Vector3<f64>` in `[l, c, h]` unit space (0–1). Derives `Default` (zero vector) and `PartialEq`; `is_default()` tests identity. Point is the centroid; interval is a spread vector. `From<Color>` initializes with default interval `[0.5, 0, 0]`. Three extractors: `top` (point + interval/2), `bottom` (point - interval/2), `middle` (point). Builder `set_interval` overrides the spread. Semantic modifiers shift the point (`active`, `rotate`, `desaturated`, `set_lit/sat/hue`), or collapse the interval (`faint`). Uses `nalgebra::Vector3` for all arithmetic.

- **Backends** (`src/backends.rs`) — `ThemeRgb` converts `Color` to sRGB bytes via `palette` crate. `From<Color>` for conversion, `Display` for hex output, `From<ThemeRgb> for anstyle::Color` for terminal styling.

- **Helix** (`src/helix.rs`) — `--helix` backend. `Node` carries its own `name: &'static str` and `style: Style`. Builder API: `node("name")` (free fn), `.child(node)`, `.color(chord)`, `.underline(chord, style)`, `.modifiers()`. `Style` has `color: Chord` (not optional — default means "inherit"), `underline: Underline`, `modifiers: &'static [Modifier]`. Cascading `Style::merge` (child overrides parent per-field). `Underline` enum (`None`/`Styled { color: Chord, style }`), `Modifier` and `UnderlineStyle` enums cover the full Helix spec. Serialization always emits `fg` (from `middle()`) and `bg` (from `bottom()`) via `ThemeRgb` hex; every scope gets a line. `theme()` returns a single nameless root `Node` that wraps the tree — its style cascades to all children but produces no output line. `emit_node` depth-first walker cascades inherited styles and serializes to inline TOML. Scope inventory in `docs/helix-scopes.md`.

- `src/main.rs` — palette definitions (two root Chords from Color, all others derived via Chord operations), output dispatch (`--json`, `--helix`, default table), CLI entry point
- `flake.nix` — builds the binary and a `json` derivation that captures `--json` output

## Build

`/nix-build` (default) builds the binary. `/nix-build json` produces `colors.json`.

## Current Focus

Style uses `Chord` for all color data; string placeholders are gone. Every scope emits `fg` + `bg` hex (currently `#000000` from default Chord). Connecting palette Chords to the theme tree is the next step — wire `.color(chord)` calls in `theme()` using Chords defined in `main.rs`.

Terminal preview table uses `anstyle::Style` for typed ANSI output.
