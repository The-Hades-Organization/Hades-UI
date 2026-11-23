use gpui::{
    AnyElement, App, Div, Hsla, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window, div, px,
};

use super::StatusBadge;
use crate::theme::palette;
use crate::tokens::{CORNER_ARM, CORNER_STROKE, FONT_HEADING, Tone, size};

/// Colchetes de canto (`.cantos`): braços de 10 px com traço de 2 px no canto superior esquerdo e no inferior direito.
/// O pai precisa ser `relative()`.
pub(crate) fn corners(color: Hsla) -> [Div; 4] {
    let (arm, stroke) = (px(CORNER_ARM), px(CORNER_STROKE));
    [
        div().absolute().top_0().left_0().w(arm).h(stroke).bg(color),
        div().absolute().top_0().left_0().w(stroke).h(arm).bg(color),
        div()
            .absolute()
            .bottom_0()
            .right_0()
            .w(arm)
            .h(stroke)
            .bg(color),
        div()
            .absolute()
            .bottom_0()
            .right_0()
            .w(stroke)
            .h(arm)
            .bg(color),
    ]
}

/// Cartão do painel (`Card` + `CardHeader`): fundo `card` a 96%, sem borda nem sombra, com colchetes de canto
/// na cor `line`. O título usa a fonte de título a 18 px, com `meta` ao lado e, à direita, o selo de estado
/// (`badge`) e as ações (`action`). `object()` é a variante compacta (p 14, gap 10). [`Meter`](super::Meter) e
/// [`MetaList`](super::MetaList) entram como filhos e ocupam a largura do cartão.
#[derive(IntoElement)]
pub struct Card {
    title: Option<SharedString>,
    description: Option<SharedString>,
    meta: Option<SharedString>,
    badge: Option<StatusBadge>,
    actions: Vec<AnyElement>,
    object: bool,
    fill: bool,
    children: Vec<AnyElement>,
}

impl Card {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            meta: None,
            badge: None,
            actions: Vec::new(),
            object: false,
            fill: false,
            children: Vec::new(),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Metadado curto ao lado do título, em `muted-foreground` a 12 px (ex.: "escolhida · 5 de 7 feitos").
    pub fn meta(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    /// Selo de estado no cabeçalho, à esquerda das ações (ex.: "Escolhida" em `Ok` com visto, "pulada" em
    /// `Neutral`). Para selo com ícone, use [`Card::status`].
    pub fn badge(self, label: impl Into<SharedString>, tone: Tone) -> Self {
        self.status(StatusBadge::new(label, tone))
    }

    /// Selo de estado montado pelo chamador (com ícone, por exemplo).
    pub fn status(mut self, badge: StatusBadge) -> Self {
        self.badge = Some(badge);
        self
    }

    /// Ação no canto direito do cabeçalho (botão, botão de ícone); várias ficam lado a lado, depois do selo.
    pub fn action(mut self, element: impl IntoElement) -> Self {
        self.actions.push(element.into_any_element());
        self
    }

    /// Ocupa a altura que sobra no pai (coluna flex); o conteúdo que também tem `fill` cresce junto.
    pub fn fill(mut self) -> Self {
        self.fill = true;
        self
    }

    /// Variante `object`: cartão de item dentro de lista ou grade.
    pub fn object(mut self) -> Self {
        self.object = true;
        self
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Card {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Card {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let (pad, gap) = if self.object {
            (14.0, 10.0)
        } else {
            (20.0, 16.0)
        };
        let mut card = div()
            .relative()
            .flex()
            .flex_col()
            .min_w_0()
            .gap(px(gap))
            .p(px(pad))
            .bg(p.card.opacity(0.96))
            .text_color(p.fg)
            .children(corners(p.line));
        if self.fill {
            card = card.flex_1().min_h_0();
        }
        if self.title.is_some()
            || self.description.is_some()
            || self.meta.is_some()
            || self.badge.is_some()
            || !self.actions.is_empty()
        {
            let mut line = div()
                .flex()
                .flex_wrap()
                .items_baseline()
                .gap_x(px(12.0))
                .gap_y(px(2.0));
            if let Some(title) = self.title {
                line = line.child(
                    div()
                        .font_family(FONT_HEADING)
                        .text_size(px(size::SECTION))
                        .child(title),
                );
            }
            if let Some(meta) = self.meta {
                line = line.child(
                    div()
                        .text_color(p.muted_fg)
                        .text_size(px(size::META))
                        .child(meta),
                );
            }
            let mut head = div()
                .flex()
                .flex_col()
                .flex_1()
                .min_w_0()
                .gap(px(2.0))
                .child(line);
            if let Some(description) = self.description {
                head = head.child(
                    div()
                        .text_color(p.muted_fg)
                        .text_size(px(size::BODY))
                        .child(description),
                );
            }
            card = card.child(
                div().flex().items_start().gap(px(12.0)).child(head).child(
                    div()
                        .flex()
                        .flex_none()
                        .items_center()
                        .flex_wrap()
                        .gap(px(8.0))
                        .children(self.badge)
                        .children(self.actions),
                ),
            );
        }
        card.children(self.children)
    }
}
