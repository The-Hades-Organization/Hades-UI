use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};

use crate::theme::palette;
use crate::tokens::{FONT_MONO, Palette, TOP_BAR_HEIGHT, size};

/// Barra de topo da janela (`header` do painel): h 48, fundo da página a 90%, linha de base de 2 px `--primary`.
#[derive(IntoElement)]
pub struct TopBar {
    title: SharedString,
    trailing: SharedString,
}

impl TopBar {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            trailing: "gpui".into(),
        }
    }

    pub fn trailing(mut self, text: impl Into<SharedString>) -> Self {
        self.trailing = text.into();
        self
    }
}

impl RenderOnce for TopBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        div()
            .flex()
            .items_center()
            .gap(px(12.0))
            .px(px(16.0))
            .h(px(TOP_BAR_HEIGHT))
            .flex_none()
            .bg(p.bg.opacity(0.9))
            .text_color(p.muted_fg)
            .shadow(vec![Palette::baseline(p.primary, 2.0)])
            .font_family(FONT_MONO)
            .text_size(px(size::META))
            .child(div().flex_1().flex().justify_center().child(self.title))
            .child(self.trailing)
    }
}
