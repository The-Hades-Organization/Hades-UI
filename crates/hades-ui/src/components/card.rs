use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};

use crate::theme::palette;
use crate::tokens::{FONT_MONO, size};

/// Cartão com fio e título em caixa alta mono; sem sombra.
#[derive(IntoElement)]
pub struct Card {
    title: Option<SharedString>,
    children: Vec<AnyElement>,
}

impl Card {
    pub fn new() -> Self {
        Self {
            title: None,
            children: Vec::new(),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Card {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Card {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let mut card = div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .px(px(12.0))
            .py(px(10.0))
            .bg(p.card)
            .text_color(p.fg)
            .border_1()
            .border_color(p.border);
        if let Some(title) = self.title {
            card = card.child(
                div()
                    .text_color(p.muted_fg)
                    .font_family(FONT_MONO)
                    .text_size(px(size::MICRO))
                    .child(title.to_uppercase()),
            );
        }
        card.children(self.children)
    }
}
