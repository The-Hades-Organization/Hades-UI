use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};

use crate::theme::palette;
use crate::tokens::{FONT_MONO, size};

/// Bloco de terminal: fundo `--term` nos dois modos, uma linha por entrada; `dim` marca saída secundária.
#[derive(IntoElement)]
pub struct TermBlock {
    lines: Vec<(SharedString, bool)>,
}

impl TermBlock {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    pub fn line(mut self, text: impl Into<SharedString>) -> Self {
        self.lines.push((text.into(), false));
        self
    }

    pub fn dim(mut self, text: impl Into<SharedString>) -> Self {
        self.lines.push((text.into(), true));
        self
    }
}

impl Default for TermBlock {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for TermBlock {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let mut block = div()
            .flex()
            .flex_col()
            .px(px(12.0))
            .py(px(10.0))
            .overflow_hidden()
            .bg(p.term)
            .text_color(p.term_fg)
            .font_family(FONT_MONO)
            .text_size(px(size::META));
        for (text, dim) in self.lines {
            let mut row = div().child(text);
            if dim {
                row = row.text_color(p.term_mute);
            }
            block = block.child(row);
        }
        block
    }
}
