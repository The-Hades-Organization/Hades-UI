use gpui::{App, IntoElement, ParentElement, RenderOnce, Styled, Window, div, px, relative};

use crate::theme::palette;
use crate::tokens::Tone;

/// Medidor de 6 px; `Neutral` pinta com a cor de texto.
#[derive(IntoElement)]
pub struct Meter {
    fraction: f32,
    tone: Tone,
}

impl Meter {
    /// `fraction` fica entre 0 e 1.
    pub fn new(fraction: f32) -> Self {
        Self {
            fraction: fraction.clamp(0.0, 1.0),
            tone: Tone::Neutral,
        }
    }

    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = tone;
        self
    }
}

impl RenderOnce for Meter {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let fill = if self.tone == Tone::Neutral {
            p.fg
        } else {
            p.tone(self.tone)
        };
        div()
            .h(px(6.0))
            .w_full()
            .bg(p.muted)
            .child(div().h_full().w(relative(self.fraction)).bg(fill))
    }
}
