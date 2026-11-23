use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};

use crate::theme::palette;
use crate::tokens::{FONT_MONO, Tone, size};

/// Um indicador da faixa: rótulo em caixa alta e valor grande em mono.
pub struct Kpi {
    label: SharedString,
    value: SharedString,
    tone: Tone,
}

impl Kpi {
    pub fn new(label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            tone: Tone::Neutral,
        }
    }

    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = tone;
        self
    }
}

/// Faixa de indicadores com fios entre as células.
#[derive(IntoElement)]
pub struct KpiStrip {
    items: Vec<Kpi>,
}

impl KpiStrip {
    pub fn new(items: impl IntoIterator<Item = Kpi>) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
}

impl RenderOnce for KpiStrip {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let last = self.items.len().saturating_sub(1);
        let mut strip = div().flex().bg(p.card.opacity(0.96));
        for (i, kpi) in self.items.into_iter().enumerate() {
            let value_color = if kpi.tone == Tone::Neutral {
                p.fg
            } else {
                p.tone(kpi.tone)
            };
            let mut cell = div().flex_1().flex().flex_col().px(px(12.0)).py(px(10.0));
            if i != last {
                cell = cell.border_r_1().border_color(p.border);
            }
            strip = strip.child(
                cell.child(
                    div()
                        .text_color(p.muted_fg)
                        .font_family(FONT_MONO)
                        .text_size(px(size::MICRO))
                        .child(kpi.label.to_uppercase()),
                )
                .child(
                    div()
                        .text_color(value_color)
                        .font_family(FONT_MONO)
                        .text_size(px(20.0))
                        .child(kpi.value),
                ),
            );
        }
        strip
    }
}
