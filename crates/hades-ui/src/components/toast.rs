use std::rc::Rc;

use gpui::{
    Animation, AnimationExt, App, BoxShadow, ClickEvent, ElementId, Hsla, InteractiveElement,
    IntoElement, ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, Styled,
    Window, div, px,
};

use super::{Dot, Spinner};
use crate::theme::palette;
use crate::tokens::{Tone, motion, size};

type CloseHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// Aviso passageiro (`ToastItem`): fundo `popover`, barra de 5 px na cor do tipo à esquerda, título 13 px,
/// descrição em `muted-foreground` e fechar que acende no hover. Entra com o `emergir` (sobe 8 px em 450 ms).
/// A fila, a posição e o tempo de vida ficam com o chamador.
#[derive(IntoElement)]
pub struct Toast {
    id: ElementId,
    tone: Tone,
    title: SharedString,
    description: Option<SharedString>,
    on_close: Option<CloseHandler>,
}

impl Toast {
    pub fn new(id: impl Into<ElementId>, tone: Tone, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            tone,
            title: title.into(),
            description: None,
            on_close: None,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn on_close(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Rc::new(f));
        self
    }
}

fn shadow() -> Vec<BoxShadow> {
    vec![
        BoxShadow::new(px(0.0), px(10.0), Hsla::black().opacity(0.7))
            .blur_radius(px(30.0))
            .spread_radius(px(-10.0)),
    ]
}

impl RenderOnce for Toast {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let color = p.tone(self.tone);
        let mark = if self.tone == Tone::Busy {
            Spinner::new((self.id.clone(), "giro"))
                .size(16.0)
                .color(color)
                .into_any_element()
        } else {
            div()
                .flex()
                .items_center()
                .justify_center()
                .size(px(16.0))
                .child(Dot::new((self.id.clone(), "ponto"), self.tone).pulse(false))
                .into_any_element()
        };
        let mut text = div().flex().flex_col().flex_1().gap(px(4.0)).child(
            div()
                .text_size(px(size::LABEL))
                .font_weight(gpui::FontWeight::MEDIUM)
                .child(self.title),
        );
        if let Some(description) = self.description {
            text = text.child(
                div()
                    .text_color(p.muted_fg)
                    .text_size(px(size::BODY))
                    .child(description),
            );
        }
        let mut close = div()
            .id((self.id.clone(), "fechar"))
            .px(px(2.0))
            .text_color(p.muted_fg)
            .text_size(px(size::BODY))
            .cursor_pointer()
            .hover(move |s| s.text_color(p.fg))
            .child("×");
        if let Some(f) = self.on_close {
            close = close.on_click(move |e, window, cx| f(e, window, cx));
        }
        div()
            .relative()
            .flex()
            .items_start()
            .gap(px(12.0))
            .w(px(384.0))
            .py(px(12.0))
            .pr(px(12.0))
            .pl(px(18.0))
            .bg(p.popover)
            .text_color(p.fg)
            .shadow(shadow())
            .child(
                div()
                    .absolute()
                    .top_0()
                    .bottom_0()
                    .left_0()
                    .w(px(5.0))
                    .bg(color),
            )
            .child(mark)
            .child(text)
            .child(close)
            .with_animation(
                self.id,
                Animation::new(motion::EMERGE).with_easing(motion::emerge()),
                |el, t| el.opacity(t).top(px(motion::EMERGE_RISE * (1.0 - t))),
            )
    }
}
