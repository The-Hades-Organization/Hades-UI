use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};

use crate::theme::palette;
use crate::tokens::{FONT_MONO, size};

#[derive(Clone, Copy)]
enum LineKind {
    Plain,
    Dim,
    Warn,
    Bad,
}

/// Bloco de terminal: fundo `--term` nos dois modos, uma linha por entrada; `dim` marca saída secundária,
/// `warn` e `bad` usam `--term-warn` e `--term-bad`.
#[derive(IntoElement)]
pub struct TermBlock {
    lines: Vec<(SharedString, LineKind)>,
}

impl TermBlock {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    pub fn line(mut self, text: impl Into<SharedString>) -> Self {
        self.lines.push((text.into(), LineKind::Plain));
        self
    }

    pub fn dim(mut self, text: impl Into<SharedString>) -> Self {
        self.lines.push((text.into(), LineKind::Dim));
        self
    }

    pub fn warn(mut self, text: impl Into<SharedString>) -> Self {
        self.lines.push((text.into(), LineKind::Warn));
        self
    }

    pub fn bad(mut self, text: impl Into<SharedString>) -> Self {
        self.lines.push((text.into(), LineKind::Bad));
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
        for (text, kind) in self.lines {
            let color = match kind {
                LineKind::Plain => p.term_fg,
                LineKind::Dim => p.term_mute,
                LineKind::Warn => p.term_warn,
                LineKind::Bad => p.term_bad,
            };
            block = block.child(div().text_color(color).child(text));
        }
        block
    }
}
