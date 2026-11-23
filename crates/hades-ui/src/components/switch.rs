use std::rc::Rc;

use gpui::{
    Animation, AnimationExt, App, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, px,
};

use super::button::focus_ring;
use crate::theme::palette;
use crate::tokens::{Palette, motion};

type ToggleHandler = Rc<dyn Fn(bool, &mut Window, &mut App)>;

/// Interruptor (`Switch`): 44×22, desligado em `control-off` com linha de base `line` e botão `muted-foreground`
/// à esquerda; ligado em `primary` com linha de base escurecida e botão `primary-foreground` à direita.
/// O botão desliza em 200 ms. O estado é do chamador.
#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    checked: bool,
    disabled: bool,
    label: Option<SharedString>,
    on_change: Option<ToggleHandler>,
}

impl Switch {
    pub fn new(id: impl Into<ElementId>, checked: bool) -> Self {
        Self {
            id: id.into(),
            checked,
            disabled: false,
            label: None,
            on_change: None,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Rótulo à direita do interruptor.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Recebe o novo valor.
    pub fn on_change(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(f));
        self
    }
}

/// Linha de base do ligado: `primary` 60% sobre preto.
fn dark_edge(primary: Hsla) -> Hsla {
    Hsla::black().blend(primary.opacity(0.6))
}

impl RenderOnce for Switch {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let (track, edge, knob, x) = if self.checked {
            (p.primary, dark_edge(p.primary), p.primary_fg, 27.0)
        } else {
            (p.control_off, p.line, p.muted_fg, 5.0)
        };
        let from = if self.checked { 5.0 } else { 27.0 };
        let knob = div()
            .absolute()
            .top(px(4.0))
            .size(px(12.0))
            .bg(knob)
            .with_animation(
                (self.id.clone(), if self.checked { "on" } else { "off" }),
                Animation::new(std::time::Duration::from_millis(200)).with_easing(motion::ease()),
                move |el, t| el.left(px(from + (x - from) * t)),
            );
        let mut track = div()
            .id(self.id.clone())
            .relative()
            .flex_none()
            .w(px(44.0))
            .h(px(22.0))
            .bg(track)
            .shadow(vec![Palette::baseline(edge, 2.0)])
            .child(knob);
        if self.disabled {
            track = track.opacity(0.55).cursor_not_allowed();
        } else {
            let ring = focus_ring(&p, p.bg);
            track = track
                .cursor_pointer()
                .tab_stop(true)
                .focus_visible(move |s| s.shadow(ring.clone()));
            if let Some(f) = self.on_change {
                let checked = self.checked;
                track = track.on_click(move |_, window, cx| f(!checked, window, cx));
            }
        }
        match self.label {
            Some(label) => div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .text_size(px(crate::tokens::size::LABEL))
                .child(track)
                .child(label)
                .into_any_element(),
            None => track.into_any_element(),
        }
    }
}
