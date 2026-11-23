use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};

use crate::theme::palette;
use crate::tokens::{FONT_MONO, STATUS_BAR_HEIGHT, size};

/// Barra de estado no pé da janela, no tom do rodapé da sidebar do painel ("Memória do Hades"): texto em
/// `muted-foreground` a 12 px, à esquerda e à direita, e o rótulo de versão que o app escolher na ponta direita,
/// em mono (ex.: `moirai 0.3.0` ou `hades-ui 0.2.4`).
#[derive(IntoElement)]
pub struct StatusBar {
    left: SharedString,
    right: SharedString,
    version: Option<SharedString>,
}

impl StatusBar {
    pub fn new(left: impl Into<SharedString>, right: impl Into<SharedString>) -> Self {
        Self {
            left: left.into(),
            right: right.into(),
            version: None,
        }
    }

    /// Rótulo de versão livre, depois do texto da direita, separado por um filete.
    pub fn version(mut self, label: impl Into<SharedString>) -> Self {
        self.version = Some(label.into());
        self
    }
}

impl RenderOnce for StatusBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let mut right = div()
            .flex()
            .flex_none()
            .items_center()
            .gap(px(12.0))
            .child(self.right);
        if let Some(version) = self.version {
            right = right
                .child(div().w(px(1.0)).h(px(12.0)).bg(p.border))
                .child(
                    div()
                        .font_family(FONT_MONO)
                        .text_size(px(size::MICRO))
                        .child(version),
                );
        }
        div()
            .flex()
            .items_center()
            .justify_between()
            .gap(px(16.0))
            .px(px(16.0))
            .h(px(STATUS_BAR_HEIGHT))
            .flex_none()
            .bg(p.bg)
            .text_color(p.muted_fg)
            .border_t_1()
            .border_color(p.border)
            .text_size(px(size::META))
            .child(div().min_w_0().truncate().child(self.left))
            .child(right)
    }
}
