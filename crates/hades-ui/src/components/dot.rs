use gpui::{
    Animation, AnimationExt, App, ElementId, IntoElement, RenderOnce, Styled, Window, div, px,
};

use crate::theme::palette;
use crate::tokens::{Tone, motion};

/// Ponto de estado (`Dot`): 8 px redondo na cor do tom; `Neutral` é `muted-foreground` a 40%.
/// `Warn` pulsa (opacidade 1 → .35 em 2,4 s), como o ponto de bloqueado do painel.
#[derive(IntoElement)]
pub struct Dot {
    id: ElementId,
    tone: Tone,
    pulse: Option<bool>,
}

impl Dot {
    pub fn new(id: impl Into<ElementId>, tone: Tone) -> Self {
        Self {
            id: id.into(),
            tone,
            pulse: None,
        }
    }

    /// Força o pulso ligado ou desligado; por padrão só `Warn` pulsa.
    pub fn pulse(mut self, on: bool) -> Self {
        self.pulse = Some(on);
        self
    }
}

impl RenderOnce for Dot {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let color = match self.tone {
            Tone::Neutral => p.muted_fg.opacity(0.4),
            tone => p.tone(tone),
        };
        let dot = div().flex_none().size(px(8.0)).rounded_full().bg(color);
        if self.pulse.unwrap_or(self.tone == Tone::Warn) {
            dot.with_animation(self.id, Animation::new(motion::PULSE).repeat(), |el, t| {
                el.opacity(motion::pulse(t))
            })
            .into_any_element()
        } else {
            dot.into_any_element()
        }
    }
}
