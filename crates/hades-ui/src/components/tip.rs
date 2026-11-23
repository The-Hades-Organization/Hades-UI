use gpui::{
    Animation, AnimationExt, AnyView, App, AppContext, BoxShadow, Context, IntoElement,
    ParentElement, Render, SharedString, Styled, Window, div, px,
};

use crate::theme::palette;
use crate::tokens::{motion, size};

/// Dica flutuante (`Tip`): fundo `popover`, 12 px, sombra curta, entra com `fade-in` de 250 ms.
/// Use com `.tooltip(Tip::text("…"))` em qualquer elemento com id.
pub struct Tip {
    text: SharedString,
}

impl Tip {
    /// Construtor no formato que o `.tooltip(...)` do gpui pede.
    pub fn text(
        text: impl Into<SharedString>,
    ) -> impl Fn(&mut Window, &mut App) -> AnyView + 'static {
        let text = text.into();
        move |_, cx| cx.new(|_| Tip { text: text.clone() }).into()
    }
}

impl Render for Tip {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = palette(cx);
        div()
            .px(px(10.0))
            .py(px(6.0))
            .max_w(px(320.0))
            .bg(p.popover)
            .text_color(p.fg)
            .text_size(px(size::META))
            .shadow(vec![
                BoxShadow::new(px(0.0), px(10.0), gpui::Hsla::black().opacity(0.7))
                    .blur_radius(px(30.0))
                    .spread_radius(px(-10.0)),
            ])
            .child(self.text.clone())
            .with_animation(
                "tip-fade",
                Animation::new(motion::FADE_IN).with_easing(motion::ease_out()),
                |el, t| el.opacity(t),
            )
    }
}
