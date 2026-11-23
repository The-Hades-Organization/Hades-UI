use std::rc::Rc;

use gpui::{
    App, BoxShadow, ElementId, FontWeight, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, linear_color_stop,
    linear_gradient, point, px,
};

use crate::theme::palette;
use crate::tokens::{FONT_MONO, RAIL_WIDTH, control, size};

type SelectHandler = Rc<dyn Fn(&SharedString, &mut Window, &mut App)>;

/// Item da navegação lateral, com contador opcional.
pub struct RailItem {
    label: SharedString,
    badge: Option<SharedString>,
    alive: bool,
}

impl RailItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            badge: None,
            alive: false,
        }
    }

    /// Página com atividade: o ponto de luz acende inteiro, longe ou perto da atual.
    pub fn alive(mut self, alive: bool) -> Self {
        self.alive = alive;
        self
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

/// Sidebar do painel, escura nos dois modos: rio de 1 px ligando pontos de luz que perdem força com a distância
/// da página aberta; a atual leva gradiente metal, filete de 3 px e o óbolo com halo. `selected` é o rótulo ativo.
/// Com `on_select`, cada item responde a clique e a Enter/Espaço; o estado é do chamador.
#[derive(IntoElement)]
pub struct Rail {
    id: ElementId,
    groups: Vec<RailGroup>,
    selected: Option<SharedString>,
    on_select: Option<SelectHandler>,
}

impl Rail {
    pub fn new(groups: impl IntoIterator<Item = RailGroup>) -> Self {
        Self {
            id: "rail".into(),
            groups: groups.into_iter().collect(),
            selected: None,
            on_select: None,
        }
    }

    /// Identificador do trilho; só precisa mudar se houver mais de um na janela.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    /// Chamado com o rótulo do item escolhido por clique ou teclado.
    pub fn on_select(mut self, f: impl Fn(&SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Rc::new(f));
        self
    }

    pub fn selected(mut self, label: impl Into<SharedString>) -> Self {
        self.selected = Some(label.into());
        self
    }
}

/// Opacidade do ponto de luz a `d` itens da página aberta (`.luz`: `max(.16, 1 − d·.17)`).
pub(crate) fn light_opacity(d: usize) -> f32 {
    (1.0 - d as f32 * 0.17).max(0.16)
}

impl RenderOnce for Rail {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let labels: Vec<SharedString> = self
            .groups
            .iter()
            .flat_map(|g| g.items.iter().map(|i| i.label.clone()))
            .collect();
        let current = self
            .selected
            .as_ref()
            .and_then(|s| labels.iter().position(|l| l == s));
        let mut nav = div().relative().flex().flex_col().child(
            div()
                .absolute()
                .top_0()
                .bottom_0()
                .left(px(27.0))
                .w(px(1.0))
                .bg(p.rail_mute.opacity(0.26)),
        );
        let mut index = 0usize;
        for group in self.groups {
            nav = nav.child(
                div()
                    .pl(px(40.0))
                    .pt(px(16.0))
                    .pb(px(6.0))
                    .text_color(p.rail_mute)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_size(px(size::MICRO))
                    .child(group.title.to_uppercase()),
            );
            for item in group.items {
                let on = current == Some(index);
                let mut row = div()
                    .id(ElementId::NamedInteger("rail-item".into(), index as u64))
                    .relative()
                    .flex()
                    .items_center()
                    .justify_between()
                    .h(px(control::MD))
                    .pl(px(40.0))
                    .pr(px(14.0));
                let light = if on {
                    row = row
                        .font_weight(FontWeight::SEMIBOLD)
                        .bg(linear_gradient(
                            90.0,
                            linear_color_stop(p.metal.opacity(0.24), 0.0),
                            linear_color_stop(p.metal.opacity(0.0), 0.9),
                        ))
                        .shadow(vec![BoxShadow {
                            color: p.metal,
                            offset: point(px(3.0), px(0.0)),
                            blur_radius: px(0.0),
                            spread_radius: px(0.0),
                            inset: true,
                        }]);
                    div()
                        .absolute()
                        .left(px(23.0))
                        .top(px((control::MD - 9.0) / 2.0))
                        .size(px(9.0))
                        .bg(p.metal)
                        .shadow(vec![
                            BoxShadow::new(px(0.0), px(0.0), p.metal.opacity(0.46))
                                .blur_radius(px(12.0))
                                .spread_radius(px(2.0)),
                        ])
                } else {
                    let d = current.map_or(usize::MAX, |c| c.abs_diff(index));
                    let opacity = if item.alive { 1.0 } else { light_opacity(d) };
                    div()
                        .absolute()
                        .left(px(24.0))
                        .top(px((control::MD - 7.0) / 2.0))
                        .size(px(7.0))
                        .bg(p.metal)
                        .opacity(opacity)
                };
                row = row.child(light);
                if let Some(f) = self.on_select.clone() {
                    let label = item.label.clone();
                    let ring = BoxShadow {
                        color: p.metal,
                        offset: point(px(0.0), px(0.0)),
                        blur_radius: px(0.0),
                        spread_radius: px(2.0),
                        inset: true,
                    };
                    row = row
                        .cursor_pointer()
                        .tab_stop(true)
                        .focus_visible(move |s| s.shadow(vec![ring.clone()]))
                        .on_click(move |_, window, cx| f(&label, window, cx));
                    if !on {
                        row = row.hover(move |s| s.bg(p.rail_accent));
                    }
                }
                index += 1;
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
                nav = nav.child(row);
            }
        }
        div()
            .id(self.id.clone())
            .flex()
            .flex_col()
            .py(px(8.0))
            .w(px(RAIL_WIDTH))
            .h_full()
            .flex_none()
            .overflow_hidden()
            .bg(p.rail)
            .text_color(p.rail_fg)
            .text_size(px(size::LABEL))
            .border_r_1()
            .border_color(p.rail_line)
            .child(nav)
    }
}

#[cfg(test)]
mod tests {
    use super::light_opacity;

    #[test]
    fn luz_perde_forca_com_a_distancia_e_para_no_piso() {
        assert!((light_opacity(0) - 1.0).abs() < 1e-6);
        assert!((light_opacity(2) - 0.66).abs() < 1e-6);
        assert!((light_opacity(9) - 0.16).abs() < 1e-6);
    }
}
