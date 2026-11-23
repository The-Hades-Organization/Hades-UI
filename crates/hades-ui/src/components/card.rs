use gpui::{
    AnyElement, App, Div, Hsla, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window, div, px,
};

use crate::theme::palette;
use crate::tokens::{CORNER_ARM, CORNER_STROKE, FONT_HEADING, size};

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
/// na cor `line`. O título usa a fonte de título a 18 px. `object()` é a variante compacta (p 14, gap 10).
#[derive(IntoElement)]
pub struct Card {
    title: Option<SharedString>,
    description: Option<SharedString>,
    object: bool,
    children: Vec<AnyElement>,
}

impl Card {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            object: false,
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
        if self.title.is_some() || self.description.is_some() {
            let mut head = div().flex().flex_col().gap(px(2.0));
            if let Some(title) = self.title {
                head = head.child(
                    div()
                        .font_family(FONT_HEADING)
                        .text_size(px(size::SECTION))
                        .child(title),
                );
            }
            if let Some(description) = self.description {
                head = head.child(
                    div()
                        .text_color(p.muted_fg)
                        .text_size(px(size::BODY))
                        .child(description),
                );
            }
            card = card.child(head);
        }
        card.children(self.children)
    }
}
