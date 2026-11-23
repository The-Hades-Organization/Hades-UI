use std::time::Duration;

use gpui::{
    App, ClipboardItem, ElementId, Entity, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, px,
};

use crate::components::{Button, Tip, button::focus_ring};
use crate::icons::{Icon, IconName};
use crate::theme::palette;

const CONFIRMATION: Duration = Duration::from_millis(1500);
const ICON_BOX: f32 = 24.0;

/// Botão que copia um texto para a área de transferência do gpui e mostra "Copiado" por 1,5 s.
/// `icon_only()` é o `CopyButton` do painel (`ghost icon-xs`): só o ícone, com o rótulo na dica.
#[derive(IntoElement)]
pub struct CopyButton {
    id: ElementId,
    text: SharedString,
    label: SharedString,
    copied_label: SharedString,
    icon_only: bool,
}

impl CopyButton {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            label: "Copiar".into(),
            copied_label: "Copiado".into(),
            icon_only: false,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }

    pub fn copied_label(mut self, label: impl Into<SharedString>) -> Self {
        self.copied_label = label.into();
        self
    }

    /// Quadrado de 24 px só com o ícone (copiar, e o visto depois de copiar); o rótulo vai para a dica.
    pub fn icon_only(mut self) -> Self {
        self.icon_only = true;
        self
    }
}

fn copy(text: &SharedString, copied: &Entity<bool>, cx: &mut App) {
    cx.write_to_clipboard(ClipboardItem::new_string(text.to_string()));
    copied.update(cx, |v, cx| {
        *v = true;
        cx.notify();
    });
    let copied = copied.clone();
    cx.spawn(async move |cx| {
        cx.background_executor().timer(CONFIRMATION).await;
        copied.update(cx, |v, cx| {
            *v = false;
            cx.notify();
        });
    })
    .detach();
}

impl RenderOnce for CopyButton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let copied =
            window.use_keyed_state(ElementId::from((self.id.clone(), "copiado")), cx, |_, _| {
                false
            });
        let done = *copied.read(cx);
        let label = if done { self.copied_label } else { self.label };
        let text = self.text;
        if !self.icon_only {
            return Button::new(self.id, label)
                .on_click(move |_, _, cx| copy(&text, &copied, cx))
                .into_any_element();
        }
        let p = palette(cx);
        let ring = focus_ring(&p, p.muted);
        let hover = p.hover;
        let fg = p.fg;
        div()
            .id(self.id)
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .size(px(ICON_BOX))
            .text_color(p.muted_fg)
            .cursor_pointer()
            .tab_stop(true)
            .hover(move |s| s.bg(hover).text_color(fg))
            .focus_visible(move |s| s.shadow(ring.clone()))
            .tooltip(Tip::text(label))
            .on_click(move |_, _, cx| copy(&text, &copied, cx))
            .child(
                Icon::new(if done {
                    IconName::Check
                } else {
                    IconName::Copy
                })
                .size(px(14.0)),
            )
            .into_any_element()
    }
}
