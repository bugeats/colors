# colors

OKLCH color palette generator. Outputs hex and RGB via `--json` or a terminal preview table.

## Architecture

Three modules with hard boundaries:

- **Chord** (`src/chord.rs`) — `{point: Color, interval: Color}` where `Color = Vector3<f64>` in `[l, c, h]` unit space (0–1). Derives `Default` (zero vector) and `PartialEq`; `is_default()` tests identity. Point is the centroid; interval is a spread vector. `From<Color>` initializes with default interval `[0.5, 0, 0]`. Three extractors: `top` (point + interval/2), `bottom` (point - interval/2), `middle` (point). Builder `set_interval` overrides the spread. Semantic modifiers shift the point (`active`, `rotate`, `desaturated`, `set_lit/sat/hue`), or collapse the interval (`faint`). Uses `nalgebra::Vector3` for all arithmetic.

- **Backends** (`src/backends.rs`) — `ThemeRgb` converts `Color` to sRGB bytes via `palette` crate. `From<Color>` for conversion, `Display` for hex output, `From<ThemeRgb> for anstyle::Color` for terminal styling.

- **Helix** (`src/helix/`) — `--helix` backend, four submodules:
  - `style.rs` — `Modifier`, `UnderlineStyle`, `Underline`, `Style` enums/structs with cascading merge and Serde serialization. `Style` fields: `color: Chord` (default = inherit), `underline: Underline`, `modifiers: &'static [Modifier]`. Serialization emits `fg`/`bg` hex via `ThemeRgb`.
  - `node.rs` — `Node { name, style, children, transform }` tree type. `transform` is `Box<dyn Fn(&Chord) -> Chord>`, defaulting to `Clone::clone`. Builder API: `node("name")`, `.child()`, `.color()`, `.transform()`, `.underline()`, `.modifiers()`. `.color()` sets an absolute Chord; `.transform()` takes `impl Fn(&Chord) -> Chord` to derive from inherited.
  - `emit.rs` — depth-first walker `emit_node` cascades inherited styles, applies node transforms, and serializes each scope as inline TOML. Cascade order: merge → transform → emit + propagate. Accumulates an ancestry chain of `(&str, Style)` pairs so inspect mode can color each path segment independently. Optional `inspect: Option<anstyle::Style>` threads a base terminal style through the walk. Optional `filter: Option<&Regex>` (`fancy-regex`) matches against the full dot-joined scope path before emitting; tree traversal continues regardless so children of non-matching nodes can still match. Helpers: `scope_path`, `fmt_inline`, `emit_scope`.
  - `theme.rs` — `theme()` returns a nameless root `Node` wrapping the full Helix scope tree. Scope inventory in `docs/helix-scopes.md`.
  - `mod.rs` — `print_helix(inspect: bool, filter: Option<&str>)` entry point. Compiles the filter regex once and passes `&Regex` through the tree walk. When `inspect`, computes root cascade into an `anstyle::Style` (bg from `bottom()`, fg from `middle()`).

- `src/main.rs` — palette definitions (two root Chords from Color, all others derived via Chord operations), output dispatch (`--json`, `--helix [--inspect] [--filter <regex>]`, default table), CLI entry point
- `flake.nix` — builds the binary and a `json` derivation that captures `--json` output

## Build

`/nix-build` (default) builds the binary. `/nix-build json` produces `colors.json`.

## Current Focus

`--filter <regex>` landed for helix output. Wiring `.color()` and `.transform()` calls in `theme()` using palette Chords from `main.rs`.
