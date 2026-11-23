use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};

use crate::icons::{Icon, IconName};
use crate::theme::palette;
use crate::tokens::{FONT_HEADING, FONT_MONO, size};

/// Cabeçalho de página em duas linhas. Na primeira, o título Marcellus 40 com o subtítulo embaixo, à esquerda, e
/// à direita as ações (botões, segmentado) e o metadado (`branch @ sha`, em mono). Na segunda, o parágrafo de
/// apoio corre na largura cheia da página, sem disputar espaço com as ações. O texto quebra em vez de cortar.
#[derive(IntoElement)]
pub struct PageHeader {
    title: SharedString,
    subtitle: Option<SharedString>,
    description: Option<SharedString>,
    meta: Option<SharedString>,
    meta_icon: Option<IconName>,
    actions: Vec<AnyElement>,
}

impl PageHeader {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            description: None,
            meta: None,
            meta_icon: None,
            actions: Vec::new(),
        }
    }

    /// Linha curta sob o título (`cur.desc` do painel): 14 px, `muted-foreground`.
    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Parágrafo de apoio abaixo do título, em largura cheia: explica o que a página faz, em `foreground` a 14 px.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Metadado à direita do título, depois das ações, em mono a 12 px e `muted-foreground` (ex.: `main @ 1a2b3c4`).
    pub fn meta(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    /// Ícone antes do metadado (ex.: [`IconName::GitBranch`]).
    pub fn meta_icon(mut self, icon: IconName) -> Self {
        self.meta_icon = Some(icon);
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
        let mut title = div().flex().flex_col().flex_1().min_w_0().child(
            div()
                .font_family(FONT_HEADING)
                .text_size(px(size::PAGE_TITLE))
                .line_height(px(size::PAGE_TITLE * 1.25))
                .child(self.title),
        );
        if let Some(subtitle) = self.subtitle {
            title = title.child(
                div()
                    .mt(px(8.0))
                    .text_color(p.muted_fg)
                    .text_size(px(size::BODY))
                    .child(subtitle),
            );
        }
        let mut side = div()
            .flex()
            .flex_none()
            .flex_wrap()
            .items_center()
            .gap(px(12.0))
            .children(self.actions);
        if let Some(meta) = self.meta {
            let mut tag = div()
                .flex()
                .items_center()
                .gap(px(6.0))
                .text_color(p.muted_fg)
                .font_family(FONT_MONO)
                .text_size(px(size::META));
            if let Some(icon) = self.meta_icon {
                tag = tag.child(Icon::new(icon).size(px(14.0)));
            }
            side = side.child(tag.child(meta));
        }
        let mut header = div()
            .flex()
            .flex_col()
            .gap(px(16.0))
            .text_color(p.fg)
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_end()
                    .justify_between()
                    .gap_x(px(24.0))
                    .gap_y(px(8.0))
                    .child(title)
                    .child(side),
            );
        if let Some(description) = self.description {
            header = header.child(
                div()
                    .w_full()
                    .text_size(px(size::BODY))
                    .line_height(px(20.0))
                    .child(description),
            );
        }
        header
    }
}
