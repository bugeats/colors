# colors

OKLCH color palette generator. Outputs hex and RGB via `--json` or a terminal preview table.

## Structure

- `src/oklch.rs` — `Oklch` type: color representation, modifiers (`dim`, `faint`, `veryfaint`, `fgdim`), interpolation, sRGB conversion
- `src/main.rs` — palette definitions, JSON/table output, CLI entry point
- `flake.nix` — builds the binary and a `json` derivation that captures `--json` output

## Build

`/nix-build` (default) builds the binary. `/nix-build json` produces `colors.json`.

## Current Focus

Oklch type extracted to its own module with modifiers as methods. Palette definitions and output remain in `main.rs`. Next natural decomposition: extract palette and/or output into their own modules when they grow.
