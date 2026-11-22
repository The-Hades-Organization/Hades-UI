use std::time::Duration;

use gpui::{App, ClipboardItem, ElementId, IntoElement, RenderOnce, SharedString, Window};

use crate::components::Button;

const CONFIRMATION: Duration = Duration::from_millis(1500);

/// Botão que copia um texto para a área de transferência do gpui e mostra "Copiado" por 1,5 s.
#[derive(IntoElement)]
pub struct CopyButton {
    id: ElementId,
    text: SharedString,
    label: SharedString,
    copied_label: SharedString,
}

impl CopyButton {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            label: "Copiar".into(),
            copied_label: "Copiado".into(),
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
}

impl RenderOnce for CopyButton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let copied =
            window.use_keyed_state(ElementId::from((self.id.clone(), "copiado")), cx, |_, _| {
                false
            });
        let label = if *copied.read(cx) {
            self.copied_label
        } else {
            self.label
        };
        let text = self.text;
        Button::new(self.id, label).on_click(move |_, _, cx| {
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
        })
    }
}
