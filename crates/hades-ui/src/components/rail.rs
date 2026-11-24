use std::rc::Rc;

use gpui::{
    AnyElement, App, BoxShadow, ElementId, FontWeight, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div,
    linear_color_stop, linear_gradient, point, prelude::FluentBuilder, px,
};

use super::{IconButton, ThemeToggle, Tracked};
use crate::icons::{Icon, IconName, brand_mark};
use crate::theme::palette;
use crate::tokens::{
    FONT_HEADING, FONT_MONO, RAIL_WIDTH, RAIL_WIDTH_COLLAPSED, Tone, control, size,
};

type SelectHandler = Rc<dyn Fn(&SharedString, &mut Window, &mut App)>;
type CollapseHandler = Rc<dyn Fn(bool, &mut Window, &mut App)>;

/// Item da navegação lateral, com contador opcional.
pub struct RailItem {
    label: SharedString,
    badge: Option<SharedString>,
    alive: bool,
    icon: Option<IconName>,
    subtitle: Option<SharedString>,
}

impl RailItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            badge: None,
            alive: false,
            icon: None,
            subtitle: None,
        }
    }

    /// Ícone de 15 px antes do rótulo: `rail-muted`, ou metal no item atual.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Página com atividade: o ponto de luz acende inteiro, longe ou perto da atual.
    pub fn alive(mut self, alive: bool) -> Self {
        self.alive = alive;
        self
    }

    /// Contador à direita, em formato livre (`"3"`, `"0/1"`, `"on"`): mono 11 px em `rail-muted`, ou metal se o
    /// item está `alive`. Recolhido, vai para a dica do ícone ("Serviços · 0/1").
    pub fn badge(mut self, badge: impl Into<SharedString>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    /// Contador "feitos/total" (ex.: `count(0, 1)` mostra "0/1").
    pub fn count(self, done: usize, total: usize) -> Self {
        self.badge(format!("{done}/{total}"))
    }

    /// Segunda linha sob o rótulo, em `rail-muted` a 11 px (ex.: "terminais e agentes").
    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = Some(subtitle.into());
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

/// Glifo da [`RailBrand`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BrandGlyph {
    Hades,
    Icon(IconName),
}

/// Marca no topo do [`Rail`] (`Brand` do painel): glifo de 32 px em metal, nome em Marcellus 20 px com
/// espaçamento de .22em e subtítulo em `rail-muted`.
pub struct RailBrand {
    glyph: BrandGlyph,
    name: SharedString,
    subtitle: Option<SharedString>,
}

impl RailBrand {
    /// Marca com o glifo do Hades; o nome vai em caixa alta. Passe o nome da ferramenta (ex.: `"Moirai"`), não "Hades".
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self {
            glyph: BrandGlyph::Hades,
            name: name.into(),
            subtitle: None,
        }
    }

    /// "HADES / skill hades", como no painel.
    pub fn hades() -> Self {
        Self::new("Hades").subtitle("skill hades")
    }

    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Troca o glifo do Hades por um ícone do conjunto (ex.: a figura da ferramenta).
    pub fn icon(mut self, icon: IconName) -> Self {
        self.glyph = BrandGlyph::Icon(icon);
        self
    }

    fn glyph(&self, p: &crate::tokens::Palette) -> AnyElement {
        match self.glyph {
            BrandGlyph::Hades => brand_mark(px(32.0), p.metal).into_any_element(),
            BrandGlyph::Icon(icon) => Icon::new(icon)
                .size(px(32.0))
                .color(p.metal)
                .into_any_element(),
        }
    }

    fn render(self, p: &crate::tokens::Palette) -> impl IntoElement {
        let glyph = match self.glyph {
            BrandGlyph::Hades => brand_mark(px(32.0), p.metal).into_any_element(),
            BrandGlyph::Icon(icon) => Icon::new(icon)
                .size(px(32.0))
                .color(p.metal)
                .into_any_element(),
        };
        let mut words = div().flex().flex_col().gap(px(4.0)).min_w_0().child(
            div()
                .font_family(FONT_HEADING)
                .line_height(px(20.0))
                .text_color(p.rail_fg)
                .child(Tracked::new(self.name.to_uppercase(), 20.0, 0.22)),
        );
        if let Some(subtitle) = self.subtitle {
            words = words.child(
                div()
                    .text_size(px(size::META))
                    .text_color(p.rail_mute)
                    .truncate()
                    .child(subtitle),
            );
        }
        div()
            .flex()
            .flex_1()
            .min_w_0()
            .items_center()
            .gap(px(12.0))
            .child(glyph)
            .child(words)
    }
}

/// Sidebar do painel, escura nos dois modos: rio de 1 px ligando pontos de luz que perdem força com a distância
/// da página aberta; a atual leva gradiente metal, filete de 3 px e o óbolo com halo. `selected` é o rótulo ativo.
/// Com `on_select`, cada item responde a clique e a Enter/Espaço; o estado é do chamador.
///
/// Recolhido (`collapsed`), o trilho fica com 64 px: só o glifo da marca, os ícones dos itens (o rótulo e o
/// contador vão para a dica) e o [`RailStat`] em forma curta. Com `on_collapse`, o botão de recolher aparece ao
/// lado da marca; o estado também é do chamador.
#[derive(IntoElement)]
pub struct Rail {
    id: ElementId,
    groups: Vec<RailGroup>,
    brand: Option<RailBrand>,
    selected: Option<SharedString>,
    on_select: Option<SelectHandler>,
    footer: Option<AnyElement>,
    stat: Option<RailStat>,
    collapsed: bool,
    on_collapse: Option<CollapseHandler>,
    theme_toggle: bool,
}

impl Rail {
    pub fn new(groups: impl IntoIterator<Item = RailGroup>) -> Self {
        Self {
            id: "rail".into(),
            groups: groups.into_iter().collect(),
            brand: None,
            selected: None,
            on_select: None,
            footer: None,
            stat: None,
            collapsed: false,
            on_collapse: None,
            theme_toggle: false,
        }
    }

    /// Bloco livre preso ao pé do trilho, abaixo dos grupos, com o respiro dos itens e o texto em `rail-muted`. Some
    /// quando o trilho está recolhido; para o estado de memória ou janela, prefira [`Rail::stat`], que tem forma curta.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    /// Bloco de estado do pé ("Memória do Hades"); recolhido, mostra só a porcentagem e a barra, com o resto na dica.
    pub fn stat(mut self, stat: RailStat) -> Self {
        self.stat = Some(stat);
        self
    }

    /// Trilho recolhido em 64 px, só com ícones.
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Mostra o botão de recolher/expandir; `f` recebe o estado pedido (`true` = recolher).
    pub fn on_collapse(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_collapse = Some(Rc::new(f));
        self
    }

    /// Põe o [`ThemeToggle`](super::ThemeToggle) no pé do trilho, para app sem barra superior.
    pub fn theme_toggle(mut self) -> Self {
        self.theme_toggle = true;
        self
    }

    /// Marca no topo, acima dos grupos.
    pub fn brand(mut self, brand: RailBrand) -> Self {
        self.brand = Some(brand);
        self
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

/// Bloco de estado do pé do [`Rail`] (o "Memória do Hades" do painel): rótulo em `rail-muted` e valor em mono
/// `rail-foreground` na mesma linha, e embaixo, se houver `fraction`, a barra de 3 px sobre `rail-foreground` a 10%.
#[derive(IntoElement)]
pub struct RailStat {
    label: SharedString,
    value: SharedString,
    fraction: Option<f32>,
    tone: Tone,
}

impl RailStat {
    pub fn new(label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            fraction: None,
            tone: Tone::Ok,
        }
    }

    /// Fração entre 0 e 1 para a barra.
    pub fn fraction(mut self, fraction: f32) -> Self {
        self.fraction = Some(fraction.clamp(0.0, 1.0));
        self
    }

    /// Cor da barra; `Neutral` usa a cor de texto do trilho.
    pub fn tone(mut self, tone: Tone) -> Self {
        self.tone = tone;
        self
    }
}

impl RailStat {
    /// Forma curta do trilho recolhido: porcentagem em mono e barra de 32 px, com rótulo e valor na dica.
    fn compact(self, id: impl Into<ElementId>, p: &crate::tokens::Palette) -> AnyElement {
        let fill = if self.tone == Tone::Neutral {
            p.rail_fg
        } else {
            p.tone(self.tone)
        };
        let tip: SharedString = format!("{} · {}", self.label, self.value).into();
        let mut block = div()
            .id(id.into())
            .flex()
            .flex_none()
            .flex_col()
            .items_center()
            .gap(px(4.0))
            .mb(px(16.0))
            .mt(px(8.0))
            .tooltip(super::Tip::text(tip));
        match self.fraction {
            Some(f) => {
                block = block
                    .child(
                        div()
                            .font_family(FONT_MONO)
                            .text_size(px(size::MICRO))
                            .text_color(p.rail_mute)
                            .child(format!("{}%", (f * 100.0).round() as u32)),
                    )
                    .child(
                        div()
                            .h(px(3.0))
                            .w(px(32.0))
                            .bg(p.rail_fg.opacity(0.1))
                            .child(div().h_full().w(gpui::relative(f)).bg(fill)),
                    );
            }
            None => {
                block = block.child(
                    div()
                        .font_family(FONT_MONO)
                        .text_size(px(size::MICRO))
                        .text_color(p.rail_mute)
                        .child(self.value),
                );
            }
        }
        block.into_any_element()
    }
}

impl RenderOnce for RailStat {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let fill = if self.tone == Tone::Neutral {
            p.rail_fg
        } else {
            p.tone(self.tone)
        };
        div()
            .flex()
            .flex_none()
            .flex_col()
            .px(px(16.0))
            .py(px(12.0))
            .border_t_1()
            .border_color(p.rail_line)
            .text_size(px(size::META))
            .text_color(p.rail_mute)
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .items_baseline()
                    .justify_between()
                    .gap_x(px(8.0))
                    .child(div().whitespace_nowrap().child(self.label))
                    .child(
                        div()
                            .whitespace_nowrap()
                            .font_family(FONT_MONO)
                            .text_color(p.rail_fg)
                            .child(self.value),
                    ),
            )
            .when_some(self.fraction, |b, f| {
                b.child(
                    div()
                        .mt(px(8.0))
                        .h(px(3.0))
                        .bg(p.rail_fg.opacity(0.1))
                        .child(div().h_full().w(gpui::relative(f)).bg(fill)),
                )
            })
    }
}

/// Opacidade do ponto de luz a `d` itens da página aberta (`.luz`: `max(.16, 1 − d·.17)`).
pub(crate) fn light_opacity(d: usize) -> f32 {
    (1.0 - d as f32 * 0.17).max(0.16)
}

/// Navegação aberta: rio, pontos de luz, títulos de grupo e itens com rótulo e contador.
fn expanded_nav(
    groups: Vec<RailGroup>,
    current: Option<usize>,
    on_select: Option<SelectHandler>,
    p: &crate::tokens::Palette,
) -> gpui::Div {
    let p = *p;
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
    for group in groups {
        nav = nav.child(
            div()
                .pl(px(40.0))
                .pt(px(16.0))
                .pb(px(6.0))
                .text_color(p.rail_mute)
                .font_weight(FontWeight::SEMIBOLD)
                .child(Tracked::new(group.title.to_uppercase(), size::MICRO, 0.14)),
        );
        for item in group.items {
            let on = current == Some(index);
            let mut row = div()
                .id(ElementId::NamedInteger("rail-item".into(), index as u64))
                .relative()
                .flex()
                .items_center()
                .justify_between()
                .min_h(px(control::MD))
                .when(item.subtitle.is_some(), |r| r.py(px(4.0)))
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
            if let Some(f) = on_select.clone() {
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
            let label = div()
                .flex()
                .flex_1()
                .min_w_0()
                .items_center()
                .gap(px(10.0))
                .when_some(item.icon, |l, icon| {
                    l.child(Icon::new(icon).size(px(15.0)).color(if on {
                        p.metal
                    } else {
                        p.rail_mute
                    }))
                })
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .min_w_0()
                        .child(div().truncate().child(item.label))
                        .when_some(item.subtitle, |c, sub| {
                            c.child(
                                div()
                                    .truncate()
                                    .text_color(p.rail_mute)
                                    .text_size(px(size::MICRO))
                                    .child(sub),
                            )
                        }),
                );
            row = row.child(label);
            if let Some(badge) = item.badge {
                row = row.child(
                    div()
                        .text_color(if item.alive {
                            p.metal
                        } else {
                            p.rail_mute.opacity(0.8)
                        })
                        .font_family(FONT_MONO)
                        .text_size(px(size::MICRO))
                        .child(badge),
                );
            }
            nav = nav.child(row);
        }
    }
    nav
}

/// Navegação recolhida (`railItem` do painel): quadrado de 36 px por item com o ícone, ponto de luz de 5 px à
/// esquerda, filete entre grupos e dica com rótulo e contador.
fn collapsed_nav(
    groups: Vec<RailGroup>,
    current: Option<usize>,
    on_select: Option<SelectHandler>,
    p: &crate::tokens::Palette,
) -> gpui::Div {
    let p = *p;
    let mut nav = div()
        .flex()
        .flex_col()
        .items_center()
        .gap(px(2.0))
        .pb(px(16.0));
    let mut index = 0usize;
    for (gi, group) in groups.into_iter().enumerate() {
        if gi > 0 {
            nav = nav.child(div().my(px(6.0)).h(px(1.0)).w(px(24.0)).bg(p.rail_line));
        }
        for item in group.items {
            let on = current == Some(index);
            let d = current.map_or(usize::MAX, |c| c.abs_diff(index));
            let opacity = if item.alive || on {
                1.0
            } else {
                light_opacity(d)
            };
            let tip: SharedString = match &item.badge {
                Some(b) => format!("{} · {b}", item.label).into(),
                None => item.label.clone(),
            };
            let mut cell = div()
                .id(ElementId::NamedInteger("rail-item".into(), index as u64))
                .relative()
                .flex()
                .items_center()
                .justify_center()
                .size(px(36.0))
                .tooltip(super::Tip::text(tip))
                .child(
                    div()
                        .absolute()
                        .left(px(-10.0))
                        .top(px(18.0 - 2.5))
                        .size(px(5.0))
                        .bg(p.metal)
                        .opacity(opacity),
                )
                .child(
                    Icon::new(item.icon.unwrap_or(IconName::CircleDashed))
                        .size(px(16.0))
                        .color(if on { p.metal } else { p.rail_mute }),
                );
            if on {
                cell = cell
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
            }
            if let Some(f) = on_select.clone() {
                let label = item.label.clone();
                let metal = p.metal;
                cell = cell
                    .cursor_pointer()
                    .tab_stop(true)
                    .focus_visible(move |s| {
                        s.shadow(vec![BoxShadow {
                            color: metal,
                            offset: point(px(0.0), px(0.0)),
                            blur_radius: px(0.0),
                            spread_radius: px(2.0),
                            inset: true,
                        }])
                    })
                    .on_click(move |_, window, cx| f(&label, window, cx));
                if !on {
                    let accent = p.rail_accent;
                    cell = cell.hover(move |s| s.bg(accent));
                }
            }
            index += 1;
            nav = nav.child(cell);
        }
    }
    nav
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
        let collapsed = self.collapsed;
        let toggle = self.on_collapse.map(|f| {
            let (icon, label) = if collapsed {
                (IconName::PanelLeftOpen, "Expandir a navegação")
            } else {
                (IconName::PanelLeftClose, "Recolher a navegação")
            };
            IconButton::new((self.id.clone(), "recolher"), icon, label)
                .on_rail()
                .on_click(move |_, window, cx| f(!collapsed, window, cx))
        });
        let head = if collapsed {
            div()
                .flex()
                .flex_none()
                .flex_col()
                .items_center()
                .gap(px(8.0))
                .pt(px(8.0))
                .pb(px(12.0))
                .when_some(self.brand.as_ref(), |h, b| h.child(b.glyph(&p)))
                .children(toggle)
        } else {
            div()
                .flex()
                .flex_none()
                .items_center()
                .justify_between()
                .gap(px(8.0))
                .px(px(16.0))
                .pt(px(12.0))
                .pb(px(8.0))
                .when_some(self.brand, |h, b| h.child(b.render(&p)))
                .children(toggle)
        };
        let nav = if collapsed {
            collapsed_nav(self.groups, current, self.on_select, &p)
        } else {
            expanded_nav(self.groups, current, self.on_select, &p)
        };
        let theme = self.theme_toggle.then(|| {
            div()
                .flex()
                .flex_none()
                .when(!collapsed, |d| d.px(px(8.0)))
                .when(collapsed, |d| d.justify_center())
                .py(px(4.0))
                .child(ThemeToggle::new().on_rail())
        });
        let id = self.id.clone();
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .pt(px(8.0))
            .w(px(if collapsed {
                RAIL_WIDTH_COLLAPSED
            } else {
                RAIL_WIDTH
            }))
            .h_full()
            .flex_none()
            .overflow_hidden()
            .bg(p.rail)
            .text_color(p.rail_fg)
            .text_size(px(size::LABEL))
            .border_r_1()
            .border_color(p.rail_line)
            .child(head)
            .child(
                div()
                    .id((id.clone(), "navegacao"))
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .child(nav.pb(px(16.0))),
            )
            .children(theme)
            .when_some(self.stat, |r, s| {
                if collapsed {
                    r.child(s.compact((id, "estado"), &p))
                } else {
                    r.child(s)
                }
            })
            .when(!collapsed, |r| {
                r.children(self.footer.map(|f| {
                    div()
                        .flex_none()
                        .px(px(16.0))
                        .pt(px(8.0))
                        .pb(px(12.0))
                        .text_color(p.rail_mute)
                        .child(f)
                }))
            })
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
