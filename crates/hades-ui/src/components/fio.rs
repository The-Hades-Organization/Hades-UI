use gpui::{
    Animation, AnimationExt, App, ElementId, IntoElement, ParentElement, RenderOnce, Styled,
    Window, div, linear_color_stop, linear_gradient, px, relative,
};

/// Largura do brilho (35%); ele vai de −100% a 290% da própria largura (`@keyframes fiar`).
const GLOW: f32 = 0.35;

use crate::theme::palette;
use crate::tokens::motion;

/// Estado do fio (`.fio[data-s]`): o traço que liga etapas e mostra o que corre entre elas.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum FioState {
    /// 1 px `border-strong`.
    #[default]
    Parado,
    /// 1,5 px na cor do texto.
    Medido,
    /// 3 px `busy` com um brilho da cor do fundo que corre em 1,8 s.
    Fiando,
    /// 1,5 px `bad`.
    Cortado,
}

/// Fio horizontal do painel.
#[derive(IntoElement)]
pub struct Fio {
    id: ElementId,
    state: FioState,
}

impl Fio {
    pub fn new(id: impl Into<ElementId>, state: FioState) -> Self {
        Self {
            id: id.into(),
            state,
        }
    }
}

impl RenderOnce for Fio {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let (h, color) = match self.state {
            FioState::Parado => (1.0, p.border_strong),
            FioState::Medido => (1.5, p.fg.opacity(0.85)),
            FioState::Fiando => (3.0, p.busy),
            FioState::Cortado => (1.5, p.bad),
        };
        let line = div()
            .relative()
            .overflow_hidden()
            .w_full()
            .h(px(h))
            .bg(color);
        if self.state != FioState::Fiando {
            return line.into_any_element();
        }
        let shine = p.bg.opacity(0.55);
        let half = |from: f32, to: f32| {
            div().size_full().bg(linear_gradient(
                90.0,
                linear_color_stop(shine.opacity(from), 0.0),
                linear_color_stop(shine.opacity(to), 1.0),
            ))
        };
        let glow = div()
            .absolute()
            .top_0()
            .bottom_0()
            .flex()
            .w(relative(GLOW))
            .child(half(0.0, 1.0))
            .child(half(1.0, 0.0));
        line.child(
            glow.with_animation(
                self.id,
                Animation::new(motion::SPIN_THREAD)
                    .repeat()
                    .with_easing(motion::spin_thread()),
                |el, t| el.left(relative(GLOW * (t * 3.9 - 1.0))),
            ),
        )
        .into_any_element()
    }
}
