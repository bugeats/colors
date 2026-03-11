use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;

use Node::*;

#[derive(Clone, Copy, Serialize)]
struct Underline {
    color: &'static str,
    style: &'static str,
}

#[derive(Clone, Copy)]
struct Style {
    fg: Option<&'static str>,
    bg: Option<&'static str>,
    underline: Option<Underline>,
    modifiers: &'static [&'static str],
}

impl Style {
    const fn new() -> Self {
        Self {
            fg: None,
            bg: None,
            underline: None,
            modifiers: &[],
        }
    }

    fn is_empty(&self) -> bool {
        self.fg.is_none()
            && self.bg.is_none()
            && self.underline.is_none()
            && self.modifiers.is_empty()
    }

    /// Child overrides parent per-field; empty child fields inherit.
    fn merge(self, child: Self) -> Self {
        Self {
            fg: child.fg.or(self.fg),
            bg: child.bg.or(self.bg),
            underline: child.underline.or(self.underline),
            modifiers: if child.modifiers.is_empty() {
                self.modifiers
            } else {
                child.modifiers
            },
        }
    }

    const fn fg(mut self, fg: &'static str) -> Self {
        self.fg = Some(fg);
        self
    }

    const fn bg(mut self, bg: &'static str) -> Self {
        self.bg = Some(bg);
        self
    }

    const fn underline(mut self, color: &'static str, style: &'static str) -> Self {
        self.underline = Some(Underline { color, style });
        self
    }

    const fn modifiers(mut self, modifiers: &'static [&'static str]) -> Self {
        self.modifiers = modifiers;
        self
    }
}

/// Bare string for fg-only (Helix shorthand), table otherwise.
impl Serialize for Style {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let is_fg_only = self.bg.is_none() && self.underline.is_none() && self.modifiers.is_empty();

        if is_fg_only {
            if let Some(fg) = self.fg {
                return serializer.serialize_str(fg);
            }
        }

        let field_count = self.fg.is_some() as usize
            + self.bg.is_some() as usize
            + self.underline.is_some() as usize
            + (!self.modifiers.is_empty()) as usize;

        let mut map = serializer.serialize_map(Some(field_count))?;

        if let Some(fg) = self.fg {
            map.serialize_entry("fg", fg)?;
        }

        if let Some(bg) = self.bg {
            map.serialize_entry("bg", bg)?;
        }

        if let Some(ref underline) = self.underline {
            map.serialize_entry("underline", underline)?;
        }

        if !self.modifiers.is_empty() {
            map.serialize_entry("modifiers", self.modifiers)?;
        }

        map.end()
    }
}

#[derive(Clone, Copy)]
enum Node {
    Section(Style, &'static [(&'static str, Node)]),
    Branch(Style, &'static [(&'static str, Node)]),
    Leaf(Style),
}

macro_rules! node_style {
    ($name:ident, $($param:ident : $ty:ty),+) => {
        const fn $name(self, $($param: $ty),+) -> Self {
            match self {
                Leaf(s) => Leaf(s.$name($($param),+)),
                Branch(s, c) => Branch(s.$name($($param),+), c),
                Section(s, c) => Section(s.$name($($param),+), c),
            }
        }
    };
}

impl Node {
    node_style!(fg, fg: &'static str);
    node_style!(bg, bg: &'static str);
    node_style!(underline, color: &'static str, style: &'static str);
    node_style!(modifiers, modifiers: &'static [&'static str]);
}

const fn leaf() -> Node {
    Leaf(Style::new())
}

const fn branch(children: &'static [(&'static str, Node)]) -> Node {
    Branch(Style::new(), children)
}

const fn section(children: &'static [(&'static str, Node)]) -> Node {
    Section(Style::new(), children)
}

/// Format a toml::Value as inline TOML (inline tables, not sections).
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
    if style.is_empty() {
        return;
    }

    let value = toml::Value::try_from(style).unwrap();
    println!("{} = {}", toml_key(path), fmt_inline(&value));
}

fn emit_node(path: &str, node: &Node, inherited: Style) {
    match node {
        Section(style, children) => {
            let cascaded = inherited.merge(*style);

            for (name, child) in *children {
                emit_node(name, child, cascaded);
            }
        }

        Branch(style, children) => {
            let cascaded = inherited.merge(*style);
            emit_scope(path, &cascaded);

            for (name, child) in *children {
                emit_node(&format!("{path}.{name}"), child, cascaded);
            }
        }

        Leaf(style) => {
            let cascaded = inherited.merge(*style);
            emit_scope(path, &cascaded);
        }
    }
}

pub fn print_helix() {
    for (name, node) in THEME {
        emit_node(name, node, Style::new());
    }
}

const THEME: &[(&str, Node)] = &[
    (
        "Syntax",
        section(&[
            ("comment", leaf().fg("comment-fg")),
            (
                "keyword",
                branch(&[("directive", leaf().fg("keyword-fg-alt"))]).fg("keyword-fg"),
            ),
            ("string", leaf().fg("string-fg")),
            (
                "constant",
                branch(&[
                    ("numeric", leaf().fg("string-fg-alt")),
                    (
                        "character",
                        branch(&[("escape", leaf().fg("keyword-fg-alt"))]),
                    ),
                    ("builtin", leaf()),
                ])
                .fg("string-fg"),
            ),
            ("type", branch(&[("builtin", leaf())]).fg("type-fg")),
            (
                "function",
                branch(&[("macro", leaf().fg("keyword-fg")), ("builtin", leaf())]).fg("normal-fg"),
            ),
            (
                "variable",
                branch(&[
                    ("builtin", leaf().fg("keyword-fg-alt")),
                    ("other", branch(&[("member", leaf().fg("normal-fg-alt"))])),
                    ("parameter", leaf()),
                ])
                .fg("normal-fg"),
            ),
            (
                "punctuation",
                branch(&[("delimiter", leaf())]).fg("punctuation-fg"),
            ),
            ("operator", leaf().fg("keyword-fg")),
            ("tag", leaf().fg("keyword-fg")),
            ("label", leaf().fg("string-fg-alt")),
            ("namespace", leaf().fg("keyword-fg-alt")),
            ("attribute", leaf().fg("keyword-fg-alt")),
            ("constructor", leaf().fg("type-fg")),
            ("special", leaf().fg("string-fg-alt")),
        ]),
    ),
    (
        "Markup",
        section(&[(
            "markup",
            branch(&[
                ("heading", leaf().fg("keyword-fg")),
                ("bold", leaf().modifiers(&["bold"])),
                ("italic", leaf().modifiers(&["italic"])),
                ("strikethrough", leaf().modifiers(&["crossed_out"])),
                (
                    "link",
                    branch(&[
                        ("url", leaf().fg("comment-fg").modifiers(&["underlined"])),
                        ("text", leaf().fg("keyword-fg")),
                    ]),
                ),
                ("raw", leaf().fg("string-fg")),
            ]),
        )]),
    ),
    (
        "Diff",
        section(&[(
            "diff",
            branch(&[
                ("plus", leaf().fg("ansi-green")),
                ("minus", leaf().fg("ansi-red")),
                ("delta", leaf().fg("ansi-blue")),
            ]),
        )]),
    ),
    (
        "UI",
        section(&[(
            "ui",
            branch(&[
                (
                    "background",
                    branch(&[("separator", leaf().fg("comment-fg"))]).bg("normal-bg"),
                ),
                (
                    "linenr",
                    branch(&[("selected", leaf().fg("normal-fg"))]).fg("comment-fg"),
                ),
                (
                    "statusline",
                    branch(&[("inactive", leaf().fg("comment-fg").bg("ui-level-1-bg"))])
                        .fg("normal-fg")
                        .bg("ui-level-1-bg"),
                ),
                ("popup", leaf().bg("ui-level-2-bg")),
                ("window", leaf().fg("ui-level-2-fg")),
                ("help", leaf().fg("normal-fg").bg("ui-level-3-bg")),
                (
                    "text",
                    branch(&[
                        ("focus", leaf().fg("normal-fg-alt")),
                        ("inactive", leaf().fg("comment-fg")),
                        ("directory", leaf().fg("keyword-fg")),
                    ])
                    .fg("normal-fg"),
                ),
                (
                    "virtual",
                    branch(&[
                        ("ruler", leaf().bg("ui-level-1-bg")),
                        ("indent-guide", leaf()),
                        ("jump-label", leaf().fg("error-fg").modifiers(&["bold"])),
                    ])
                    .fg("visible-whitespace-fg"),
                ),
                (
                    "selection",
                    branch(&[("primary", leaf())]).bg("selection-bg"),
                ),
                (
                    "cursor",
                    branch(&[
                        ("select", leaf().bg("cursor-bg")),
                        ("insert", leaf().bg("normal-fg")),
                        (
                            "primary",
                            branch(&[
                                ("select", leaf().bg("cursor-bg")),
                                ("insert", leaf().bg("normal-fg")),
                            ]),
                        ),
                        ("match", leaf().bg("selection-bg-alt")),
                    ])
                    .modifiers(&["reversed"]),
                ),
                (
                    "cursorline",
                    branch(&[("primary", leaf().bg("ui-level-1-bg"))]),
                ),
                (
                    "highlight",
                    branch(&[("frameline", leaf().bg("error-bg"))]).bg("selection-bg"),
                ),
                (
                    "debug",
                    branch(&[("breakpoint", leaf().fg("error-fg"))]).fg("error-bg"),
                ),
                (
                    "menu",
                    branch(&[
                        ("selected", leaf().fg("ui-level-2-bg").bg("normal-fg")),
                        ("scroll", leaf().fg("comment-fg").bg("ui-level-1-bg")),
                    ])
                    .fg("normal-fg")
                    .bg("ui-level-2-bg"),
                ),
            ]),
        )]),
    ),
    (
        "Diagnostics",
        section(&[
            (
                "diagnostic",
                branch(&[
                    ("hint", leaf().underline("fg-hint", "curl")),
                    ("info", leaf().underline("fg-info", "curl")),
                    ("warning", leaf().underline("fg-warn", "curl")),
                    ("error", leaf().underline("fg-err", "curl")),
                    ("unnecessary", leaf().modifiers(&["dim"])),
                    ("deprecated", leaf().modifiers(&["crossed_out"])),
                ]),
            ),
            ("warning", leaf().fg("fg-warn")),
            ("error", leaf().fg("fg-err")),
            ("info", leaf().fg("fg-info")),
            ("hint", leaf().fg("fg-hint")),
        ]),
    ),
];
