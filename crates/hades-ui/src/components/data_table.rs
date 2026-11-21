use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};

use crate::theme::palette;
use crate::tokens::{FONT_MONO, size};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Align {
    #[default]
    Start,
    End,
}

/// Tabela simples de células de texto: cabeçalho mono em caixa alta, fios entre linhas, linha selecionável.
/// Para células com selo ou ação, monte as linhas com `div` e use as mesmas cores da paleta.
#[derive(IntoElement)]
pub struct DataTable {
    columns: Vec<(SharedString, Align)>,
    rows: Vec<Vec<SharedString>>,
    selected: Option<usize>,
}

impl DataTable {
    pub fn new(columns: impl IntoIterator<Item = (SharedString, Align)>) -> Self {
        Self {
            columns: columns.into_iter().collect(),
            rows: Vec::new(),
            selected: None,
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
}

impl RenderOnce for DataTable {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let aligns: Vec<Align> = self.columns.iter().map(|(_, a)| *a).collect();
        let cell = |text: SharedString, align: Align| {
            let c = div().flex_1().min_w_0().px(px(8.0)).py(px(7.0)).child(text);
            if align == Align::End {
                c.flex().justify_end()
            } else {
                c
            }
        };
        let mut head = div()
            .flex()
            .border_b_1()
            .border_color(p.border_strong)
            .text_color(p.muted_fg)
            .font_family(FONT_MONO)
            .text_size(px(size::MICRO));
        for (title, align) in self.columns {
            head = head.child(cell(title.to_uppercase().into(), align));
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
            table = table.child(line);
        }
        table
    }
}
