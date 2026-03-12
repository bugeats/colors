use serde::ser::{SerializeMap, Serializer};
use serde::Serialize;

use crate::backends::ThemeRgb;
use crate::chord::Chord;

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub(super) enum Modifier {
    Bold,
    Dim,
    Italic,
    Underlined,
    SlowBlink,
    RapidBlink,
    Reversed,
    Hidden,
    CrossedOut,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(dead_code)]
pub(super) enum UnderlineStyle {
    Line,
    Curl,
    Dashed,
    Dotted,
    DoubleLine,
}

#[derive(Clone, Copy, Default)]
pub(super) enum Underline {
    #[default]
    None,
    Styled {
        color: Chord,
        style: UnderlineStyle,
    },
}

impl Underline {
    fn is_none(&self) -> bool {
        matches!(self, Underline::None)
    }

    fn merge(self, child: Self) -> Self {
        if child.is_none() {
            self
        } else {
            child
        }
    }
}

impl Serialize for Underline {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Underline::None => serializer.serialize_none(),

            Underline::Styled { color, style } => {
                let mut map = serializer.serialize_map(None)?;

                if !color.is_default() {
                    map.serialize_entry("color", &ThemeRgb::from(color.middle()).to_string())?;
                }

                map.serialize_entry("style", style)?;
                map.end()
            }
        }
    }
}

#[derive(Clone, Copy, Default)]
pub(super) struct Style {
    pub(super) color: Chord,
    pub(super) underline: Underline,
    pub(super) modifiers: &'static [Modifier],
}

impl Style {
    pub(super) fn merge(self, child: Self) -> Self {
        Self {
            color: if child.color.is_default() {
                self.color
            } else {
                child.color
            },
            underline: self.underline.merge(child.underline),
            modifiers: if child.modifiers.is_empty() {
                self.modifiers
            } else {
                child.modifiers
            },
        }
    }
}

impl Serialize for Style {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let fg = ThemeRgb::from(self.color.middle()).to_string();
        let bg = ThemeRgb::from(self.color.bottom()).to_string();

        let field_count =
            2 + (!self.underline.is_none()) as usize + (!self.modifiers.is_empty()) as usize;

        let mut map = serializer.serialize_map(Some(field_count))?;
        map.serialize_entry("fg", fg.as_str())?;
        map.serialize_entry("bg", bg.as_str())?;

        if !self.underline.is_none() {
            map.serialize_entry("underline", &self.underline)?;
        }

        if !self.modifiers.is_empty() {
            map.serialize_entry("modifiers", self.modifiers)?;
        }

        map.end()
    }
}
