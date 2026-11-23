use std::rc::Rc;

use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Window, div, px,
};

use crate::icons::{Icon, IconName};
use crate::theme::palette;
use crate::tokens::{FONT_MONO, size};
use crate::tokens::{Palette, control};

type ToggleHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// Filtro em chips (o `filterBtn` do painel): fundo `secondary`; ligado leva o visto e a linha de base `--primary`.
/// Cada chip liga ou desliga. O estado é do chamador: `active` e `on_toggle`.
#[derive(IntoElement)]
pub struct FilterBar {
    label: Option<SharedString>,
    chips: Vec<(SharedString, bool)>,
    on_toggle: Option<ToggleHandler>,
}

impl FilterBar {
    pub fn new() -> Self {
        Self {
            label: None,
            chips: Vec::new(),
            on_toggle: None,
        }
    }

    /// Rótulo do grupo, em caixa alta antes dos chips (ex.: "Papel").
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn chip(mut self, label: impl Into<SharedString>, active: bool) -> Self {
        self.chips.push((label.into(), active));
        self
    }

    pub fn on_toggle(mut self, f: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_toggle = Some(Rc::new(f));
        self
    }
}

impl Default for FilterBar {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for FilterBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let mut bar = div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(px(8.0))
            .text_size(px(size::META));
        if let Some(label) = self.label {
            bar = bar.child(
                div()
                    .text_color(p.muted_fg)
                    .font_family(FONT_MONO)
                    .text_size(px(size::MICRO))
                    .child(label.to_uppercase()),
            );
        }
        for (i, (label, active)) in self.chips.into_iter().enumerate() {
            let hover = p.secondary_hover;
            let mut chip = div()
                .id(ElementId::NamedInteger("chip".into(), i as u64))
                .flex()
                .items_center()
                .gap(px(6.0))
                .h(px(control::XS))
                .px(px(10.0))
                .bg(p.muted)
                .text_color(p.fg)
                .text_size(px(size::LABEL))
                .font_weight(gpui::FontWeight::MEDIUM)
                .hover(move |s| s.bg(hover));
            chip = if active {
                chip.shadow(vec![Palette::baseline(p.primary, 2.0)])
                    .child(Icon::new(IconName::Check).size(px(14.0)))
            } else {
                chip
            };
            chip = chip.child(label);
            if let Some(f) = self.on_toggle.clone() {
                chip = chip
                    .cursor_pointer()
                    .on_click(move |_, window, cx| f(i, window, cx));
            }
            bar = bar.child(chip);
        }
        bar
    }
}
