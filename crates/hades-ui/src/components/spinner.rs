use std::f32::consts::{FRAC_PI_2, PI};

use gpui::{
    Animation, AnimationExt, App, Bounds, ElementId, Hsla, IntoElement, ParentElement, PathBuilder,
    Pixels, RenderOnce, Styled, Window, canvas, div, point, px,
};

use crate::theme::palette;
use crate::tokens::motion;

/// Giro de carregamento (`I.LoaderCircle animate-spin`): arco de 3/4 com traço de 2 px e ponta seca, uma volta por segundo.
/// Com movimento reduzido, o gpui mostra o arco parado.
#[derive(IntoElement)]
pub struct Spinner {
    id: ElementId,
    size: f32,
    color: Option<Hsla>,
}

impl Spinner {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            size: 16.0,
            color: None,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Cor do traço; por padrão a do texto.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

/// Arco de 3/4 de volta começando em `turn` (0..1) voltas a partir do topo.
fn paint_arc(bounds: Bounds<Pixels>, turn: f32, stroke: f32, color: Hsla, window: &mut Window) {
    let side = f32::from(bounds.size.width.min(bounds.size.height));
    let r = (side - stroke) / 2.0;
    let c = bounds.center();
    let at = |a: f32| point(c.x + px(r * a.cos()), c.y + px(r * a.sin()));
    let start = turn * 2.0 * PI - FRAC_PI_2;
    let mut path = PathBuilder::stroke(px(stroke));
    path.move_to(at(start));
    path.arc_to(
        point(px(r), px(r)),
        px(0.0),
        true,
        true,
        at(start + 1.5 * PI),
    );
    if let Ok(path) = path.build() {
        window.paint_path(path, color);
    }
}

impl RenderOnce for Spinner {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or_else(|| palette(cx).fg);
        let side = self.size;
        let stroke = (side / 8.0).max(1.5);
        div().flex_none().size(px(side)).with_animation(
            self.id,
            Animation::new(motion::SPIN).repeat(),
            move |el, delta| {
                el.child(
                    canvas(
                        |_, _, _| (),
                        move |b, _, w, _| paint_arc(b, delta, stroke, color, w),
                    )
                    .size_full(),
                )
            },
        )
    }
}
