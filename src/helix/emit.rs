use fancy_regex::Regex;

use super::node::Node;
use super::style::Style;
use crate::backends::ThemeRgb;

fn scope_path(ancestors: &[(&str, Style)], name: &str) -> String {
    let mut parts: Vec<&str> = ancestors.iter().map(|(n, _)| *n).collect();
    parts.push(name);
    parts.join(".")
}

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

fn emit_scope(
    ancestors: &[(&str, Style)],
    name: &str,
    style: &Style,
    inspect: Option<anstyle::Style>,
) {
    let value = toml::Value::try_from(style).unwrap();
    let rhs = fmt_inline(&value);

    match inspect {
        Some(base) => {
            let fg = |s: &Style| {
                let middle = ThemeRgb::from(s.color.middle());
                let bottom = ThemeRgb::from(s.color.bottom());
                base.fg_color(Some(middle.into()))
                    .bg_color(Some(bottom.into()))
            };

            let name_ansi = fg(style);

            if ancestors.is_empty() {
                print!("{name_ansi}{name}{name_ansi:#}");
            } else {
                let first_ansi = fg(&ancestors[0].1);
                print!("{first_ansi}\"{}{first_ansi:#}", ancestors[0].0);

                for (seg_name, seg_style) in &ancestors[1..] {
                    let seg_ansi = fg(seg_style);
                    print!("{seg_ansi}.{seg_name}{seg_ansi:#}");
                }

                print!("{name_ansi}.{name}\"{name_ansi:#}");
            }

            println!(" {base}= {rhs}{base:#}");
        }

        None => {
            let path = scope_path(ancestors, name);

            if path.contains('.') {
                println!("\"{path}\" = {rhs}");
            } else {
                println!("{path} = {rhs}");
            }
        }
    }
}

pub(super) fn emit_node(
    ancestors: &[(&str, Style)],
    node: &Node,
    inherited: Style,
    inspect: Option<anstyle::Style>,
    filter: Option<&Regex>,
) {
    let mut cascaded = inherited.merge(node.style);
    cascaded.color = (node.transform)(&cascaded.color);

    let mut child_ancestors = ancestors.to_vec();

    if !node.name.is_empty() {
        let path = scope_path(ancestors, node.name);

        if filter.map_or(true, |re| re.is_match(&path).unwrap_or(false)) {
            emit_scope(ancestors, node.name, &cascaded, inspect);
        }

        child_ancestors.push((node.name, cascaded));
    }

    for child in &node.children {
        emit_node(&child_ancestors, child, cascaded, inspect, filter);
    }
}
