use gpui::{
    Animation, AnimationExt, App, ElementId, IntoElement, ParentElement, RenderOnce, Styled,
    Window, div, linear_color_stop, linear_gradient, px, relative,
};

use crate::theme::palette;
use crate::tokens::motion;

/// Bloco de carregamento (`Skeleton`): fundo `muted` com uma faixa clara que atravessa em 1,6 s, linear e sem fim.
#[derive(IntoElement)]
pub struct Skeleton {
    id: ElementId,
    width: Option<f32>,
    height: f32,
}

impl Skeleton {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            width: None,
            height: 16.0,
        }
    }

    /// Largura fixa; sem ela ocupa a linha inteira.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}

impl RenderOnce for Skeleton {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let shine = p.bg.blend(p.muted.opacity(0.5));
        let mut block = div()
            .relative()
            .overflow_hidden()
            .h(px(self.height))
            .bg(p.muted);
        block = match self.width {
            Some(w) => block.w(px(w)),
            None => block.w_full(),
        };
        let band = div()
            .absolute()
            .top_0()
            .bottom_0()
            .w(relative(0.5))
            .flex()
            .child(div().size_full().bg(linear_gradient(
                90.0,
                linear_color_stop(shine.opacity(0.0), 0.0),
                linear_color_stop(shine, 1.0),
            )))
            .child(div().size_full().bg(linear_gradient(
                90.0,
                linear_color_stop(shine, 0.0),
                linear_color_stop(shine.opacity(0.0), 1.0),
            )));
        block.child(band.with_animation(
            self.id,
            Animation::new(motion::SHIMMER).repeat(),
            |el, t| el.left(relative(t * 1.5 - 0.5)),
        ))
    }
}
