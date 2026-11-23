use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};

use crate::theme::palette;
use crate::tokens::{FONT_MONO, Palette, size};

/// Tecla (`Kbd`): fundo `muted`, 11 px mono, linha de base de 1 px `line`, altura 20.
#[derive(IntoElement)]
pub struct Kbd {
    keys: SharedString,
}

impl Kbd {
    pub fn new(keys: impl Into<SharedString>) -> Self {
        Self { keys: keys.into() }
    }
}

impl RenderOnce for Kbd {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        div()
            .flex()
            .flex_none()
            .items_center()
            .h(px(20.0))
            .px(px(6.0))
            .bg(p.muted)
            .text_color(p.muted_fg)
            .font_family(FONT_MONO)
            .text_size(px(size::MICRO))
            .shadow(vec![Palette::baseline(p.line, 1.0)])
            .child(self.keys)
    }
}
