mod oklch;

use oklch::Oklch;
use serde::Serialize;
use serde_json::ser::{PrettyFormatter, Serializer};
use serde_json::{Map, Value, json};

fn palette() -> Vec<(&'static str, Oklch)> {
    let bg = Oklch::new(0.249193, 0.004805, 67.6049);
    let fg = Oklch::new(0.794120, 0.023073, 74.0457);

    let red = Oklch::new(0.805840, 0.102584, 38.0);
    let yellow = red.with_hue(94.0564);
    let orange = red.with_hue(59.1782);
    let green = red.with_hue(152.9690);
    let blue = red.with_hue(275.8353);
    let magenta = red.with_hue(320.1364);
    let brown = Oklch::new(0.794724, 0.057422, 77.4583);

    let err_red = Oklch::new(0.802315, 0.091671, 27.5819);
    let warn_orange = Oklch::new(0.799843, 0.087862, 48.3133);
    let info_seafoam = Oklch::new(0.784713, 0.067461, 163.5503);
    let hint_cyan = Oklch::new(0.783736, 0.066794, 172.4786);

    let ansi_red = Oklch::new(fg.dim().l, 0.09, 40.0);
    let ansi_yellow = ansi_red.rotate(60.0);
    let ansi_green = ansi_red.rotate(120.0);
    let ansi_blue = ansi_red.rotate(190.0).ansi_desat();
    let ansi_cyan = ansi_blue.rotate(-33.0).ansi_desat();
    let ansi_magenta = ansi_red.rotate(-20.0);
    let ansi_white = fg;
    let ansi_black = bg;

    // Hue-rotated alternates for secondary roles
    let alt_green = Oklch::new(0.788608, 0.070668, 126.8088);
    let alt_blue = Oklch::new(0.788950, 0.076424, 229.8027);
    let alt_magenta = Oklch::new(0.796963, 0.074607, 287.1678);
    let alt_orange = Oklch::new(0.803755, 0.085227, 18.1987);

    let cursor_bg = Oklch::new(0.795777, 0.099834, 273.8793);

    let punc = fg.dim().dim().with_chroma(0.14).with_hue(33.0);

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

fn print_json(palette: &[(&str, Oklch)]) {
    let mut hex_map = Map::new();
    let mut rgb_map = Map::new();

    for (name, color) in palette {
        hex_map.insert(name.to_string(), Value::String(color.to_hex()));

        let (r, g, b) = color.to_rgb();
        let mut entry = Map::new();
        entry.insert("r".into(), Value::Number((r as i64).into()));
        entry.insert("g".into(), Value::Number((g as i64).into()));
        entry.insert("b".into(), Value::Number((b as i64).into()));
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

fn print_table(palette: &[(&str, Oklch)]) {
    const BLOCK: char = '\u{2588}';
    let max_name = palette.iter().map(|(n, _)| n.len()).max().unwrap_or(0);

    for (name, color) in palette {
        let (r, g, b) = color.to_rgb();
        let hex = color.to_hex();

        println!(
            "{:<width$}  \x1b[38;2;{};{};{}m{BLOCK}{BLOCK}{BLOCK}{BLOCK}\x1b[0m {}",
            name,
            r,
            g,
            b,
            hex,
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
