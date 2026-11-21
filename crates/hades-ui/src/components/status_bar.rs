use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};

use crate::theme::palette;
use crate::tokens::{FONT_MONO, STATUS_BAR_HEIGHT, size};

/// Barra de estado no pé da janela: texto à esquerda e à direita, em mono.
#[derive(IntoElement)]
pub struct StatusBar {
    left: SharedString,
    right: SharedString,
}

impl StatusBar {
    pub fn new(left: impl Into<SharedString>, right: impl Into<SharedString>) -> Self {
        Self {
            left: left.into(),
            right: right.into(),
        }
    }
}

impl RenderOnce for StatusBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        div()
            .flex()
            .items_center()
            .justify_between()
            .px(px(12.0))
            .h(px(STATUS_BAR_HEIGHT))
            .flex_none()
            .bg(p.bg)
            .text_color(p.muted_fg)
            .border_t_1()
            .border_color(p.border)
            .font_family(FONT_MONO)
            .text_size(px(size::MICRO))
            .child(self.left)
            .child(self.right)
    }
}
