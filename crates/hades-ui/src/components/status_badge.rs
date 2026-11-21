use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};

use crate::theme::palette;
use crate::tokens::{FONT_MONO, Tone, size};

/// Selo de estado: texto mono com fio da própria cor. A cor é função (estado), nunca enfeite.
#[derive(IntoElement)]
pub struct StatusBadge {
    label: SharedString,
    tone: Tone,
}

impl StatusBadge {
    pub fn new(label: impl Into<SharedString>, tone: Tone) -> Self {
        Self {
            label: label.into(),
            tone,
        }
    }
}

impl RenderOnce for StatusBadge {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = palette(cx).tone(self.tone);
        div()
            .flex_none()
            .px(px(6.0))
            .py(px(4.0))
            .border_1()
            .border_color(color)
            .text_color(color)
            .font_family(FONT_MONO)
            .text_size(px(size::MICRO))
            .child(self.label)
    }
}
