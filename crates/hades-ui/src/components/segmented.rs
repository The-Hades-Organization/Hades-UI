use std::rc::Rc;

use gpui::{
    App, ElementId, FontWeight, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, px,
};

use crate::theme::palette;
use crate::tokens::size;

type SelectHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// Seletor de uma opção entre poucas (`.seg`). O estado é do chamador: `selected` e `on_select`.
#[derive(IntoElement)]
pub struct Segmented {
    id: ElementId,
    options: Vec<SharedString>,
    selected: usize,
    on_select: Option<SelectHandler>,
}

impl Segmented {
    pub fn new(
        id: impl Into<ElementId>,
        options: impl IntoIterator<Item = impl Into<SharedString>>,
    ) -> Self {
        Self {
            id: id.into(),
            options: options.into_iter().map(Into::into).collect(),
            selected: 0,
            on_select: None,
        }
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    pub fn on_select(mut self, f: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(f));
        self
    }
}

impl RenderOnce for Segmented {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let mut row = div()
            .id(self.id.clone())
            .flex()
            .flex_none()
            .border_1()
            .border_color(p.border_strong)
            .text_size(px(size::META));
        for (i, label) in self.options.into_iter().enumerate() {
            let on = i == self.selected;
            let mut seg = div()
                .id(ElementId::NamedInteger("seg".into(), i as u64))
                .px(px(10.0))
                .py(px(6.0))
                .text_color(p.fg)
                .child(label);
            if on {
                seg = seg.bg(p.muted).font_weight(FontWeight::SEMIBOLD);
            }
            if let Some(f) = self.on_select.clone() {
                seg = seg
                    .cursor_pointer()
                    .on_click(move |_, window, cx| f(i, window, cx));
            }
            row = row.child(seg);
        }
        row
    }
}
