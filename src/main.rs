mod oklch;

use oklch::Oklch;
use serde::Serialize;
use serde_json::ser::{PrettyFormatter, Serializer};
use serde_json::{json, Map, Value};

fn palette() -> Vec<(&'static str, Oklch)> {
    let bg = Oklch::new(0.249193, 0.004805, 67.6049);
    let fg = Oklch::new(0.794120, 0.023073, 74.0457);
    let red = Oklch::new(0.805840, 0.102584, 21.7726);
    let yellow = Oklch::new(0.792200, 0.075818, 94.0564);
    let orange = Oklch::new(0.798851, 0.076616, 59.1782);
    let green = Oklch::new(0.783678, 0.074871, 152.9690);
    let blue = Oklch::new(0.795322, 0.071877, 275.8353);
    let magenta = Oklch::new(0.802758, 0.090264, 320.1364);
    let brown = Oklch::new(0.794724, 0.057422, 77.4583);

    let err_red = Oklch::new(0.802315, 0.091671, 27.5819);
    let warn_orange = Oklch::new(0.799843, 0.087862, 48.3133);
    let info_seafoam = Oklch::new(0.784713, 0.067461, 163.5503);
    let hint_cyan = Oklch::new(0.783736, 0.066794, 172.4786);

    // Desaturated, slightly hue-shifted variants for terminal ANSI colors
    let ansi_black = Oklch::new(0.379031, 0.005196, 91.5253);
    let ansi_red = Oklch::new(0.768060, 0.083875, 31.1293);
    let ansi_green = Oklch::new(0.751049, 0.057458, 160.1909);
    let ansi_yellow = Oklch::new(0.757027, 0.062501, 101.5023);
    let ansi_blue = Oklch::new(0.762743, 0.059775, 285.9812);
    let ansi_cyan = Oklch::new(0.749799, 0.070587, 198.0765);
    let ansi_magenta = Oklch::new(0.766598, 0.074011, 327.0291);
    let ansi_white = Oklch::new(0.758464, 0.010864, 81.7919);

    // High-lightness, low-chroma variants for ANSI bright colors
    let bright_black = Oklch::new(0.896388, 0.044675, 83.9000);
    let bright_red = Oklch::new(0.901970, 0.046140, 29.6410);
    let bright_green = Oklch::new(0.889740, 0.057892, 159.0431);
    let bright_yellow = Oklch::new(0.895544, 0.046603, 100.5593);
    let bright_blue = Oklch::new(0.899768, 0.046324, 286.7777);
    let bright_cyan = Oklch::new(0.889396, 0.053221, 197.7065);
    let bright_magenta = Oklch::new(0.903967, 0.055747, 326.9639);

    // Hue-rotated alternates for secondary roles
    let alt_green = Oklch::new(0.788608, 0.070668, 126.8088);
    let alt_blue = Oklch::new(0.788950, 0.076424, 229.8027);
    let alt_magenta = Oklch::new(0.796963, 0.074607, 287.1678);
    let alt_orange = Oklch::new(0.803755, 0.085227, 18.1987);

    let cursor_bg = Oklch::new(0.795777, 0.099834, 273.8793);

    vec![
        ("COLOR_ANSI_BLACK", ansi_black),
        ("COLOR_ANSI_BLACK_DIM", ansi_black.dim()),
        ("COLOR_ANSI_BLACK_LIGHT", bright_black),
        ("COLOR_ANSI_BLUE", ansi_blue.dim()),
        ("COLOR_ANSI_BLUE_DIM", ansi_blue.dim().dim()),
        ("COLOR_ANSI_BLUE_LIGHT", bright_blue),
        ("COLOR_ANSI_CYAN", ansi_cyan),
        ("COLOR_ANSI_CYAN_DIM", ansi_cyan.dim()),
        ("COLOR_ANSI_CYAN_LIGHT", bright_cyan),
        ("COLOR_ANSI_GREEN", ansi_green),
        ("COLOR_ANSI_GREEN_DIM", ansi_green.dim()),
        ("COLOR_ANSI_GREEN_LIGHT", bright_green),
        ("COLOR_ANSI_MAGENTA", ansi_magenta),
        ("COLOR_ANSI_MAGENTA_DIM", ansi_magenta.dim()),
        ("COLOR_ANSI_MAGENTA_LIGHT", bright_magenta),
        ("COLOR_ANSI_RED", ansi_red),
        ("COLOR_ANSI_RED_DIM", ansi_red.dim()),
        ("COLOR_ANSI_RED_LIGHT", bright_red),
        ("COLOR_ANSI_WHITE", ansi_white),
        ("COLOR_ANSI_WHITE_DIM", ansi_white.dim()),
        ("COLOR_ANSI_WHITE_LIGHT", bright_black),
        ("COLOR_ANSI_YELLOW", ansi_yellow),
        ("COLOR_ANSI_YELLOW_DIM", ansi_yellow.dim()),
        ("COLOR_ANSI_YELLOW_LIGHT", bright_yellow),
        ("COLOR_COMMENT_FG", fg.dim()),
        ("COLOR_CURSOR_BG", cursor_bg),
        ("COLOR_ERROR_BG", magenta.veryfaint(bg.l)),
        ("COLOR_ERROR_FG", magenta),
        ("COLOR_KEYWORD_FG_ALT", alt_green),
        ("COLOR_KEYWORD_FG", green),
        ("COLOR_NORMAL_BG", bg),
        ("COLOR_NORMAL_BG_ALT", bg.veryfaint(bg.l)),
        ("COLOR_NORMAL_FG_ALT", fg.fgdim(fg.l)),
        ("COLOR_NORMAL_FG", fg),
        ("COLOR_PUNCTUATION_ACTIVE_BG", bg.interp(brown, 4.0 / 12.0)),
        ("COLOR_PUNCTUATION_ACTIVE_FG", red),
        ("COLOR_PUNCTUATION_FAINT_BG", red.faint(bg.l)),
        ("COLOR_PUNCTUATION_FG", red.dim()),
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
    let max_name = palette.iter().map(|(n, _)| n.len()).max().unwrap_or(0);

    for (name, color) in palette {
        let (r, g, b) = color.to_rgb();
        let hex = color.to_hex();

        println!(
            "{:<width$}  \x1b[38;2;{};{};{}m\u{2588}\u{2588}\x1b[0m {}",
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
