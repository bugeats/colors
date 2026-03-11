mod backends;
mod chord;

use backends::{OklchHex, OklchRgb};
use chord::Chord;
use serde::Serialize;
use serde_json::ser::{PrettyFormatter, Serializer};
use serde_json::{json, Map, Value};

fn palette() -> Vec<(&'static str, Chord)> {
    let bg = Chord::new(0.249193, 0.012013, 0.187791);
    let fg = Chord::new(0.794120, 0.057683, 0.205683);

    let red = Chord::new(0.805840, 0.256460, 0.105556);
    let yellow = red.with_hue(0.261268);
    let orange = red.with_hue(0.164384);
    let green = red.with_hue(0.424914);
    let blue = red.with_hue(0.766209);
    let magenta = red.with_hue(0.889268);
    let brown = Chord::new(0.794724, 0.143555, 0.215162);

    let err_red = Chord::new(0.802315, 0.229178, 0.076616);
    let warn_orange = Chord::new(0.799843, 0.219655, 0.134204);
    let info_seafoam = Chord::new(0.784713, 0.168653, 0.454306);
    let hint_cyan = Chord::new(0.783736, 0.166985, 0.479107);

    let ansi_red = Chord::new(fg.dim().l, 0.225, 0.111111);
    let ansi_yellow = ansi_red.rotate(1.0 / 6.0);
    let ansi_green = ansi_red.rotate(1.0 / 3.0);
    let ansi_blue = ansi_red.rotate(19.0 / 36.0).ansi_desat();
    let ansi_cyan = ansi_blue.rotate(-0.091667).ansi_desat();
    let ansi_magenta = ansi_red.rotate(-1.0 / 18.0);
    let ansi_white = fg;
    let ansi_black = bg;

    // Hue-rotated alternates for secondary roles
    let alt_green = Chord::new(0.788608, 0.176670, 0.352247);
    let alt_blue = Chord::new(0.788950, 0.191060, 0.638341);
    let alt_magenta = Chord::new(0.796963, 0.186518, 0.797688);
    let alt_orange = Chord::new(0.803755, 0.213068, 0.050552);

    let cursor_bg = Chord::new(0.795777, 0.249585, 0.760776);

    let punc = fg.dim().dim().with_chroma(0.35).with_hue(0.091667);

    vec![
        ("COLOR_ANSI_BLACK_DIM", ansi_black.ansi_dim()),
        ("COLOR_ANSI_BLACK", ansi_black),
        ("COLOR_ANSI_BLACK_LIGHT", ansi_black.ansi_bright()),
        ("COLOR_ANSI_WHITE_DIM", ansi_white.ansi_dim()),
        ("COLOR_ANSI_WHITE", ansi_white),
        ("COLOR_ANSI_WHITE_LIGHT", ansi_white.ansi_bright()),
        ("COLOR_ANSI_RED_DIM", ansi_red.ansi_dim()),
        ("COLOR_ANSI_RED", ansi_red),
        ("COLOR_ANSI_RED_LIGHT", ansi_red.ansi_bright()),
        ("COLOR_ANSI_YELLOW_DIM", ansi_yellow.ansi_dim()),
        ("COLOR_ANSI_YELLOW", ansi_yellow),
        ("COLOR_ANSI_YELLOW_LIGHT", ansi_yellow.ansi_bright()),
        ("COLOR_ANSI_GREEN_DIM", ansi_green.ansi_dim()),
        ("COLOR_ANSI_GREEN", ansi_green),
        ("COLOR_ANSI_GREEN_LIGHT", ansi_green.ansi_bright()),
        ("COLOR_ANSI_CYAN_DIM", ansi_cyan.ansi_dim()),
        ("COLOR_ANSI_CYAN", ansi_cyan),
        ("COLOR_ANSI_CYAN_LIGHT", ansi_cyan.ansi_bright()),
        ("COLOR_ANSI_BLUE_DIM", ansi_blue.ansi_dim()),
        ("COLOR_ANSI_BLUE", ansi_blue),
        ("COLOR_ANSI_BLUE_LIGHT", ansi_blue.ansi_bright()),
        ("COLOR_ANSI_MAGENTA_DIM", ansi_magenta.ansi_dim()),
        ("COLOR_ANSI_MAGENTA", ansi_magenta),
        ("COLOR_ANSI_MAGENTA_LIGHT", ansi_magenta.ansi_bright()),
        //
        ("COLOR_NORMAL_BG", bg),
        ("COLOR_NORMAL_BG_ALT", bg.veryfaint(bg.l)),
        ("COLOR_NORMAL_FG", fg),
        ("COLOR_NORMAL_FG_ALT", fg.fgdim(fg.l)),
        ("COLOR_COMMENT_FG", fg.dim()),
        ("COLOR_PUNCTUATION_FG", punc),
        ("COLOR_PUNCTUATION_ACTIVE_FG", punc.ansi_bright()),
        ("COLOR_PUNCTUATION_ACTIVE_BG", bg.interp(brown, 4.0 / 12.0)),
        ("COLOR_PUNCTUATION_FAINT_BG", punc.faint(bg.l)),
        ("COLOR_CURSOR_BG", cursor_bg),
        ("COLOR_ERROR_BG", magenta.veryfaint(bg.l)),
        ("COLOR_ERROR_FG", magenta),
        ("COLOR_KEYWORD_FG_ALT", alt_green),
        ("COLOR_KEYWORD_FG", green),
        ("COLOR_SELECTION_BG_ALT", alt_blue.faint(bg.l)),
        ("COLOR_SELECTION_BG", blue.faint(bg.l)),
        ("COLOR_STRING_FG_ALT", orange),
        ("COLOR_STRING_FG", yellow),
        ("COLOR_TYPE_FG", fg.interp(bg, 1.0 / 12.0)),
        ("COLOR_UI_LEVEL_1_BG", bg.interp(brown, 1.0 / 24.0)),
        ("COLOR_UI_LEVEL_1_FG", bg.interp(brown, 4.0 / 12.0)),
        ("COLOR_UI_LEVEL_2_BG", bg.interp(brown, 2.0 / 24.0)),
        ("COLOR_UI_LEVEL_2_FG", bg.interp(brown, 8.0 / 12.0)),
        ("COLOR_UI_LEVEL_3_BG", bg.interp(brown, 3.0 / 24.0)),
        ("COLOR_UI_LEVEL_3_FG", brown),
        ("COLOR_VISIBLE_WHITESPACE_FG", brown.faint(bg.l)),
        ("BG_ERR", magenta.veryfaint(bg.l)),
        ("BG_WARN", alt_magenta.veryfaint(bg.l)),
        ("BG_INFO", orange.veryfaint(bg.l)),
        ("BG_HINT", alt_orange.veryfaint(bg.l)),
        ("FG_ERR", err_red.dim()),
        ("FG_WARN", warn_orange.dim()),
        ("FG_INFO", info_seafoam.dim()),
        ("FG_HINT", hint_cyan.dim()),
    ]
}

fn print_json(palette: &[(&str, Chord)]) {
    let mut hex_map = Map::new();
    let mut rgb_map = Map::new();

    for (name, color) in palette {
        let hex = color.render::<OklchHex>();
        hex_map.insert(name.to_string(), Value::String(hex.to_string()));

        let rgb = color.render::<OklchRgb>();
        let mut entry = Map::new();
        entry.insert("r".into(), Value::Number((rgb.r as i64).into()));
        entry.insert("g".into(), Value::Number((rgb.g as i64).into()));
        entry.insert("b".into(), Value::Number((rgb.b as i64).into()));
        rgb_map.insert(name.to_string(), Value::Object(entry));
    }

    let output = json!({
        "colors": {
            "hex": Value::Object(hex_map),
            "rgb": Value::Object(rgb_map),
        }
    });

    let mut buf = Vec::new();
    let formatter = PrettyFormatter::with_indent(b"    ");
    let mut ser = Serializer::with_formatter(&mut buf, formatter);
    output.serialize(&mut ser).unwrap();

    println!("{}", String::from_utf8(buf).unwrap());
}

fn print_table(palette: &[(&str, Chord)]) {
    const BLOCK: char = '\u{2588}';
    let max_name = palette.iter().map(|(n, _)| n.len()).max().unwrap_or(0);

    for (name, color) in palette {
        let rgb = color.render::<OklchRgb>();
        let hex = color.render::<OklchHex>();

        println!(
            "{:<width$}  \x1b[38;2;{};{};{}m{BLOCK}{BLOCK}{BLOCK}{BLOCK}\x1b[0m {hex}",
            name,
            rgb.r,
            rgb.g,
            rgb.b,
            width = max_name,
        );
    }
}

fn main() {
    let palette = palette();

    if std::env::args().any(|a| a == "--json") {
        print_json(&palette);
    } else {
        print_table(&palette);
    }
}
