use super::node::{Node, node};
use super::style::{Modifier, UnderlineStyle};
use crate::chord::Chord;
use crate::helix::theme::palette::punc;

use Modifier::*;
use UnderlineStyle::*;
use nalgebra::Vector3;

mod palette {
    use super::*;

    pub fn normal() -> Chord {
        Chord::from(Vector3::new(0.80, 0.05, 0.20)).set_interval([1.11, 0.05, -0.03].into())
    }

    pub fn punc() -> Chord {
        normal().mk_red().shift_sat(0.2).push_back().push_back()
    }

    pub fn brown() -> Chord {
        let norm = normal();
        let intr = norm.interval + Vector3::new(-0.45, 0.03, 0.1);

        normal()
            .rotate(3.0 / 48.0)
            .set_sat(norm.sat() + 0.08)
            .set_lit(norm.lit() - 0.18)
            .set_interval(intr)
    }
}
// ----

pub(super) fn theme() -> Node {
    let keyword = node("keyword")
        .transform(|c| c.mk_saturated().mk_green().pin_bottom(c))
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

    node("")
        .transform(|_| palette::normal())
        .child(markup())
        .child(node("attribute"))
        .child(node("tabstop"))
        .child(
            node("type")
                .modifiers(&[Modifier::Bold])
                .transform(|c| c.dust())
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
            node("string")
                .transform(|c| c.mk_saturated().mk_yellow().pin_bottom(c))
                .child(node("regexp"))
                .child(
                    node("special")
                        .child(node("path"))
                        .child(node("url"))
                        .child(node("symbol")),
                ),
        )
        .child(
            node("variable")
                .child(node("builtin"))
                .child(node("parameter"))
                .child(node("other").child(node("member").child(node("private")))),
        )
        .child(node("label"))
        .child(keyword)
        .child(node("operator"))
        .child(
            node("function")
                .transform(|c| c.shimmer().pin_bottom(&palette::normal()))
                .child(node("builtin"))
                .child(node("method").child(node("private")))
                .child(node("macro"))
                .child(node("special")),
        )
        .child(node("tag").child(node("builtin")))
        .child(node("namespace").modifiers(&[Modifier::Bold]))
        .child(node("special"))
        .child(
            node("comment")
                .transform(|c| c.desaturated().push_back().pin_bottom(c))
                .child(node("line").child(node("documentation")))
                .child(node("block").child(node("documentation")))
                .child(node("unused")),
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
        .child(ui())
        .child(
            node("punctuation")
                .transform(|c| punc().pin_bottom(c))
                .child(node("delimiter"))
                .child(node("bracket"))
                .child(node("special")),
        )
        .child(
            node("diff")
                .transform(|c| {
                    c.candy()
                        .mk_blue()
                        .push_back()
                        .push_back()
                        .push_back()
                        .push_back()
                        .pin_bottom(&palette::normal())
                })
                .child(
                    node("plus")
                        .transform(|c| c.mk_green().pin_bottom(&palette::normal()))
                        .child(node("gutter")),
                )
                .child(
                    node("minus")
                        .child(node("gutter"))
                        .transform(|c| c.mk_red()),
                )
                .child(
                    node("delta")
                        .transform(|c| c.mk_orange())
                        .child(node("moved"))
                        .child(node("conflict").transform(|c| {
                            let red = palette::normal().candy().mk_red();
                            c.mix(&red.pin_bottom(&red)).push_back().push_back()
                        }))
                        .child(node("gutter")),
                ),
        )
}

fn markup() -> Node {
    node("markup")
        .child(
            node("heading")
                .modifiers(&[Modifier::Bold])
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
        )
}

fn cursor() -> Node {
    let norm = node("normal");

    let insert = node("insert")
        .modifiers(&[Modifier::Reversed])
        .transform(|c| c.mk_red());

    let select = node("select").transform(|c| c.mk_green());

    node("cursor")
        .transform(|chord| chord.mk_blue().candy().inverted())
        .child(norm.clone())
        .child(insert.clone())
        .child(select.clone())
        .child(node("match").transform(|_| {
            palette::punc()
                .candy()
                .pop_up()
                .pin_bottom(&palette::normal())
        }))
        .child(
            node("primary")
                .transform(|c| c.pop_up())
                .child(norm)
                .child(insert)
                .child(select),
        )
}

fn ui() -> Node {
    node("ui")
        .transform(|_| palette::brown())
        .child(
            node("background")
                .transform(|_| palette::normal())
                .child(node("separator").transform(|_| palette::brown().pop_up())),
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
        .child(
            node("highlight")
                .transform(|c| c.inverted().push_back())
                .child(node("frameline")),
        )
        .child(
            node("debug")
                .child(node("breakpoint"))
                .child(node("active")),
        )
        .child(
            node("gutter")
                .transform(|_| palette::normal().faintly())
                .child(
                    node("selected")
                        .transform(|c| c.pop_up().pin_bottom(c))
                        .child(node("virtual")),
                )
                .child(node("virtual")),
        )
        .child(
            node("linenr")
                .transform(|_| palette::normal().faintly().pin_bottom(&palette::normal()))
                .child(node("selected").transform(|c| c.pop_up().pin_bottom(c))),
        )
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
        .child(node("window").transform(|c| c.push_back().push_back().pin_bottom(c)))
        .child(node("help"))
        .child(picker())
        .child(
            node("text")
                .transform(|_| palette::normal())
                .child(node("focus").transform(|c| c.pin_bottom(&palette::brown()).pop_up()))
                .child(node("inactive").transform(|c| c.push_back().push_back().pin_bottom(c)))
                .child(node("info"))
                .child(node("directory").transform(|c| c.push_back().pin_bottom(c)))
                .child(node("symlink").transform(|c| c.alt(1001))),
        )
        .child(virtualx())
        .child(
            node("menu")
                .transform(|c| c.pop_up())
                .child(node("selected").transform(|c| c.inverted().push_back()))
                .child(node("scroll")),
        )
        .child(cursor())
}

fn picker() -> Node {
    node("picker").transform(|c| c.pop_up()).child(
        node("header").transform(|c| c.pop_up()).child(
            node("column")
                .transform(|c| c.pop_up())
                .child(node("active").transform(|c| c.inverted().push_back().push_back())),
        ),
    )
}

fn virtualx() -> Node {
    node("virtual")
        .transform(|_| palette::brown())
        .child(node("ruler"))
        .child(node("whitespace").transform(|c| c.faintly().pin_bottom(&palette::normal())))
        .child(node("indent-guide").transform(|c| c.mk_void().pin_bottom(&palette::normal())))
        .child(
            node("inlay-hint")
                .child(node("parameter"))
                .child(node("type")),
        )
        .child(node("wrap"))
        .child(node("jump-label").modifiers(&[Bold]))
}
