use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};

use crate::components::card::corners;
use crate::theme::palette;
use crate::tokens::{FONT_HEADING, size};

/// Estado vazio: o que falta e, opcionalmente, o próximo passo. Texto curto, sem ilustração.
#[derive(IntoElement)]
pub struct EmptyState {
    title: SharedString,
    hint: Option<SharedString>,
    fill: bool,
}

impl EmptyState {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            hint: None,
            fill: false,
        }
    }

    /// Ocupa a altura que sobra no pai (coluna flex), com o texto centrado nela.
    pub fn fill(mut self) -> Self {
        self.fill = true;
        self
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
            .relative()
            .bg(p.card.opacity(0.96))
            .children(corners(p.line))
            .text_color(p.fg)
            .child(
                div()
                    .font_family(FONT_HEADING)
                    .text_size(px(size::SECTION))
                    .child(self.title),
            );
        if self.fill {
            body = body.flex_1().min_h_0();
        }
        if let Some(hint) = self.hint {
            body = body.child(
                div()
                    .text_color(p.muted_fg)
                    .text_size(px(size::LABEL))
                    .text_center()
                    .child(hint),
            );
        }
        body
    }
}
