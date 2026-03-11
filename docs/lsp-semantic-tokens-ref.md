# LSP Semantic Token Types

The LSP spec (3.16+) defines a standard set of semantic token types and modifiers. These are the closest thing to a universal, editor-agnostic vocabulary for syntax highlighting semantics.

## The 22 Token Types

`namespace`, `type`, `class`, `enum`, `interface`, `struct`, `typeParameter`, `parameter`, `variable`, `property`, `enumMember`, `event`, `function`, `method`, `macro`, `keyword`, `modifier`, `comment`, `string`, `number`, `regexp`, `operator`

## Standard Modifiers

`declaration`, `definition`, `readonly`, `static`, `deprecated`, `abstract`, `async`, `modification`, `documentation`, `defaultLibrary`

Types and modifiers compose — a token has exactly one type and zero or more modifiers.

## Where to Find This

- **LSP 3.17 Spec** (search for `SemanticTokenTypes` and `SemanticTokenModifiers`):
  https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/

- **`lsp-types` Rust crate** (defines `SemanticTokenType` and `SemanticTokenModifier` as constants):
  https://docs.rs/lsp-types/latest/lsp_types/struct.SemanticTokenType.html

- **VS Code semantic highlighting guide** (good overview of how tokens map to TextMate scopes):
  https://code.visualstudio.com/api/language-extensions/semantic-highlight-guide

- **gopls semantic tokens doc** (practical example of which tokens a real server emits vs ignores):
  https://go.googlesource.com/tools/+/refs/tags/v0.3.0/gopls/doc/semantictokens.md

## Notes

- Individual language servers can define custom token types beyond the standard 22. rust-analyzer, for example, adds things like `builtinType`, `lifetime`, `selfKeyword`, etc.
- Not every server emits every type. gopls skips `class`, `enum`, `interface`, `struct`, `typeParameter`, `property`, `enumMember`, `event`, `macro`, `modifier`, `regexp` since they don't apply to Go.
- The token encoding is positional (5 integers per token: delta line, delta col, length, type index, modifier bitmask). The type/modifier indices reference the legend sent during initialization.
