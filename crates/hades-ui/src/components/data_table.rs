use std::rc::Rc;

use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Window, div, px,
};

use crate::theme::palette;
use crate::tokens::{FONT_MONO, size};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Align {
    #[default]
    Start,
    /// Coluna numérica: alinhada à direita e em fonte mono, de largura fixa por dígito.
    Numeric,
    End,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

type IndexHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// Tabela simples de células de texto: cabeçalho mono em caixa alta, fios entre linhas, linha selecionável.
/// Para células com selo ou ação, monte as linhas com `div` e use as mesmas cores da paleta.
#[derive(IntoElement)]
pub struct DataTable {
    columns: Vec<(SharedString, Align)>,
    rows: Vec<Vec<SharedString>>,
    selected: Option<usize>,
    sort: Option<(usize, SortDirection)>,
    on_select: Option<IndexHandler>,
    on_sort: Option<IndexHandler>,
}

impl DataTable {
    pub fn new(columns: impl IntoIterator<Item = (SharedString, Align)>) -> Self {
        Self {
            columns: columns.into_iter().collect(),
            rows: Vec::new(),
            selected: None,
            sort: None,
            on_select: None,
            on_sort: None,
        }
    }

    pub fn row(mut self, cells: impl IntoIterator<Item = SharedString>) -> Self {
        self.rows.push(cells.into_iter().collect());
        self
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = Some(index);
        self
    }

    /// Coluna ordenada e sentido, mostrados como seta no cabeçalho. A ordenação em si é do chamador.
    pub fn sorted(mut self, column: usize, direction: SortDirection) -> Self {
        self.sort = Some((column, direction));
        self
    }

    /// Clique numa linha; recebe o índice dela.
    pub fn on_select(mut self, f: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(f));
        self
    }

    /// Clique no cabeçalho; recebe o índice da coluna.
    pub fn on_sort(mut self, f: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_sort = Some(Rc::new(f));
        self
    }
}

impl RenderOnce for DataTable {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let aligns: Vec<Align> = self.columns.iter().map(|(_, a)| *a).collect();
        let cell = |text: SharedString, align: Align| {
            let c = div().flex_1().min_w_0().px(px(8.0)).py(px(7.0)).child(text);
            match align {
                Align::Start => c,
                Align::Numeric => c.flex().justify_end().font_family(FONT_MONO),
                Align::End => c.flex().justify_end(),
            }
        };
        let mut head = div()
            .flex()
            .border_b_1()
            .border_color(p.border_strong)
            .text_color(p.muted_fg)
            .font_family(FONT_MONO)
            .text_size(px(size::MICRO));
        for (i, (title, align)) in self.columns.into_iter().enumerate() {
            let mark = match self.sort {
                Some((col, SortDirection::Ascending)) if col == i => " ▲",
                Some((col, SortDirection::Descending)) if col == i => " ▼",
                _ => "",
            };
            let c = cell(format!("{}{mark}", title.to_uppercase()).into(), align);
            let c = match self.on_sort.clone() {
                Some(on_sort) => c
                    .id(ElementId::NamedInteger("col".into(), i as u64))
                    .cursor_pointer()
                    .on_click(move |_, window, cx| on_sort(i, window, cx))
                    .into_any_element(),
                None => c.into_any_element(),
            };
            head = head.child(c);
        }
        let mut table = div()
            .flex()
            .flex_col()
            .text_color(p.fg)
            .text_size(px(size::LABEL))
            .child(head);
        for (i, row) in self.rows.into_iter().enumerate() {
            let mut line = div().flex().border_b_1().border_color(p.border);
            if self.selected == Some(i) {
                line = line.bg(p.muted);
            }
            for (text, align) in row.into_iter().zip(aligns.iter()) {
                line = line.child(cell(text, *align));
            }
            if let Some(on_select) = self.on_select.clone() {
                table = table.child(
                    line.id(ElementId::NamedInteger("row".into(), i as u64))
                        .cursor_pointer()
                        .hover(|s| s.bg(p.muted.opacity(0.6)))
                        .on_click(move |_, window, cx| on_select(i, window, cx)),
                );
            } else {
                table = table.child(line);
            }
        }
        table
    }
}
