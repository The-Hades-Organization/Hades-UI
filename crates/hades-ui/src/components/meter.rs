use gpui::{
    App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px, relative,
};

use crate::theme::palette;
use crate::tokens::{FONT_MONO, Tone, size};

/// Barra de progresso do painel (`Progress`): 4 px, trilho `--border`; `Neutral` pinta com a cor de texto.
/// Com `label` ou `value`, ganha a linha de cima do rodapé "Memória do Hades": rótulo à esquerda em
/// `muted-foreground` e valor em mono à direita, e a barra fica 8 px abaixo.
#[derive(IntoElement)]
pub struct Meter {
    fraction: f32,
    over: bool,
    real: f32,
    tone: Tone,
    label: Option<SharedString>,
    value: Option<SharedString>,
}

impl Meter {
    /// A barra vai de 0 a 1. Acima de 1 (estouro) ela fica cheia na cor de perigo, com um corte na ponta, e,
    /// sem `value`, o rótulo mostra o percentual real (ex.: "130%").
    pub fn new(fraction: f32) -> Self {
        let fraction = if fraction.is_nan() {
            0.0
        } else {
            fraction.max(0.0)
        };
        Self {
            fraction: fraction.min(1.0),
            over: fraction > 1.0,
            real: fraction,
            tone: Tone::Neutral,
            label: None,
            value: None,
        }
    }

    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = tone;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Valor lido, em mono (ex.: "2,1 GiB de 4 GiB").
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }
}

impl RenderOnce for Meter {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let fill = if self.over {
            p.bad
        } else if self.tone == Tone::Neutral {
            p.fg
        } else {
            p.tone(self.tone)
        };
        let bar = div()
            .h(px(4.0))
            .w_full()
            .bg(p.border)
            .relative()
            .child(div().h_full().w(relative(self.fraction)).bg(fill));
        let bar = if self.over {
            bar.child(
                div()
                    .absolute()
                    .top_0()
                    .right_0()
                    .h_full()
                    .w(px(2.0))
                    .bg(p.fg),
            )
        } else {
            bar
        };
        let value = match self.value {
            Some(v) => Some(v),
            None if self.over => Some(format!("{:.0}%", self.real * 100.0).into()),
            None => None,
        };
        if self.label.is_none() && value.is_none() {
            return bar.into_any_element();
        }
        let mut head = div()
            .flex()
            .flex_wrap()
            .items_baseline()
            .justify_between()
            .gap_x(px(8.0))
            .text_size(px(size::META));
        head = head.child(
            div()
                .text_color(p.muted_fg)
                .child(self.label.unwrap_or_default()),
        );
        if let Some(value) = value {
            let color = if self.over { p.bad } else { p.fg };
            head = head.child(div().font_family(FONT_MONO).text_color(color).child(value));
        }
        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .w_full()
            .child(head)
            .child(bar)
            .into_any_element()
    }
}
