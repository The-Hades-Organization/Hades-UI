use std::rc::Rc;

use gpui::{
    App, ClickEvent, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    Styled, Window, div, px,
};

use crate::components::Button;
use crate::components::card::corners;
use crate::theme::palette;
use crate::tokens::{FONT_HEADING, FONT_MONO, size};

type Handler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// Diálogo de confirmação sobre a janela inteira. Renderize-o por último, só enquanto estiver aberto;
/// quem decide abrir e fechar é o chamador, nos handlers de `on_confirm` e `on_cancel`.
#[derive(IntoElement)]
pub struct Confirm {
    title: SharedString,
    detail: Option<SharedString>,
    target: Option<SharedString>,
    confirm_label: SharedString,
    cancel_label: SharedString,
    on_confirm: Option<Handler>,
    on_cancel: Option<Handler>,
}

impl Confirm {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            detail: None,
            target: None,
            confirm_label: "Confirmar".into(),
            cancel_label: "Cancelar".into(),
            on_confirm: None,
            on_cancel: None,
        }
    }

    pub fn detail(mut self, detail: impl Into<SharedString>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Caminho ou alvo da ação, em mono e à vista (ex.: o arquivo que será gravado).
    pub fn target(mut self, target: impl Into<SharedString>) -> Self {
        self.target = Some(target.into());
        self
    }

    pub fn confirm_label(mut self, label: impl Into<SharedString>) -> Self {
        self.confirm_label = label.into();
        self
    }

    pub fn cancel_label(mut self, label: impl Into<SharedString>) -> Self {
        self.cancel_label = label.into();
        self
    }

    pub fn on_confirm(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_confirm = Some(Rc::new(f));
        self
    }

    pub fn on_cancel(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_cancel = Some(Rc::new(f));
        self
    }
}

impl RenderOnce for Confirm {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let mut confirm = Button::new("confirm-ok", self.confirm_label).primary();
        if let Some(f) = self.on_confirm {
            confirm = confirm.on_click(move |e, w, cx| f(e, w, cx));
        }
        let mut cancel = Button::new("confirm-cancel", self.cancel_label);
        if let Some(f) = self.on_cancel {
            cancel = cancel.on_click(move |e, w, cx| f(e, w, cx));
        }
        let mut body = div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .w(px(420.0))
            .p(px(20.0))
            .relative()
            .bg(p.card)
            .shadow(vec![p.dialog_shadow()])
            .text_color(p.fg)
            .children(corners(p.primary))
            .child(
                div()
                    .font_family(FONT_HEADING)
                    .text_size(px(size::SECTION))
                    .child(self.title),
            );
        if let Some(detail) = self.detail {
            body = body.child(
                div()
                    .text_size(px(size::LABEL))
                    .text_color(p.muted_fg)
                    .child(detail),
            );
        }
        if let Some(target) = self.target {
            body = body.child(
                div()
                    .px(px(10.0))
                    .py(px(7.0))
                    .bg(p.popover)
                    .font_family(FONT_MONO)
                    .text_size(px(size::META))
                    .child(target),
            );
        }
        body = body.child(
            div()
                .flex()
                .justify_end()
                .gap(px(8.0))
                .child(cancel)
                .child(confirm),
        );
        div()
            .id("confirm-scrim")
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(gpui::black().opacity(0.5))
            .occlude()
            .child(body)
    }
}
