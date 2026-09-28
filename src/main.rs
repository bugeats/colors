mod backends;
mod chord;
mod helix;

use backends::ThemeRgb;
use chord::{Chord, Color};
use serde::Serialize;
use serde_json::ser::{PrettyFormatter, Serializer};
use serde_json::{Map, Value, json};

fn normal() -> Chord {
    Chord::from(Color::new(0.79, 0.035, 0.197)).set_interval([1.06, 0.02, -0.03].into())
}

fn lev3() -> Chord {
    let norm = normal();

    norm.set_sat(0.10)
        .set_lit(0.68)
        .set_hue(norm.hue() - 0.06)
        .set_interval([0.51, 0.09, 0.0].into())
}

fn ansi(rot: f64) -> Chord {
    let ansi_red = normal()
        .set_lit(lev3().get_lit())
        .set_sat(0.22)
        .set_hue(0.08)
        .set_interval([0.15, 0.08, -0.04].into());

    let base = ansi_red.rotate(rot).set_lit(lev3().get_lit());

    let warm = lev3().set_sat(0.4).set_hue(0.4);
    let w2 = base.mix(&warm);

    // base.mix(&warm).mix(&base)
    base.mix(&w2).mix(&base)
}

fn palette() -> Vec<(&'static str, Color)> {
    let norm = normal();
    let normal_alt = norm.shift_sat(0.01).scale(0.96).rotate(2.0 / 12.0);

    let level_3 = lev3();

    let level_1 = level_3
        .set_hue(norm.hue() - 0.1)
        .shift_sat(-0.038)
        .set_lit(0.58);

    let level_2 = level_3.mix(&level_1);

    let ansi_red = ansi(0.0);
    let ansi_yellow = ansi(1.0 / 6.0);
    let ansi_green = ansi(2.0 / 6.0);
    let ansi_cyan = ansi(3.0 / 6.0);
    let ansi_blue = ansi(4.0 / 6.0);
    let ansi_magenta = ansi(5.0 / 6.0);
    let ansi_white = ansi_red.desaturated();
    let ansi_black = ansi_blue.desaturated().scale(0.25).set_lit(0.33);

    let ui_mixer_warm = level_1.shift_hue(0.15).scale(0.7).shift_sat(-0.12);
    let ui_mixer_cool = ui_mixer_warm.shift_hue(0.4);
    let ansi_ui_red = ansi_red.mix(&ui_mixer_warm);
    let ansi_ui_yellow = ansi_yellow.mix(&ui_mixer_warm);
    let ansi_ui_green = ansi_green.mix(&ui_mixer_warm);
    let ansi_ui_cyan = ansi_cyan.mix(&ui_mixer_cool);
    let ansi_ui_blue = ansi_blue.mix(&ui_mixer_cool);
    let ansi_ui_magenta = ansi_magenta.mix(&ui_mixer_cool);
    let ansi_ui_white = ansi_white.mix(&ui_mixer_warm);
    let ansi_ui_black = ansi_black.mix(&ui_mixer_cool);

    let punct = level_1.set_sat(0.22).rotate(-3.0 / 24.0);
    let comment = norm.mix(&level_1);
    let whitespace = comment.set_hue(0.37).set_sat(ansi_red.sat() / 3.0);

    let chromatic = norm.set_sat(0.2);
    let keyword = chromatic.rotate(3.0 / 12.0);
    let keyword_alt = keyword.rotate(1.0 / 12.0);
    let literal = chromatic.rotate(2.0 / 24.0);
    let literal_alt = literal.rotate(-2.0 / 24.0);
    let tipe = keyword.set_sat(0.04);

    let error = comment.set_sat(0.34).rotate(-1.0 / 12.0);

    let warn = error.rotate(3.0 / 24.0);
    let info = warn.rotate(2.0 / 24.0);
    let hint = info.rotate(3.0 / 24.0);

    let cursor = ansi_magenta.rotate(9.0 / 12.0);
    let selection = level_1.rotate(3.0 / 6.0).set_sat(0.2);
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
        ("COLOR_ANSI_UI_BLACK_DIM", ansi_ui_black.bottom()),
        ("COLOR_ANSI_UI_BLACK", ansi_ui_black.middle()),
        ("COLOR_ANSI_UI_BLACK_LIGHT", ansi_ui_black.top()),
        ("COLOR_ANSI_UI_WHITE_DIM", ansi_ui_white.bottom()),
        ("COLOR_ANSI_UI_WHITE", ansi_ui_white.middle()),
        ("COLOR_ANSI_UI_WHITE_LIGHT", ansi_ui_white.top()),
        ("COLOR_ANSI_UI_RED_DIM", ansi_ui_red.bottom()),
        ("COLOR_ANSI_UI_RED", ansi_ui_red.middle()),
        ("COLOR_ANSI_UI_RED_LIGHT", ansi_ui_red.top()),
        ("COLOR_ANSI_UI_YELLOW_DIM", ansi_ui_yellow.bottom()),
        ("COLOR_ANSI_UI_YELLOW", ansi_ui_yellow.middle()),
        ("COLOR_ANSI_UI_YELLOW_LIGHT", ansi_ui_yellow.top()),
        ("COLOR_ANSI_UI_GREEN_DIM", ansi_ui_green.bottom()),
        ("COLOR_ANSI_UI_GREEN", ansi_ui_green.middle()),
        ("COLOR_ANSI_UI_GREEN_LIGHT", ansi_ui_green.top()),
        ("COLOR_ANSI_UI_CYAN_DIM", ansi_ui_cyan.bottom()),
        ("COLOR_ANSI_UI_CYAN", ansi_ui_cyan.middle()),
        ("COLOR_ANSI_UI_CYAN_LIGHT", ansi_ui_cyan.top()),
        ("COLOR_ANSI_UI_BLUE_DIM", ansi_ui_blue.bottom()),
        ("COLOR_ANSI_UI_BLUE", ansi_ui_blue.middle()),
        ("COLOR_ANSI_UI_BLUE_LIGHT", ansi_ui_blue.top()),
        ("COLOR_ANSI_UI_MAGENTA_DIM", ansi_ui_magenta.bottom()),
        ("COLOR_ANSI_UI_MAGENTA", ansi_ui_magenta.middle()),
        ("COLOR_ANSI_UI_MAGENTA_LIGHT", ansi_ui_magenta.top()),
        //
        ("COLOR_UI_LEVEL_1_BG", level_1.bottom()),
        ("COLOR_UI_LEVEL_1_FG", level_1.middle()),
        ("COLOR_UI_LEVEL_2_BG", level_2.bottom()),
        ("COLOR_UI_LEVEL_2_FG", level_2.middle()),
        ("COLOR_UI_LEVEL_3_BG", level_3.bottom()),
        ("COLOR_UI_LEVEL_3_FG", level_3.middle()),
        //
        ("COLOR_NORMAL_BG", norm.bottom()),
        ("COLOR_NORMAL_FG", norm.middle()),
        ("COLOR_NORMAL_BG_ALT", normal_alt.bottom()),
        ("COLOR_NORMAL_FG_ALT", normal_alt.middle()),
        ("COLOR_COMMENT_FG", comment.middle()),
        ("COLOR_VISIBLE_WHITESPACE_FG", whitespace.bottom()),
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
        ("COLOR_CURSOR_BG", cursor.middle()),
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
        let rgb = ThemeRgb::from(*color);
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

    let nfg = ThemeRgb::from(normal().middle());
    let nbg = ThemeRgb::from(normal().bottom());
    let base = anstyle::Style::new()
        .fg_color(Some(nfg.into()))
        .bg_color(Some(nbg.into()));

    for (name, color) in palette {
        let rgb = ThemeRgb::from(*color);
        let swatch = base.fg_color(Some(rgb.into()));

        print!("{base}{name:<max_name$}  {swatch}{BLOCK}{BLOCK}{BLOCK}{BLOCK} {rgb}    \n{base:#}");
    }
}

fn main() {
    let palette = palette();

    let args: Vec<_> = std::env::args().collect();

    if args.iter().any(|a| a == "--json") {
        print_json(&palette);
    } else if args.iter().any(|a| a == "--helix") {
        let inspect = args.iter().any(|a| a == "--inspect");
        let filter = args
            .iter()
            .position(|a| a == "--filter")
            .map(|i| args[i + 1].as_str());
        helix::print_helix(inspect, filter);
    } else {
        print_table(&palette);
    }
}
