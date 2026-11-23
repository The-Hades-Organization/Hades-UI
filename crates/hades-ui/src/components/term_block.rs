use gpui::{
    App, ElementId, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};

use crate::components::CopyButton;
use crate::theme::palette;
use crate::tokens::{FONT_MONO, size};

#[derive(Clone, Copy)]
enum LineKind {
    Plain,
    Dim,
    Warn,
    Bad,
}

/// Bloco de código ou de saída, uma linha por entrada; `dim` marca saída secundária, `warn` e `bad` pintam a linha.
///
/// O padrão é o bloco de comando do painel (`bg-muted`, rosado no claro): texto `foreground` em mono 12, linhas
/// longas quebram em vez de cortar. `copy` embute o botão de copiar no canto superior direito. `terminal()` volta ao
/// fundo `--term`, escuro nos dois modos, para log e saída de processo.
#[derive(IntoElement)]
pub struct TermBlock {
    lines: Vec<(SharedString, LineKind)>,
    copy: Option<(ElementId, SharedString)>,
    terminal: bool,
}

impl TermBlock {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            copy: None,
            terminal: false,
        }
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

    /// Botão de copiar embutido no canto, que copia `text` (em geral o comando, sem a saída).
    pub fn copy(mut self, id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        self.copy = Some((id.into(), text.into()));
        self
    }

    /// Fundo `--term`, escuro nos dois modos, como o `<pre>` de log do painel.
    pub fn terminal(mut self) -> Self {
        self.terminal = true;
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
        let (bg, plain, dim, warn, bad) = if self.terminal {
            (p.term, p.term_fg, p.term_mute, p.term_warn, p.term_bad)
        } else {
            (p.muted, p.fg, p.muted_fg, p.warn, p.bad)
        };
        let mut text = div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .py(px(4.0))
            .line_height(px(20.0));
        for (line, kind) in self.lines {
            let color = match kind {
                LineKind::Plain => plain,
                LineKind::Dim => dim,
                LineKind::Warn => warn,
                LineKind::Bad => bad,
            };
            text = text.child(div().w_full().min_w_0().text_color(color).child(line));
        }
        div()
            .flex()
            .items_start()
            .gap(px(4.0))
            .w_full()
            .min_w_0()
            .py(px(4.0))
            .pl(px(12.0))
            .pr(px(4.0))
            .bg(bg)
            .text_color(plain)
            .font_family(FONT_MONO)
            .text_size(px(size::META))
            .child(text)
            .children(
                self.copy
                    .map(|(id, value)| CopyButton::new(id, value).icon_only()),
            )
    }
}
