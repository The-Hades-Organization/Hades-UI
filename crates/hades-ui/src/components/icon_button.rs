use std::rc::Rc;

use gpui::{
    App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, px,
};

use crate::components::{Tip, button::focus_ring};
use crate::icons::{Icon, IconName};
use crate::theme::palette;
use crate::tokens::Palette;

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// Botão só com ícone (`Button variant="ghost" size="icon-sm"` do painel): quadrado de 32 px, sem fundo, hover
/// `--hover`; o rótulo vai para a dica e para quem lê a tela. `pressed` marca o estado ligado com a linha de base
/// `--primary`, como `aria-pressed`.
#[derive(IntoElement)]
pub struct IconButton {
    id: ElementId,
    icon: IconName,
    label: SharedString,
    pressed: bool,
    on_click: Option<ClickHandler>,
}

impl IconButton {
    pub fn new(id: impl Into<ElementId>, icon: IconName, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            icon,
            label: label.into(),
            pressed: false,
            on_click: None,
        }
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    pub fn on_click(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(f));
        self
    }
}

impl RenderOnce for IconButton {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let ring = focus_ring(&p, p.bg);
        let hover = p.hover;
        let mut button = div()
            .id(self.id)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(px(32.0))
            .text_color(p.fg)
            .cursor_pointer()
            .tab_stop(true)
            .hover(move |s| s.bg(hover))
            .focus_visible(move |s| s.shadow(ring.clone()))
            .tooltip(Tip::text(self.label))
            .child(Icon::new(self.icon).size(px(16.0)));
        if self.pressed {
            button = button
                .bg(p.muted)
                .shadow(vec![Palette::baseline(p.primary, 2.0)]);
        }
        if let Some(f) = self.on_click {
            button = button.on_click(move |e, window, cx| f(e, window, cx));
        }
        button
    }
}
