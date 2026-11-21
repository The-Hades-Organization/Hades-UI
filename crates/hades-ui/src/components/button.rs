use std::rc::Rc;

use gpui::{
    App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, px,
};
use gpui_component::tooltip::Tooltip;

use crate::theme::palette;
use crate::tokens::size;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonKind {
    #[default]
    Secondary,
    Primary,
}

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// Botão quadrado do design (`.btn` e `.btn.p`). Desabilitado, mostra o motivo no tooltip.
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    kind: ButtonKind,
    disabled_reason: Option<SharedString>,
    on_click: Option<ClickHandler>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind: ButtonKind::default(),
            disabled_reason: None,
            on_click: None,
        }
    }

    pub fn primary(mut self) -> Self {
        self.kind = ButtonKind::Primary;
        self
    }

    /// Desabilita o botão; o motivo aparece ao pairar o ponteiro.
    pub fn disabled(mut self, reason: impl Into<SharedString>) -> Self {
        self.disabled_reason = Some(reason.into());
        self
    }

    pub fn on_click(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(f));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let (bg, fg, border) = match self.kind {
            ButtonKind::Primary => (p.primary, p.primary_fg, p.primary),
            ButtonKind::Secondary => (p.card, p.fg, p.border_strong),
        };
        let base = div()
            .id(self.id)
            .px(px(10.0))
            .py(px(7.0))
            .border_1()
            .border_color(border)
            .bg(bg)
            .text_color(fg)
            .text_size(px(size::META))
            .child(self.label);
        match self.disabled_reason {
            Some(reason) => base
                .opacity(0.45)
                .cursor_not_allowed()
                .tooltip(move |window, cx| Tooltip::new(reason.clone()).build(window, cx)),
            None => {
                let base = base
                    .cursor_pointer()
                    .focus_visible(|s| s.border_color(p.vivo).border_2());
                match self.on_click {
                    Some(f) => base.on_click(move |e, window, cx| f(e, window, cx)),
                    None => base,
                }
            }
        }
    }
}
