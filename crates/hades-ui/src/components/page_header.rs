use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};

use crate::theme::palette;
use crate::tokens::size;

/// Cabeçalho de página: título à esquerda e as ações (botões, segmentado) à direita.
#[derive(IntoElement)]
pub struct PageHeader {
    title: SharedString,
    actions: Vec<AnyElement>,
}

impl PageHeader {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            actions: Vec::new(),
        }
    }

    pub fn action(mut self, element: impl IntoElement) -> Self {
        self.actions.push(element.into_any_element());
        self
    }
}

impl RenderOnce for PageHeader {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        div()
            .flex()
            .items_center()
            .gap(px(10.0))
            .text_color(p.fg)
            .child(
                div()
                    .flex_1()
                    .text_size(px(size::PROSE))
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .child(self.title),
            )
            .children(self.actions)
    }
}
