use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;

use crate::backends::ThemeRgb;
use crate::chord::Chord;

use Modifier::*;
use UnderlineStyle::*;

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
enum Modifier {
    Bold,
    Dim,
    Italic,
    Underlined,
    SlowBlink,
    RapidBlink,
    Reversed,
    Hidden,
    CrossedOut,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
enum UnderlineStyle {
    Line,
    Curl,
    Dashed,
    Dotted,
    DoubleLine,
}

#[derive(Clone, Copy, Default)]
enum Underline {
    #[default]
    None,
    Styled {
        color: Chord,
        style: UnderlineStyle,
    },
}

impl Underline {
    fn is_none(&self) -> bool {
        matches!(self, Underline::None)
    }

    /// Child overrides parent; None inherits.
    fn merge(self, child: Self) -> Self {
        if child.is_none() {
            self
        } else {
            child
        }
    }
}

impl Serialize for Underline {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Underline::None => serializer.serialize_none(),

            Underline::Styled { color, style } => {
                let mut map = serializer.serialize_map(None)?;

                if !color.is_default() {
                    map.serialize_entry("color", &ThemeRgb::from(color.middle()).to_string())?;
                }

                map.serialize_entry("style", style)?;
                map.end()
            }
        }
    }
}

#[derive(Clone, Copy, Default)]
struct Style {
    color: Chord,
    underline: Underline,
    modifiers: &'static [Modifier],
}

impl Style {
    /// Child overrides parent per-field; empty child fields inherit.
    fn merge(self, child: Self) -> Self {
        Self {
            color: if child.color.is_default() {
                self.color
            } else {
                child.color
            },
            underline: self.underline.merge(child.underline),
            modifiers: if child.modifiers.is_empty() {
                self.modifiers
            } else {
                child.modifiers
            },
        }
    }
}

impl Serialize for Style {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let fg = ThemeRgb::from(self.color.middle()).to_string();
        let bg = ThemeRgb::from(self.color.bottom()).to_string();

        let field_count =
            2 + (!self.underline.is_none()) as usize + (!self.modifiers.is_empty()) as usize;

        let mut map = serializer.serialize_map(Some(field_count))?;
        map.serialize_entry("fg", fg.as_str())?;
        map.serialize_entry("bg", bg.as_str())?;

        if !self.underline.is_none() {
            map.serialize_entry("underline", &self.underline)?;
        }

        if !self.modifiers.is_empty() {
            map.serialize_entry("modifiers", self.modifiers)?;
        }

        map.end()
    }
}

struct Node {
    name: &'static str,
    style: Style,
    children: Vec<Node>,
}

impl Node {
    fn child(mut self, node: Node) -> Self {
        self.children.push(node);
        self
    }

    fn color(mut self, color: Chord) -> Self {
        self.style.color = color;
        self
    }

    fn underline(mut self, color: Chord, style: UnderlineStyle) -> Self {
        self.style.underline = Underline::Styled { color, style };
        self
    }

    fn modifiers(mut self, modifiers: &'static [Modifier]) -> Self {
        self.style.modifiers = modifiers;
        self
    }
}

fn node(name: &'static str) -> Node {
    Node {
        name,
        style: Style::default(),
        children: Vec::new(),
    }
}

fn fmt_inline(value: &toml::Value) -> String {
    match value {
        toml::Value::Table(table) => {
            let entries: Vec<_> = table
                .iter()
                .map(|(k, v)| format!("{k} = {}", fmt_inline(v)))
                .collect();
            format!("{{ {} }}", entries.join(", "))
        }

        toml::Value::Array(arr) => {
            let items: Vec<_> = arr.iter().map(fmt_inline).collect();
            format!("[{}]", items.join(", "))
        }

        other => other.to_string(),
    }
}

fn toml_key(key: &str) -> String {
    if key.contains('.') {
        format!("\"{key}\"")
    } else {
        key.to_string()
    }
}

fn emit_scope(path: &str, style: &Style) {
    let value = toml::Value::try_from(style).unwrap();
    println!("{} = {}", toml_key(path), fmt_inline(&value));
}

fn emit_node(prefix: &str, node: &Node, inherited: Style) {
    let path = if node.name.is_empty() {
        prefix.to_string()
    } else if prefix.is_empty() {
        node.name.to_string()
    } else {
        format!("{prefix}.{}", node.name)
    };

    let cascaded = inherited.merge(node.style);

    if !node.name.is_empty() {
        emit_scope(&path, &cascaded);
    }

    for child in &node.children {
        emit_node(&path, child, cascaded);
    }
}

pub fn print_helix() {
    emit_node("", &theme(), Style::default());
}

fn theme() -> Node {
    node("")
        .child(node("attribute"))
        .child(
            node("type")
                .child(node("builtin"))
                .child(node("parameter"))
                .child(node("enum").child(node("variant"))),
        )
        .child(node("constructor"))
        .child(
            node("constant")
                .child(node("builtin").child(node("boolean")))
                .child(node("character").child(node("escape")))
                .child(node("numeric").child(node("integer")).child(node("float"))),
        )
        .child(
            node("string").child(node("regexp")).child(
                node("special")
                    .child(node("path"))
                    .child(node("url"))
                    .child(node("symbol")),
            ),
        )
        .child(
            node("comment")
                .child(node("line").child(node("documentation")))
                .child(node("block").child(node("documentation")))
                .child(node("unused")),
        )
        .child(
            node("variable")
                .child(node("builtin"))
                .child(node("parameter"))
                .child(node("other").child(node("member").child(node("private")))),
        )
        .child(node("label"))
        .child(
            node("punctuation")
                .child(node("delimiter"))
                .child(node("bracket"))
                .child(node("special")),
        )
        .child(
            node("keyword")
                .child(
                    node("control")
                        .child(node("conditional"))
                        .child(node("repeat"))
                        .child(node("import"))
                        .child(node("return"))
                        .child(node("exception")),
                )
                .child(node("operator"))
                .child(node("directive"))
                .child(node("function"))
                .child(node("storage").child(node("type")).child(node("modifier"))),
        )
        .child(node("operator"))
        .child(
            node("function")
                .child(node("builtin"))
                .child(node("method").child(node("private")))
                .child(node("macro"))
                .child(node("special")),
        )
        .child(node("tag").child(node("builtin")))
        .child(node("namespace"))
        .child(node("special"))
        .child(
            node("markup")
                .child(
                    node("heading")
                        .child(node("marker"))
                        .child(node("1"))
                        .child(node("2"))
                        .child(node("3"))
                        .child(node("4"))
                        .child(node("5"))
                        .child(node("6"))
                        .child(node("completion"))
                        .child(node("hover")),
                )
                .child(
                    node("list")
                        .child(node("unnumbered"))
                        .child(node("numbered"))
                        .child(node("checked"))
                        .child(node("unchecked")),
                )
                .child(node("bold").modifiers(&[Bold]))
                .child(node("italic").modifiers(&[Italic]))
                .child(node("strikethrough").modifiers(&[CrossedOut]))
                .child(
                    node("link")
                        .child(node("url").modifiers(&[Underlined]))
                        .child(node("label"))
                        .child(node("text")),
                )
                .child(node("quote"))
                .child(
                    node("raw")
                        .child(
                            node("inline")
                                .child(node("completion"))
                                .child(node("hover")),
                        )
                        .child(node("block")),
                )
                .child(
                    node("normal")
                        .child(node("completion"))
                        .child(node("hover")),
                ),
        )
        .child(
            node("diff")
                .child(node("plus").child(node("gutter")))
                .child(node("minus").child(node("gutter")))
                .child(
                    node("delta")
                        .child(node("moved"))
                        .child(node("conflict"))
                        .child(node("gutter")),
                ),
        )
        .child(
            node("diagnostic")
                .child(node("hint").underline(Chord::default(), Curl))
                .child(node("info").underline(Chord::default(), Curl))
                .child(node("warning").underline(Chord::default(), Curl))
                .child(node("error").underline(Chord::default(), Curl))
                .child(node("unnecessary").modifiers(&[Dim]))
                .child(node("deprecated").modifiers(&[CrossedOut])),
        )
        .child(node("warning"))
        .child(node("error"))
        .child(node("info"))
        .child(node("hint"))
        .child(
            node("ui")
                .child(node("background").child(node("separator")))
                .child(
                    node("cursor")
                        .modifiers(&[Reversed])
                        .child(node("normal"))
                        .child(node("insert"))
                        .child(node("select"))
                        .child(node("match"))
                        .child(
                            node("primary")
                                .child(node("normal"))
                                .child(node("insert"))
                                .child(node("select")),
                        ),
                )
                .child(
                    node("cursorline")
                        .child(node("primary"))
                        .child(node("secondary")),
                )
                .child(
                    node("cursorcolumn")
                        .child(node("primary"))
                        .child(node("secondary")),
                )
                .child(node("selection").child(node("primary")))
                .child(node("highlight").child(node("frameline")))
                .child(
                    node("debug")
                        .child(node("breakpoint"))
                        .child(node("active")),
                )
                .child(
                    node("gutter")
                        .child(node("selected").child(node("virtual")))
                        .child(node("virtual")),
                )
                .child(node("linenr").child(node("selected")))
                .child(
                    node("statusline")
                        .child(node("inactive"))
                        .child(node("normal"))
                        .child(node("insert"))
                        .child(node("select"))
                        .child(node("separator"))
                        .child(node("active")),
                )
                .child(
                    node("bufferline")
                        .child(node("active"))
                        .child(node("background")),
                )
                .child(node("popup").child(node("info")))
                .child(node("window"))
                .child(node("help"))
                .child(
                    node("picker")
                        .child(node("header").child(node("column").child(node("active")))),
                )
                .child(
                    node("text")
                        .child(node("focus"))
                        .child(node("inactive"))
                        .child(node("info"))
                        .child(node("directory"))
                        .child(node("symlink")),
                )
                .child(
                    node("virtual")
                        .child(node("ruler"))
                        .child(node("whitespace"))
                        .child(node("indent-guide"))
                        .child(
                            node("inlay-hint")
                                .child(node("parameter"))
                                .child(node("type")),
                        )
                        .child(node("wrap"))
                        .child(node("jump-label").modifiers(&[Bold])),
                )
                .child(node("menu").child(node("selected")).child(node("scroll"))),
        )
        .child(node("tabstop"))
}
