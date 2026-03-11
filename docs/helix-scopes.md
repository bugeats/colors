# Helix Theme Scopes (Complete)

Derived from `helix-view/src/theme.rs`, `book/src/themes.md`, and all
`theme.get()` / `find_highlight_exact()` call sites in the Helix source (main branch).

The theme system uses `HashMap<String, Style>` — any key is valid. The `try_get`
method implements dot-separated fallback: `ui.text.focus` falls back to `ui.text`,
then `ui`. Parent scopes act as defaults for their children.

## Syntax

Tree-sitter scopes. Longest matching key wins.

- `attribute`
- `type`
  - `type.builtin`
  - `type.parameter`
  - `type.enum`
    - `type.enum.variant`
- `constructor`
- `constant`
  - `constant.builtin`
    - `constant.builtin.boolean`
  - `constant.character`
    - `constant.character.escape`
  - `constant.numeric`
    - `constant.numeric.integer`
    - `constant.numeric.float`
- `string`
  - `string.regexp`
  - `string.special`
    - `string.special.path`
    - `string.special.url`
    - `string.special.symbol`
- `comment`
  - `comment.line`
    - `comment.line.documentation`
  - `comment.block`
    - `comment.block.documentation`
  - `comment.unused`
- `variable`
  - `variable.builtin`
  - `variable.parameter`
  - `variable.other`
    - `variable.other.member`
      - `variable.other.member.private`
- `label`
- `punctuation`
  - `punctuation.delimiter`
  - `punctuation.bracket`
  - `punctuation.special`
- `keyword`
  - `keyword.control`
    - `keyword.control.conditional`
    - `keyword.control.repeat`
    - `keyword.control.import`
    - `keyword.control.return`
    - `keyword.control.exception`
  - `keyword.operator`
  - `keyword.directive`
  - `keyword.function`
  - `keyword.storage`
    - `keyword.storage.type`
    - `keyword.storage.modifier`
- `operator`
- `function`
  - `function.builtin`
  - `function.method`
    - `function.method.private`
  - `function.macro`
  - `function.special`
- `tag`
  - `tag.builtin`
- `namespace`
- `special`

## Markup

- `markup.heading`
  - `markup.heading.marker`
  - `markup.heading.1` .. `markup.heading.6`
  - `markup.heading.completion`
  - `markup.heading.hover`
- `markup.list`
  - `markup.list.unnumbered`
  - `markup.list.numbered`
  - `markup.list.checked`
  - `markup.list.unchecked`
- `markup.bold`
- `markup.italic`
- `markup.strikethrough`
- `markup.link`
  - `markup.link.url`
  - `markup.link.label`
  - `markup.link.text`
- `markup.quote`
- `markup.raw`
  - `markup.raw.inline`
    - `markup.raw.inline.completion`
    - `markup.raw.inline.hover`
  - `markup.raw.block`
- `markup.normal`
  - `markup.normal.completion`
  - `markup.normal.hover`

## Diff

- `diff.plus`
  - `diff.plus.gutter`
- `diff.minus`
  - `diff.minus.gutter`
- `diff.delta`
  - `diff.delta.moved`
  - `diff.delta.conflict`
  - `diff.delta.gutter`

## Diagnostics (inline)

- `diagnostic`
  - `diagnostic.hint`
  - `diagnostic.info`
  - `diagnostic.warning`
  - `diagnostic.error`
  - `diagnostic.unnecessary`
  - `diagnostic.deprecated`

## Diagnostics (severity indicators)

- `warning`
- `error`
- `info`
- `hint`

## UI

### Background
- `ui.background`
- `ui.background.separator`

### Cursor
- `ui.cursor`
  - `ui.cursor.normal`
  - `ui.cursor.insert`
  - `ui.cursor.select`
  - `ui.cursor.match`
  - `ui.cursor.primary`
    - `ui.cursor.primary.normal`
    - `ui.cursor.primary.insert`
    - `ui.cursor.primary.select`

### Cursor line / column
- `ui.cursorline.primary`
- `ui.cursorline.secondary`
- `ui.cursorcolumn.primary`
- `ui.cursorcolumn.secondary`

### Selection
- `ui.selection`
- `ui.selection.primary`

### Highlight
- `ui.highlight`
- `ui.highlight.frameline`

### Debug
- `ui.debug`
- `ui.debug.breakpoint`
- `ui.debug.active`

### Gutter
- `ui.gutter`
- `ui.gutter.selected`
- `ui.gutter.virtual`
- `ui.gutter.selected.virtual`

### Line numbers
- `ui.linenr`
- `ui.linenr.selected`

### Statusline
- `ui.statusline`
  - `ui.statusline.inactive`
  - `ui.statusline.normal`
  - `ui.statusline.insert`
  - `ui.statusline.select`
  - `ui.statusline.separator`
  - `ui.statusline.active`

### Bufferline
- `ui.bufferline`
  - `ui.bufferline.active`
  - `ui.bufferline.background`

### Popup / window
- `ui.popup`
  - `ui.popup.info`
- `ui.window`
- `ui.help`

### Picker
- `ui.picker.header`
  - `ui.picker.header.column`
    - `ui.picker.header.column.active`

### Text
- `ui.text`
  - `ui.text.focus`
  - `ui.text.inactive`
  - `ui.text.info`
  - `ui.text.directory`
  - `ui.text.symlink`

### Virtual
- `ui.virtual`
  - `ui.virtual.ruler`
  - `ui.virtual.whitespace`
  - `ui.virtual.indent-guide`
  - `ui.virtual.inlay-hint`
    - `ui.virtual.inlay-hint.parameter`
    - `ui.virtual.inlay-hint.type`
  - `ui.virtual.wrap`
  - `ui.virtual.jump-label`

### Menu
- `ui.menu`
  - `ui.menu.selected`
  - `ui.menu.scroll`

## Misc

- `tabstop` — snippet placeholder highlighting
