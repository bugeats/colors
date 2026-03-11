mod backends;
mod chord;

use backends::Rgb;
use chord::{Chord, Color};
use serde::Serialize;
use serde_json::ser::{PrettyFormatter, Serializer};
use serde_json::{Map, Value, json};

fn normal() -> Chord {
    Chord::from(Color::new(0.79, 0.035, 0.197)).set_interval([1.06, 0.02, -0.03])
}

fn palette() -> Vec<(&'static str, Color)> {
    let normal = normal();
    let normal_alt = normal.rotate(1.0 / 12.0);

    let spread = 0.08;

    let level_1 = normal
        .set_lit(normal.get_lit() - (spread * 4.5))
        .set_sat(0.05)
        .set_interval([spread * 3.0, 0.0, 0.0]);

    let level_2 = level_1.set_lit(level_1.get_lit() + spread);
    let level_3 = level_2.set_lit(level_2.get_lit() + spread);

    let ansi_red = normal
        .set_lit(normal.get_lit() - (level_2.get_lit() - level_1.get_lit()))
        .set_sat(0.2)
        .set_hue(0.08)
        .set_interval([0.17, 0.1, -0.12]);
    let ansi_yellow = ansi_red.rotate(1.0 / 6.0);
    let ansi_green = ansi_red.rotate(2.0 / 6.0);
    let ansi_cyan = ansi_red.rotate(3.0 / 6.0);
    let ansi_blue = ansi_red.rotate(4.0 / 6.0);
    let ansi_magenta = ansi_red.rotate(5.0 / 6.0);
    let ansi_white = ansi_red.desaturated();
    let ansi_black = ansi_yellow.faint().desaturated();

    let punct = normal
        .set_lit(normal.get_lit() - (spread * 2.0))
        .set_sat(0.2)
        .rotate(-3.0 / 24.0);

    let comment = normal
        .set_lit(level_2.get_lit())
        .set_interval(normal.interval * (spread * 4.0));

    let chromatic = normal.set_sat(0.2);
    let keyword = chromatic.rotate(3.0 / 12.0);
    let keyword_alt = keyword.rotate(1.0 / 12.0);
    let literal = chromatic.rotate(2.0 / 24.0);
    let literal_alt = literal.rotate(-2.0 / 24.0);
    let tipe = keyword.set_sat(0.04);

    let error = comment.set_sat(0.2).rotate(-1.0 / 12.0);
    let warn = error.rotate(3.0 / 24.0);
    let info = warn.rotate(2.0 / 24.0);
    let hint = info.rotate(3.0 / 24.0);

    // Syntax
    let selection = level_1.rotate(3.0 / 6.0);
    let selection_alt = selection.rotate(-2.0 / 12.0);

    vec![
        ("COLOR_ANSI_BLACK_DIM", ansi_black.bottom()),
        ("COLOR_ANSI_BLACK", ansi_black.middle()),
        ("COLOR_ANSI_BLACK_LIGHT", ansi_black.top()),
        ("COLOR_ANSI_WHITE_DIM", ansi_white.bottom()),
        ("COLOR_ANSI_WHITE", ansi_white.middle()),
        ("COLOR_ANSI_WHITE_LIGHT", ansi_white.top()),
        ("COLOR_ANSI_RED_DIM", ansi_red.bottom()),
        ("COLOR_ANSI_RED", ansi_red.middle()),
        ("COLOR_ANSI_RED_LIGHT", ansi_red.top()),
        ("COLOR_ANSI_YELLOW_DIM", ansi_yellow.bottom()),
        ("COLOR_ANSI_YELLOW", ansi_yellow.middle()),
        ("COLOR_ANSI_YELLOW_LIGHT", ansi_yellow.top()),
        ("COLOR_ANSI_GREEN_DIM", ansi_green.bottom()),
        ("COLOR_ANSI_GREEN", ansi_green.middle()),
        ("COLOR_ANSI_GREEN_LIGHT", ansi_green.top()),
        ("COLOR_ANSI_CYAN_DIM", ansi_cyan.bottom()),
        ("COLOR_ANSI_CYAN", ansi_cyan.middle()),
        ("COLOR_ANSI_CYAN_LIGHT", ansi_cyan.top()),
        ("COLOR_ANSI_BLUE_DIM", ansi_blue.bottom()),
        ("COLOR_ANSI_BLUE", ansi_blue.middle()),
        ("COLOR_ANSI_BLUE_LIGHT", ansi_blue.top()),
        ("COLOR_ANSI_MAGENTA_DIM", ansi_magenta.bottom()),
        ("COLOR_ANSI_MAGENTA", ansi_magenta.middle()),
        ("COLOR_ANSI_MAGENTA_LIGHT", ansi_magenta.top()),
        //
        ("COLOR_UI_LEVEL_1_BG", level_1.bottom()),
        ("COLOR_UI_LEVEL_1_FG", level_1.middle()),
        ("COLOR_UI_LEVEL_2_BG", level_2.bottom()),
        ("COLOR_UI_LEVEL_2_FG", level_2.middle()),
        ("COLOR_UI_LEVEL_3_BG", level_3.bottom()),
        ("COLOR_UI_LEVEL_3_FG", level_3.middle()),
        //
        ("COLOR_NORMAL_BG", normal.bottom()),
        ("COLOR_NORMAL_FG", normal.middle()),
        ("COLOR_NORMAL_BG_ALT", normal_alt.bottom()),
        ("COLOR_NORMAL_FG_ALT", normal_alt.middle()),
        ("COLOR_COMMENT_FG", comment.middle()),
        ("COLOR_VISIBLE_WHITESPACE_FG", comment.bottom()),
        ("COLOR_PUNCTUATION_FAINT_BG", punct.bottom()),
        ("COLOR_PUNCTUATION_FG", punct.middle()),
        ("COLOR_PUNCTUATION_ACTIVE_BG", punct.active().bottom()),
        ("COLOR_PUNCTUATION_ACTIVE_FG", punct.active().middle()),
        ("COLOR_KEYWORD_FG", keyword.middle()),
        ("COLOR_KEYWORD_FG_ALT", keyword_alt.middle()),
        ("COLOR_STRING_FG", literal.middle()),
        ("COLOR_STRING_FG_ALT", literal_alt.middle()),
        ("COLOR_TYPE_FG", tipe.middle()),
        ("COLOR_SELECTION_BG", selection.bottom()),
        ("COLOR_SELECTION_BG_ALT", selection_alt.bottom()),
        ("COLOR_CURSOR_BG", selection.top()),
        ("COLOR_ERROR_BG", error.bottom()),
        ("COLOR_ERROR_FG", error.middle()),
        ("BG_ERR", error.bottom()),
        ("FG_ERR", error.middle()),
        ("BG_WARN", warn.bottom()),
        ("FG_WARN", warn.middle()),
        ("BG_INFO", info.bottom()),
        ("FG_INFO", info.middle()),
        ("BG_HINT", hint.bottom()),
        ("FG_HINT", hint.middle()),
    ]
}

fn print_json(palette: &[(&str, Color)]) {
    let mut hex_map = Map::new();
    let mut rgb_map = Map::new();

    for (name, color) in palette {
        let rgb = Rgb::from(*color);
        hex_map.insert(name.to_string(), Value::String(rgb.to_string()));

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

    let nfg = Rgb::from(normal().middle());
    let sfg = format!("\x1b[38;2;{};{};{}m", nfg.r, nfg.g, nfg.b);
    let nbg = Rgb::from(normal().bottom());
    let sbg = format!("\x1b[48;2;{};{};{}m", nbg.r, nbg.g, nbg.b);

    for (name, color) in palette {
        let rgb = Rgb::from(*color);

        print!("{}{}", &sfg, &sbg);

        print!(
            "{:<width$}  \x1b[38;2;{};{};{}m{BLOCK}{BLOCK}{BLOCK}{BLOCK}\x1b[0m",
            name,
            rgb.r,
            rgb.g,
            rgb.b,
            width = max_name,
        );

        print!("{}{}", &sfg, &sbg);
        print!(" {rgb}    \n");
        print!("\x1b[0m");
    }

    // print!("\x1b[0m");
}

fn main() {
    let palette = palette();

    if std::env::args().any(|a| a == "--json") {
        print_json(&palette);
    } else {
        print_table(&palette);
    }
}
