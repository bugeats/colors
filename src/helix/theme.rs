use super::node::{Node, node};
use super::style::{Modifier, UnderlineStyle};
use crate::chord::Chord;

use Modifier::*;
use UnderlineStyle::*;
use nalgebra::Vector3;

pub(super) fn theme() -> Node {
    let markup = node("markup")
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
        );

    let ui = node("ui")
        .transform(|chord| chord.mk_red())
        .child(node("background").child(node("separator")))
        .child(
            node("cursor")
                .transform(|chord| chord.mk_blue())
                .modifiers(&[Reversed])
                .child(node("normal"))
                .child(node("insert").transform(|chord| chord.mk_green()))
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
        .child(node("picker").child(node("header").child(node("column").child(node("active")))))
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
        .child(node("menu").child(node("selected")).child(node("scroll")));

    let keyword = node("keyword")
        .transform(|c| c.mk_saturated().mk_green())
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
        .child(
            node("storage")
                .transform(|c| c.mk_bamp(5))
                .child(node("type").transform(|c| c.mk_bamp(12)))
                .child(node("modifier").transform(|c| c.mk_bamp(7))),
        );

    let root = node("")
        .transform(|_| {
            Chord::from(Vector3::new(0.79, 0.035, 0.197)).set_interval([1.06, 0.02, -0.03])
        })
        .child(node("attribute"))
        .child(node("tabstop"))
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
        .child(keyword)
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
        .child(markup)
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
        .child(ui);

    root
}
