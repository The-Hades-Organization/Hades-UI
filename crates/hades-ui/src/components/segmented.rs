use std::rc::Rc;

use gpui::{
    App, ElementId, FontWeight, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, div, px,
};

use super::button::focus_ring;
use crate::theme::palette;
use crate::tokens::{Palette, control, size};

type SelectHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// Seletor de uma opção entre poucas (`Segmented` do painel): selecionado em `secondary` com linha de base `primary`. O estado é do chamador: `selected` e `on_select`.
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
            .h(px(control::MD))
            .text_size(px(size::LABEL))
            .font_weight(FontWeight::SEMIBOLD);
        for (i, label) in self.options.into_iter().enumerate() {
            let on = i == self.selected;
            let mut seg = div()
                .id(ElementId::NamedInteger("seg".into(), i as u64))
                .flex()
                .items_center()
                .h_full()
                .px(px(12.0))
                .text_color(p.muted_fg)
                .child(label);
            if on {
                seg = seg
                    .bg(p.muted)
                    .text_color(p.fg)
                    .shadow(vec![Palette::baseline(p.primary, 2.0)]);
            }
            if let Some(f) = self.on_select.clone() {
                let ring = focus_ring(&p, p.bg);
                seg = seg
                    .cursor_pointer()
                    .tab_stop(true)
                    .hover(move |s| s.bg(p.hover).text_color(p.fg))
                    .focus_visible(move |s| s.shadow(ring.clone()))
                    .on_click(move |_, window, cx| f(i, window, cx));
            }
            row = row.child(seg);
        }
        row
    }
}
