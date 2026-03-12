use super::style::{Modifier, Style, Underline, UnderlineStyle};
use crate::chord::Chord;

pub(super) struct Node {
    pub(super) name: &'static str,
    pub(super) style: Style,
    pub(super) children: Vec<Node>,
    pub(super) transform: Box<dyn Fn(&Chord) -> Chord>,
}

impl Node {
    pub(super) fn child(mut self, node: Node) -> Self {
        self.children.push(node);
        self
    }

    pub(super) fn transform(mut self, f: impl Fn(&Chord) -> Chord + 'static) -> Self {
        self.transform = Box::new(f);
        self
    }

    pub(super) fn underline(mut self, color: Chord, style: UnderlineStyle) -> Self {
        self.style.underline = Underline::Styled { color, style };
        self
    }

    pub(super) fn modifiers(mut self, modifiers: &'static [Modifier]) -> Self {
        self.style.modifiers = modifiers;
        self
    }
}

pub(super) fn node(name: &'static str) -> Node {
    Node {
        name,
        style: Style::default(),
        children: Vec::new(),
        transform: Box::new(Clone::clone),
    }
}
