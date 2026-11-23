use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    ScrollHandle, SharedString, StatefulInteractiveElement, Styled, Window, div, px,
};

use crate::theme::palette;
use crate::tokens::{FONT_MONO, size};

/// Largura máxima do conteúdo (`max-w-[1500px]` do `main` do painel).
const PAGE_MAX_WIDTH: f32 = 1500.0;

/// Área de conteúdo de uma página (`main` do painel): ocupa a largura e a altura que sobram ao lado do trilho, com
/// as margens do painel (32 px dos lados, 28 em cima), e rola na vertical.
///
/// Os filhos ficam numa coluna flex com 24 px entre eles. Filho com `fill` (`Card`, `DataTable`, `EmptyState`, ou
/// [`Page::fill_child`]) cresce até o pé da janela, então tabela e estado vazio não deixam área morta embaixo.
#[derive(IntoElement)]
pub struct Page {
    id: ElementId,
    header: Option<AnyElement>,
    children: Vec<AnyElement>,
    scroll: Option<ScrollHandle>,
}

impl Page {
    /// `id` identifica a área de rolagem.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            header: None,
            children: Vec::new(),
            scroll: None,
        }
    }

    /// Liga a rolagem a um `ScrollHandle` do app (para rolar por código ou ler a posição).
    pub fn track_scroll(mut self, handle: &ScrollHandle) -> Self {
        self.scroll = Some(handle.clone());
        self
    }

    /// Cabeçalho da página (em geral um `PageHeader`), acima dos filhos.
    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    /// Embrulha `element` numa coluna que cresce até o pé da página, para conteúdo sem `fill` próprio.
    pub fn fill_child(mut self, element: impl IntoElement) -> Self {
        self.children.push(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .min_h_0()
                .min_w_0()
                .child(element)
                .into_any_element(),
        );
        self
    }
}

impl ParentElement for Page {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Page {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let mut area = div().id(self.id);
        if let Some(handle) = &self.scroll {
            area = area.track_scroll(handle);
        }
        area.flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .min_h_0()
            .size_full()
            .overflow_y_scroll()
            .bg(p.bg)
            .text_color(p.fg)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_grow(1.0)
                    .flex_shrink_0()
                    .min_h_full()
                    .w_full()
                    .max_w(px(PAGE_MAX_WIDTH))
                    .mx_auto()
                    .gap(px(24.0))
                    .px(px(32.0))
                    .pt(px(28.0))
                    .pb(px(24.0))
                    .children(self.header)
                    .children(self.children),
            )
    }
}

/// Valor de uma linha da [`MetaList`].
enum MetaValue {
    Text(SharedString),
    Mono(SharedString),
    Element(AnyElement),
}

/// Lista de metadados (rótulo e valor), uma linha por par. O rótulo fica numa coluna de largura fixa em
/// `muted-foreground`; o valor ocupa o resto e quebra em vez de cortar, mesmo sem espaço (caminho, sha).
#[derive(IntoElement)]
pub struct MetaList {
    rows: Vec<(SharedString, MetaValue)>,
    label_width: f32,
}

impl MetaList {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            label_width: 160.0,
        }
    }

    pub fn row(mut self, label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        self.rows
            .push((label.into(), MetaValue::Text(value.into())));
        self
    }

    /// Linha com o valor em mono (caminho, sha, comando).
    pub fn mono(mut self, label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        self.rows
            .push((label.into(), MetaValue::Mono(value.into())));
        self
    }

    /// Linha com um elemento no lugar do valor (ex.: um [`Meter`](super::Meter) com rótulo e valor próprios,
    /// ou um selo), alinhada às outras.
    pub fn element(mut self, label: impl Into<SharedString>, value: impl IntoElement) -> Self {
        self.rows
            .push((label.into(), MetaValue::Element(value.into_any_element())));
        self
    }

    /// Largura da coluna de rótulos; o padrão é 160 px.
    pub fn label_width(mut self, width: f32) -> Self {
        self.label_width = width;
        self
    }
}

impl Default for MetaList {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for MetaList {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let mut list = div()
            .flex()
            .flex_col()
            .gap(px(6.0))
            .w_full()
            .min_w_0()
            .text_size(px(size::LABEL))
            .line_height(px(20.0));
        for (label, value) in self.rows {
            let cell = div().flex_1().min_w_0().text_color(p.fg);
            let value = match value {
                MetaValue::Text(t) => cell.child(t),
                MetaValue::Mono(t) => cell
                    .font_family(FONT_MONO)
                    .text_size(px(size::META))
                    .child(t),
                MetaValue::Element(e) => cell.pt(px(2.0)).child(e),
            };
            list = list.child(
                div()
                    .flex()
                    .items_start()
                    .gap(px(12.0))
                    .w_full()
                    .min_w_0()
                    .child(
                        div()
                            .flex_none()
                            .w(px(self.label_width))
                            .text_color(p.muted_fg)
                            .child(label),
                    )
                    .child(value),
            );
        }
        list
    }
}
