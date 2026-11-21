use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};

use crate::theme::palette;
use crate::tokens::size;

/// Estado vazio: o que falta e, opcionalmente, o próximo passo. Texto curto, sem ilustração.
#[derive(IntoElement)]
pub struct EmptyState {
    title: SharedString,
    hint: Option<SharedString>,
}

impl EmptyState {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            hint: None,
        }
    }

    pub fn hint(mut self, hint: impl Into<SharedString>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

impl RenderOnce for EmptyState {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let mut body = div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(4.0))
            .p(px(24.0))
            .border_1()
            .border_color(p.border)
            .text_color(p.fg)
            .child(
                div()
                    .text_size(px(size::BODY))
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .child(self.title),
            );
        if let Some(hint) = self.hint {
            body = body.child(
                div()
                    .text_color(p.muted_fg)
                    .text_size(px(size::LABEL))
                    .child(hint),
            );
        }
        body
    }
}
