use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};

use crate::theme::palette;
use crate::tokens::{FONT_MONO, RAIL_WIDTH, size};

/// Item da navegação lateral, com contador opcional.
pub struct RailItem {
    label: SharedString,
    badge: Option<SharedString>,
}

impl RailItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            badge: None,
        }
    }

    pub fn badge(mut self, badge: impl Into<SharedString>) -> Self {
        self.badge = Some(badge.into());
        self
    }
}

/// Grupo de itens com título em caixa alta.
pub struct RailGroup {
    title: SharedString,
    items: Vec<RailItem>,
}

impl RailGroup {
    pub fn new(title: impl Into<SharedString>, items: impl IntoIterator<Item = RailItem>) -> Self {
        Self {
            title: title.into(),
            items: items.into_iter().collect(),
        }
    }
}

/// Trilho lateral escuro nos dois modos; `selected` é o rótulo do item ativo.
#[derive(IntoElement)]
pub struct Rail {
    groups: Vec<RailGroup>,
    selected: Option<SharedString>,
}

impl Rail {
    pub fn new(groups: impl IntoIterator<Item = RailGroup>) -> Self {
        Self {
            groups: groups.into_iter().collect(),
            selected: None,
        }
    }

    pub fn selected(mut self, label: impl Into<SharedString>) -> Self {
        self.selected = Some(label.into());
        self
    }
}

impl RenderOnce for Rail {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let mut rail = div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .py(px(12.0))
            .w(px(RAIL_WIDTH))
            .h_full()
            .flex_none()
            .overflow_hidden()
            .bg(p.rail)
            .text_color(p.rail_fg)
            .text_size(px(size::LABEL));
        for group in self.groups {
            rail = rail.child(
                div()
                    .px(px(14.0))
                    .pt(px(12.0))
                    .pb(px(6.0))
                    .text_color(p.rail_mute)
                    .font_family(FONT_MONO)
                    .text_size(px(size::MICRO))
                    .child(group.title.to_uppercase()),
            );
            for item in group.items {
                let on = self.selected.as_ref() == Some(&item.label);
                let mut row = div()
                    .flex()
                    .justify_between()
                    .px(px(14.0))
                    .py(px(7.0))
                    .border_l_2()
                    .border_color(gpui::transparent_black());
                if on {
                    row = row
                        .bg(p.rail_sel.opacity(0.14))
                        .border_color(p.rail_sel)
                        .text_color(p.rail_sel);
                }
                row = row.child(item.label);
                if let Some(badge) = item.badge {
                    row = row.child(
                        div()
                            .text_color(p.rail_mute)
                            .font_family(FONT_MONO)
                            .text_size(px(size::MICRO))
                            .child(badge),
                    );
                }
                rail = rail.child(row);
            }
        }
        rail
    }
}
