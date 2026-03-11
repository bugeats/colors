mod backends;
mod chord;

use backends::{OklchHex, OklchRgb};
use chord::{Chord, Color};
use serde::Serialize;
use serde_json::ser::{PrettyFormatter, Serializer};
use serde_json::{json, Map, Value};

fn palette() -> Vec<(&'static str, Color)> {
    // Root chords — the only place Color is constructed
    let normal = Chord::from(Color::new(0.52, 0.035, 0.197));
    let chromatic = Chord::from(Color::new(0.52, 0.18, 0.106));

    // ANSI — black is collapsed normal, chromatic colors evenly spaced at 1/6 rotation
    let black = normal.set_interval([0.0, 0.0, 0.0]);

    let ansi_red = chromatic.ansi();
    let ansi_yellow = ansi_red.rotate(1.0 / 6.0);
    let ansi_green = ansi_red.rotate(2.0 / 6.0);
    let ansi_cyan = ansi_red.rotate(3.0 / 6.0);
    let ansi_blue = ansi_red.rotate(4.0 / 6.0);
    let ansi_magenta = ansi_red.rotate(5.0 / 6.0);

    // Punctuation — desaturated, collapsed to dim register
    let punctuation = normal.set_interval([0.0, 0.0, 0.0]).dim().dim();
    let punc_active = punctuation.light().set_interval([0.15, 0.0, 0.0]);

    // Semantic
    let error = chromatic.rotate(0.783);
    let cursor = chromatic.rotate(0.655);

    // Syntax
    let keyword = chromatic.rotate(0.319);
    let keyword_alt = chromatic.rotate(0.246);
    let selection = chromatic.rotate(0.660);
    let selection_alt = chromatic.rotate(0.532);
    let string_yellow = chromatic.rotate(0.155);
    let string_orange = chromatic.rotate(0.058);

    // UI levels — progressively wider intervals
    let ui_level_1 = normal.set_interval([0.20, 0.05, 0.0]);
    let ui_level_2 = normal.set_interval([0.35, 0.08, 0.0]);
    let ui_level_3 = normal.set_interval([0.50, 0.10, 0.0]);

    // Diagnostics
    let err_diag = chromatic.rotate(-0.029);
    let warn_diag = chromatic.rotate(0.028);
    let info_diag = chromatic.rotate(0.348);
    let hint_diag = chromatic.rotate(0.373);

    vec![
        ("COLOR_ANSI_BLACK_DIM", black.dim().top()),
        ("COLOR_ANSI_BLACK", black.top()),
        ("COLOR_ANSI_BLACK_LIGHT", black.light().top()),
        ("COLOR_ANSI_WHITE_DIM", normal.dim().top()),
        ("COLOR_ANSI_WHITE", normal.top()),
        ("COLOR_ANSI_WHITE_LIGHT", normal.light().top()),
        ("COLOR_ANSI_RED_DIM", ansi_red.dim().top()),
        ("COLOR_ANSI_RED", ansi_red.top()),
        ("COLOR_ANSI_RED_LIGHT", ansi_red.light().top()),
        ("COLOR_ANSI_YELLOW_DIM", ansi_yellow.dim().top()),
        ("COLOR_ANSI_YELLOW", ansi_yellow.top()),
        ("COLOR_ANSI_YELLOW_LIGHT", ansi_yellow.light().top()),
        ("COLOR_ANSI_GREEN_DIM", ansi_green.dim().top()),
        ("COLOR_ANSI_GREEN", ansi_green.top()),
        ("COLOR_ANSI_GREEN_LIGHT", ansi_green.light().top()),
        ("COLOR_ANSI_CYAN_DIM", ansi_cyan.dim().top()),
        ("COLOR_ANSI_CYAN", ansi_cyan.top()),
        ("COLOR_ANSI_CYAN_LIGHT", ansi_cyan.light().top()),
        ("COLOR_ANSI_BLUE_DIM", ansi_blue.dim().top()),
        ("COLOR_ANSI_BLUE", ansi_blue.top()),
        ("COLOR_ANSI_BLUE_LIGHT", ansi_blue.light().top()),
        ("COLOR_ANSI_MAGENTA_DIM", ansi_magenta.dim().top()),
        ("COLOR_ANSI_MAGENTA", ansi_magenta.top()),
        ("COLOR_ANSI_MAGENTA_LIGHT", ansi_magenta.light().top()),
        //
        ("COLOR_NORMAL_BG", normal.bottom()),
        ("COLOR_NORMAL_BG_ALT", normal.soften().bottom()),
        ("COLOR_NORMAL_FG", normal.top()),
        ("COLOR_NORMAL_FG_ALT", normal.soften().top()),
        ("COLOR_COMMENT_FG", normal.dim().top()),
        ("COLOR_PUNCTUATION_FG", punctuation.middle()),
        ("COLOR_PUNCTUATION_ACTIVE_FG", punc_active.top()),
        ("COLOR_PUNCTUATION_ACTIVE_BG", punc_active.bottom()),
        ("COLOR_PUNCTUATION_FAINT_BG", punctuation.faint().middle()),
        ("COLOR_CURSOR_BG", cursor.top()),
        ("COLOR_ERROR_BG", error.tint().middle()),
        ("COLOR_ERROR_FG", error.top()),
        ("COLOR_KEYWORD_FG_ALT", keyword_alt.top()),
        ("COLOR_KEYWORD_FG", keyword.top()),
        ("COLOR_SELECTION_BG_ALT", selection_alt.faint().middle()),
        ("COLOR_SELECTION_BG", selection.faint().middle()),
        ("COLOR_STRING_FG_ALT", string_orange.top()),
        ("COLOR_STRING_FG", string_yellow.top()),
        ("COLOR_TYPE_FG", normal.soften().top()),
        ("COLOR_UI_LEVEL_1_BG", ui_level_1.bottom()),
        ("COLOR_UI_LEVEL_1_FG", ui_level_1.top()),
        ("COLOR_UI_LEVEL_2_BG", ui_level_2.bottom()),
        ("COLOR_UI_LEVEL_2_FG", ui_level_2.top()),
        ("COLOR_UI_LEVEL_3_BG", ui_level_3.bottom()),
        ("COLOR_UI_LEVEL_3_FG", ui_level_3.top()),
        (
            "COLOR_VISIBLE_WHITESPACE_FG",
            chromatic.rotate(0.109).faint().middle(),
        ),
        ("BG_ERR", error.tint().middle()),
        ("BG_WARN", chromatic.rotate(0.692).tint().middle()),
        ("BG_INFO", string_orange.tint().middle()),
        ("BG_HINT", chromatic.rotate(-0.055).tint().middle()),
        ("FG_ERR", err_diag.dim().top()),
        ("FG_WARN", warn_diag.dim().top()),
        ("FG_INFO", info_diag.dim().top()),
        ("FG_HINT", hint_diag.dim().top()),
    ]
}

fn print_json(palette: &[(&str, Color)]) {
    let mut hex_map = Map::new();
    let mut rgb_map = Map::new();

    for (name, color) in palette {
        let hex = OklchHex::from(*color);
        hex_map.insert(name.to_string(), Value::String(hex.to_string()));

        let rgb = OklchRgb::from(*color);
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

fn print_table(palette: &[(&str, Color)]) {
    const BLOCK: char = '\u{2588}';
    let max_name = palette.iter().map(|(n, _)| n.len()).max().unwrap_or(0);

    for (name, color) in palette {
        let rgb = OklchRgb::from(*color);
        let hex = OklchHex::from(*color);

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
