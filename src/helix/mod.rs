mod emit;
mod node;
mod style;
mod theme;

use fancy_regex::Regex;

use crate::backends::ThemeRgb;
use emit::emit_node;
use style::Style;
use theme::theme;

pub fn print_helix(inspect: bool, filter: Option<&str>) {
    let root = theme();
    let inherited = Style::default();
    let filter = filter.map(|pat| Regex::new(pat).expect("invalid --filter regex"));

    let base = if inspect {
        let mut cascaded = inherited.merge(root.style);
        cascaded.color = (root.transform)(&cascaded.color);

        let fg = ThemeRgb::from(cascaded.color.middle());
        let bg = ThemeRgb::from(cascaded.color.bottom());

        Some(
            anstyle::Style::new()
                .fg_color(Some(fg.into()))
                .bg_color(Some(bg.into())),
        )
    } else {
        None
    };

    emit_node(&[], &root, inherited, base, filter.as_ref());
}
