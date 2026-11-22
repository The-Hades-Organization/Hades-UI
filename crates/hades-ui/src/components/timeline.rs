use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};

use crate::components::StatusBadge;
use crate::theme::palette;
use crate::tokens::{FONT_MONO, Tone, size};

struct Entry {
    time: SharedString,
    text: SharedString,
    source: Option<(SharedString, Tone)>,
}

/// Linha do tempo: coluna de hora em mono e o evento; a cor só aparece no selo da fonte.
#[derive(IntoElement)]
pub struct Timeline {
    entries: Vec<Entry>,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn entry(mut self, time: impl Into<SharedString>, text: impl Into<SharedString>) -> Self {
        self.entries.push(Entry {
            time: time.into(),
            text: text.into(),
            source: None,
        });
        self
    }

    /// Evento com selo da fonte (`tone` identifica a origem, não o estado).
    pub fn sourced(
        mut self,
        time: impl Into<SharedString>,
        text: impl Into<SharedString>,
        source: impl Into<SharedString>,
        tone: Tone,
    ) -> Self {
        self.entries.push(Entry {
            time: time.into(),
            text: text.into(),
            source: Some((source.into(), tone)),
        });
        self
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Timeline {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let mut list = div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .text_color(p.fg)
            .text_size(px(size::META));
        for e in self.entries {
            let mut row = div()
                .flex()
                .items_start()
                .gap(px(10.0))
                .child(
                    div()
                        .flex_none()
                        .w(px(76.0))
                        .text_color(p.muted_fg)
                        .font_family(FONT_MONO)
                        .child(e.time),
                )
                .child(div().flex_1().min_w_0().child(e.text));
            if let Some((label, tone)) = e.source {
                row = row.child(StatusBadge::new(label, tone));
            }
            list = list.child(row);
        }
        list
    }
}
