use std::rc::Rc;

use gpui::{
    App, BoxShadow, ClickEvent, ElementId, FontWeight, Hsla, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div,
    pattern_slash, px,
};
use gpui_component::tooltip::Tooltip;

use super::Spinner;
use crate::icons::{Icon, IconName};
use crate::theme::palette;
use crate::tokens::{Palette, control, size};

/// Variante do botão (`BTN_V` do painel).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonKind {
    /// `secondary` (= `outline`): fundo `secondary`.
    #[default]
    Secondary,
    /// `default`: fundo `primary`.
    Primary,
    Ghost,
    /// `destructive`: fundo `danger-soft`, texto `bad`.
    Destructive,
    GhostDestructive,
    Link,
}

/// Tamanho do botão (`BTN_S` do painel).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ButtonSize {
    #[default]
    Md,
    Xs,
    Lg,
    /// Quadrado de 36 px, para botão só com ícone ou glifo.
    Icon,
}

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// Botão quadrado do painel (`Button` de `app.tsx`).
/// Desabilitado esmaece a 60% com hachura diagonal e mostra o motivo no tooltip; carregando mantém a cor e gira.
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    kind: ButtonKind,
    size: ButtonSize,
    icon: Option<IconName>,
    loading: bool,
    disabled_reason: Option<SharedString>,
    on_click: Option<ClickHandler>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind: ButtonKind::default(),
            size: ButtonSize::default(),
            icon: None,
            loading: false,
            disabled_reason: None,
            on_click: None,
        }
    }

    pub fn primary(self) -> Self {
        self.kind(ButtonKind::Primary)
    }

    pub fn kind(mut self, kind: ButtonKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Ícone antes do rótulo, na cor do texto do botão; carregando, o giro toma o lugar dele.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Em andamento: gira, não aceita clique e não esmaece.
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Desabilita o botão; o motivo aparece ao pairar o ponteiro.
    pub fn disabled(mut self, reason: impl Into<SharedString>) -> Self {
        self.disabled_reason = Some(reason.into());
        self
    }

    pub fn on_click(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Rc::new(f));
        self
    }
}

/// (fundo, texto, fundo no hover, texto no hover).
fn colors(kind: ButtonKind, p: &Palette) -> (Option<Hsla>, Hsla, Option<Hsla>, Hsla) {
    match kind {
        ButtonKind::Primary => (
            Some(p.primary),
            p.primary_fg,
            Some(p.primary_hover),
            p.primary_fg,
        ),
        ButtonKind::Secondary => (Some(p.muted), p.fg, Some(p.secondary_hover), p.fg),
        ButtonKind::Ghost => (None, p.fg, Some(p.hover), p.fg),
        ButtonKind::Destructive => (Some(p.danger_soft), p.bad, Some(p.danger_soft_hover), p.bad),
        ButtonKind::GhostDestructive => (None, p.bad, Some(p.danger_soft), p.bad),
        ButtonKind::Link => (None, p.fg, None, p.fg),
    }
}

/// Contorno de foco do painel (`outline-2 outline-offset-3 outline-ring`): duas sombras sem desfoque,
/// a de fora na cor do anel e a de dentro na do fundo, deixam o vão de 3 px.
pub(crate) fn focus_ring(p: &Palette, gap_color: Hsla) -> Vec<BoxShadow> {
    vec![
        BoxShadow::new(px(0.0), px(0.0), p.primary).spread_radius(px(5.0)),
        BoxShadow::new(px(0.0), px(0.0), gap_color).spread_radius(px(3.0)),
    ]
}

impl RenderOnce for Button {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let (bg, fg, hover_bg, hover_fg) = colors(self.kind, &p);
        let (h, pad_x, text) = match self.size {
            ButtonSize::Md => (control::MD, 14.0, size::LABEL),
            ButtonSize::Xs => (control::XS, 8.0, size::META),
            ButtonSize::Lg => (control::LG, 24.0, size::LABEL),
            ButtonSize::Icon => (control::ICON, 0.0, size::LABEL),
        };
        let mut base = div()
            .id(self.id.clone())
            .relative()
            .flex()
            .flex_none()
            .items_center()
            .justify_center()
            .gap(px(8.0))
            .h(px(h))
            .px(px(pad_x))
            .text_color(fg)
            .text_size(px(text))
            .font_weight(FontWeight::MEDIUM)
            .whitespace_nowrap();
        if self.size == ButtonSize::Icon {
            base = base.w(px(h));
        }
        if let Some(bg) = bg {
            base = base.bg(bg);
        }
        if let Some(icon) = self.icon.filter(|_| !self.loading) {
            let side = if self.size == ButtonSize::Xs {
                14.0
            } else {
                16.0
            };
            base = base.child(Icon::new(icon).size(px(side)));
        }
        if self.kind == ButtonKind::Link {
            let underline_hover = p.primary;
            base = base.group("hades-ui-link").child(
                div()
                    .border_b_1()
                    .border_color(p.line)
                    .group_hover("hades-ui-link", move |s| s.border_color(underline_hover))
                    .child(self.label.clone()),
            );
        } else {
            base = base.child(self.label.clone());
        }
        if self.loading {
            return base
                .child(Spinner::new((self.id, "giro")).size(14.0).color(fg))
                .into_any_element();
        }
        if let Some(reason) = self.disabled_reason {
            return base
                .opacity(0.6)
                .cursor_not_allowed()
                .child(
                    div()
                        .absolute()
                        .inset_0()
                        .bg(pattern_slash(p.fg.opacity(0.16), 1.0, 5.0)),
                )
                .tooltip(move |window, cx| Tooltip::new(reason.clone()).build(window, cx))
                .into_any_element();
        }
        let ring = focus_ring(&p, p.bg);
        let base = base
            .cursor_pointer()
            .tab_stop(true)
            .hover(move |s| {
                let s = s.text_color(hover_fg);
                match hover_bg {
                    Some(bg) => s.bg(bg),
                    None => s,
                }
            })
            .active(|s| s.mt(px(1.0)).mb(px(-1.0)))
            .focus_visible(move |s| s.shadow(ring.clone()));
        match self.on_click {
            Some(f) => base
                .on_click(move |e, window, cx| f(e, window, cx))
                .into_any_element(),
            None => base.into_any_element(),
        }
    }
}
