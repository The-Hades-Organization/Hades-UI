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
    on_rail: bool,
    on_click: Option<ClickHandler>,
}

impl IconButton {
    pub fn new(id: impl Into<ElementId>, icon: IconName, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            icon,
            label: label.into(),
            pressed: false,
            on_rail: false,
            on_click: None,
        }
    }

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    /// Cores do trilho escuro (botão de recolher do painel): ícone em `rail-muted`, `rail-foreground` no hover,
    /// sem fundo e com o anel de foco em metal.
    pub fn on_rail(mut self) -> Self {
        self.on_rail = true;
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
        let mut button = div()
            .id(self.id)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(px(32.0))
            .cursor_pointer()
            .tab_stop(true);
        button = if self.on_rail {
            let (hover_fg, metal) = (p.rail_fg, p.metal);
            button
                .text_color(p.rail_mute)
                .hover(move |s| s.text_color(hover_fg))
                .focus_visible(move |s| {
                    s.shadow(vec![
                        gpui::BoxShadow::new(px(0.0), px(0.0), metal).spread_radius(px(2.0)),
                    ])
                })
        } else {
            let ring = focus_ring(&p, p.bg);
            let hover = p.hover;
            button
                .text_color(p.fg)
                .hover(move |s| s.bg(hover))
                .focus_visible(move |s| s.shadow(ring.clone()))
        };
        button = button
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
