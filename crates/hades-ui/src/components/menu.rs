use std::rc::Rc;

use gpui::{
    Animation, AnimationExt, AnyElement, App, ClickEvent, ElementId, InteractiveElement,
    IntoElement, MouseButton, ParentElement, Pixels, Point, RenderOnce, SharedString,
    StatefulInteractiveElement, Styled, Window, anchored, deferred, div, px,
};

use crate::icons::{Icon, IconName};
use crate::theme::palette;
use crate::tokens::{Palette, motion, size};

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// Item de [`Menu`].
pub enum MenuEntry {
    Item {
        label: SharedString,
        danger: bool,
        disabled: bool,
        on_click: Option<ClickHandler>,
    },
    Separator,
}

impl MenuEntry {
    pub fn item(label: impl Into<SharedString>) -> Self {
        Self::Item {
            label: label.into(),
            danger: false,
            disabled: false,
            on_click: None,
        }
    }

    pub fn separator() -> Self {
        Self::Separator
    }

    /// Ação destrutiva: texto `bad`, hover `danger-soft`.
    pub fn danger(mut self) -> Self {
        if let Self::Item { danger, .. } = &mut self {
            *danger = true;
        }
        self
    }

    pub fn disabled(mut self, value: bool) -> Self {
        if let Self::Item { disabled, .. } = &mut self {
            *disabled = value;
        }
        self
    }

    pub fn on_click(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        if let Self::Item { on_click, .. } = &mut self {
            *on_click = Some(Rc::new(f));
        }
        self
    }
}

/// Painel de menu (`Menu` do painel): fundo `popover`, 4 px de respiro, linha de base `primary` e a sombra `--elev`,
/// entra com `fade-in`. Itens de 14 px com hover `hover`; perigoso em `bad`; desabilitado a 50%.
/// Para abrir num ponto, use [`ContextMenu`] ou ponha o menu num `anchored()` do gpui.
#[derive(IntoElement)]
pub struct Menu {
    id: ElementId,
    entries: Vec<MenuEntry>,
    icons: Vec<(usize, IconName)>,
}

impl Menu {
    pub fn new(id: impl Into<ElementId>, entries: impl IntoIterator<Item = MenuEntry>) -> Self {
        Self {
            id: id.into(),
            entries: entries.into_iter().collect(),
            icons: Vec::new(),
        }
    }

    /// Ícone de 16 px antes do rótulo da entrada `index` (contando separadores), na cor do texto do item.
    pub fn icon(mut self, index: usize, icon: IconName) -> Self {
        self.icons.retain(|(i, _)| *i != index);
        self.icons.push((index, icon));
        self
    }
}

fn icon_at(icons: &[(usize, IconName)], index: usize) -> Option<IconName> {
    icons
        .iter()
        .find(|(i, _)| *i == index)
        .map(|(_, icon)| *icon)
}

impl RenderOnce for Menu {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let mut panel = div()
            .id(self.id.clone())
            .flex()
            .flex_col()
            .min_w(px(192.0))
            .p(px(4.0))
            .pb(px(6.0))
            .bg(p.popover)
            .text_color(p.fg)
            .text_size(px(size::BODY))
            .shadow(vec![Palette::baseline(p.primary, 2.0), p.elevation()]);
        for (i, entry) in self.entries.into_iter().enumerate() {
            panel = panel.child(match entry {
                MenuEntry::Separator => div()
                    .mx(px(-4.0))
                    .my(px(4.0))
                    .h(px(1.0))
                    .bg(p.border)
                    .into_any_element(),
                MenuEntry::Item {
                    label,
                    danger,
                    disabled,
                    on_click,
                } => item(
                    &p,
                    i,
                    icon_at(&self.icons, i),
                    label,
                    danger,
                    disabled,
                    on_click,
                ),
            });
        }
        panel.with_animation(
            self.id,
            Animation::new(motion::FADE_IN).with_easing(motion::ease_out()),
            |el, t| el.opacity(t),
        )
    }
}

fn item(
    p: &Palette,
    i: usize,
    icon: Option<IconName>,
    label: SharedString,
    danger: bool,
    disabled: bool,
    on_click: Option<ClickHandler>,
) -> AnyElement {
    let (fg, hover) = if danger {
        (p.bad, p.danger_soft)
    } else {
        (p.fg, p.hover)
    };
    let mut row = div()
        .id(ElementId::NamedInteger("menu-item".into(), i as u64))
        .flex()
        .items_center()
        .gap(px(8.0))
        .px(px(8.0))
        .py(px(6.0))
        .text_color(fg);
    if let Some(icon) = icon {
        row = row.child(Icon::new(icon));
    }
    let row = row.child(label);
    if disabled {
        return row.opacity(0.5).into_any_element();
    }
    let row = row
        .cursor_pointer()
        .tab_stop(true)
        .hover(move |s| s.bg(hover))
        .focus_visible(move |s| s.bg(hover));
    match on_click {
        Some(f) => row
            .on_click(move |e, window, cx| f(e, window, cx))
            .into_any_element(),
        None => row.into_any_element(),
    }
}

type MenuBuilder = Rc<dyn Fn(&mut Window, &mut App) -> Vec<MenuEntry>>;

/// Menu de contexto: o clique direito no filho abre um [`Menu`] no ponteiro; clique fora ou numa ação fecha.
#[derive(IntoElement)]
pub struct ContextMenu {
    id: ElementId,
    child: AnyElement,
    entries: MenuBuilder,
    icons: Vec<(usize, IconName)>,
}

impl ContextMenu {
    pub fn new(
        id: impl Into<ElementId>,
        child: impl IntoElement,
        entries: impl Fn(&mut Window, &mut App) -> Vec<MenuEntry> + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            child: child.into_any_element(),
            entries: Rc::new(entries),
            icons: Vec::new(),
        }
    }

    /// Ícone da entrada `index` do menu aberto; ver [`Menu::icon`].
    pub fn icon(mut self, index: usize, icon: IconName) -> Self {
        self.icons.retain(|(i, _)| *i != index);
        self.icons.push((index, icon));
        self
    }
}

impl RenderOnce for ContextMenu {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let open = window.use_keyed_state(self.id.clone(), cx, |_, _| None::<Point<Pixels>>);
        let at = *open.read(cx);
        let set = open.clone();
        let mut root = div().id(self.id.clone()).child(self.child).on_mouse_down(
            MouseButton::Right,
            move |e, _, cx| {
                set.update(cx, |s, cx| {
                    *s = Some(e.position);
                    cx.notify();
                });
            },
        );
        if let Some(at) = at {
            let close = open.clone();
            let close_on_pick = open.clone();
            let entries = (self.entries)(window, cx).into_iter().map(|e| match e {
                MenuEntry::Item {
                    label,
                    danger,
                    disabled,
                    on_click,
                } => {
                    let close = close_on_pick.clone();
                    let action = on_click.clone();
                    MenuEntry::Item {
                        label,
                        danger,
                        disabled,
                        on_click: Some(Rc::new(move |ev, window, cx| {
                            close.update(cx, |s, cx| {
                                *s = None;
                                cx.notify();
                            });
                            if let Some(f) = &action {
                                f(ev, window, cx);
                            }
                        })),
                    }
                }
                MenuEntry::Separator => MenuEntry::Separator,
            });
            root = root.child(deferred(
                anchored().position(at).snap_to_window().child(
                    div()
                        .on_mouse_down_out(move |_, _, cx| {
                            close.update(cx, |s, cx| {
                                *s = None;
                                cx.notify();
                            });
                        })
                        .child(
                            self.icons
                                .into_iter()
                                .fold(Menu::new((self.id, "menu"), entries), |menu, (i, icon)| {
                                    menu.icon(i, icon)
                                }),
                        ),
                ),
            ));
        }
        root
    }
}
