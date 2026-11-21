use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};

use crate::theme::palette;
use crate::tokens::{FONT_MONO, TOP_BAR_HEIGHT, size};

/// Barra de topo escura da janela: título centralizado e rótulo do app à direita.
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
            .px(px(12.0))
            .h(px(TOP_BAR_HEIGHT))
            .flex_none()
            .bg(p.rail)
            .text_color(p.rail_mute)
            .border_b_1()
            .border_color(p.rail_line)
            .font_family(FONT_MONO)
            .text_size(px(size::META))
            .child(div().flex_1().flex().justify_center().child(self.title))
            .child(self.trailing)
    }
}
