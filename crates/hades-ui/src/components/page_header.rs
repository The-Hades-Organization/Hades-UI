use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};

use crate::theme::palette;
use crate::tokens::{FONT_HEADING, size};

/// Largura do bloco de texto do cabeçalho (`max-w-[70ch]` do painel a 14 px).
const TEXT_MAX_WIDTH: f32 = 640.0;

/// Cabeçalho de página: título Marcellus 40 à esquerda, com subtítulo e parágrafo de apoio opcionais embaixo, e as
/// ações (botões, segmentado, metadado) à direita, alinhadas pela base. O texto quebra em vez de cortar.
#[derive(IntoElement)]
pub struct PageHeader {
    title: SharedString,
    subtitle: Option<SharedString>,
    description: Option<SharedString>,
    actions: Vec<AnyElement>,
}

impl PageHeader {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            description: None,
            actions: Vec::new(),
        }
    }

    /// Linha curta sob o título (`cur.desc` do painel): 14 px, `muted-foreground`.
    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Parágrafo de apoio abaixo do subtítulo: explica o que a página faz, em `foreground` a 14 px.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn action(mut self, element: impl IntoElement) -> Self {
        self.actions.push(element.into_any_element());
        self
    }
}

impl RenderOnce for PageHeader {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let mut text = div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .max_w(px(TEXT_MAX_WIDTH))
            .child(
                div()
                    .font_family(FONT_HEADING)
                    .text_size(px(size::PAGE_TITLE))
                    .line_height(px(size::PAGE_TITLE * 1.25))
                    .child(self.title),
            );
        if let Some(subtitle) = self.subtitle {
            text = text.child(
                div()
                    .mt(px(8.0))
                    .text_color(p.muted_fg)
                    .text_size(px(size::BODY))
                    .child(subtitle),
            );
        }
        if let Some(description) = self.description {
            text = text.child(
                div()
                    .mt(px(16.0))
                    .text_size(px(size::BODY))
                    .line_height(px(20.0))
                    .child(description),
            );
        }
        div()
            .flex()
            .flex_wrap()
            .items_end()
            .justify_between()
            .gap_x(px(24.0))
            .gap_y(px(8.0))
            .text_color(p.fg)
            .child(text)
            .child(
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .gap(px(12.0))
                    .children(self.actions),
            )
    }
}
