use gpui::{
    App, ElementId, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window, div, px,
};

use super::Spinner;
use crate::icons::{Icon, IconName};
use crate::theme::palette;
use crate::tokens::{Tone, size};

/// Selo de estado (`BADGE_V`): cheio na cor do estado, texto na cor do fundo, 12 px peso 600.
/// `Busy` leva o giro. A cor é função (estado), nunca enfeite.
#[derive(IntoElement)]
pub struct StatusBadge {
    label: SharedString,
    tone: Tone,
    icon: Option<IconName>,
}

impl StatusBadge {
    pub fn new(label: impl Into<SharedString>, tone: Tone) -> Self {
        Self {
            label: label.into(),
            tone,
            icon: None,
        }
    }

    /// Ícone de 12 px antes do rótulo, na cor do texto (ex.: o visto de "Escolhida").
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }
}

impl RenderOnce for StatusBadge {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let color = p.tone(self.tone);
        let mut badge = div()
            .flex()
            .flex_none()
            .items_center()
            .gap(px(4.0))
            .px(px(8.0))
            .py(px(2.0))
            .bg(color)
            .text_color(p.bg)
            .text_size(px(size::META))
            .line_height(px(16.0))
            .font_weight(FontWeight::SEMIBOLD);
        if self.tone == Tone::Busy {
            let id = ElementId::Name(format!("selo-{}", self.label).into());
            badge = badge.child(Spinner::new(id).size(12.0).color(p.bg));
        }
        if let Some(icon) = self.icon {
            badge = badge.child(Icon::new(icon).size(px(12.0)).color(p.bg));
        }
        badge.child(self.label)
    }
}
