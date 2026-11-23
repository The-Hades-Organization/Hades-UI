use std::rc::Rc;

use gpui::prelude::FluentBuilder as _;
use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, px,
};

use crate::components::{Dot, IconButton, Kbd, button::focus_ring};
use crate::icons::{Icon, IconName};
use crate::theme::palette;
use crate::tokens::{FONT_MONO, Palette, TOP_BAR_HEIGHT, Tone, size};

type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

struct Status {
    tone: Tone,
    label: SharedString,
    detail: Option<SharedString>,
}

struct Live {
    tone: Tone,
    label: SharedString,
    value: Option<SharedString>,
}

struct Search {
    placeholder: SharedString,
    shortcut: SharedString,
    on_click: Option<ClickHandler>,
}

struct ThemeToggle {
    dark: bool,
    on_toggle: ClickHandler,
}

/// Barra de topo da janela (`header` do painel): h 48, fundo da página a 90%, linha de base de 2 px `--primary`.
///
/// Slots, todos opcionais, na ordem do painel: estado à esquerda (`status`: ponto, rótulo e detalhe), título no meio,
/// e à direita o indicador ao vivo (`live`), o campo de busca (`search`, que o app liga à sua ação e ao atalho), as
/// ações (`action`), o botão de tema (`theme_toggle`) e o texto de `trailing`.
#[derive(IntoElement)]
pub struct TopBar {
    title: SharedString,
    trailing: Option<SharedString>,
    status: Option<Status>,
    live: Option<Live>,
    search: Option<Search>,
    actions: Vec<AnyElement>,
    theme: Option<ThemeToggle>,
}

impl TopBar {
    /// `title` pode ser vazio: o painel não mostra título no topo.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            trailing: None,
            status: None,
            live: None,
            search: None,
            actions: Vec::new(),
            theme: None,
        }
    }

    /// Texto mono no canto direito, depois de tudo.
    pub fn trailing(mut self, text: impl Into<SharedString>) -> Self {
        self.trailing = Some(text.into());
        self
    }

    /// Estado do ambiente à esquerda (`StateLine`): ponto no tom e rótulo em `foreground`.
    pub fn status(mut self, tone: Tone, label: impl Into<SharedString>) -> Self {
        self.status = Some(Status {
            tone,
            label: label.into(),
            detail: self.status.and_then(|s| s.detail),
        });
        self
    }

    /// Complemento do estado, em `muted-foreground` depois de " · " (ex.: "1 rodando").
    pub fn status_detail(mut self, detail: impl Into<SharedString>) -> Self {
        let detail = Some(detail.into());
        match self.status.as_mut() {
            Some(s) => s.detail = detail,
            None => {
                self.status = Some(Status {
                    tone: Tone::Neutral,
                    label: SharedString::default(),
                    detail,
                })
            }
        }
        self
    }

    /// Indicador ao vivo (`LiveIndicator`): ponto, rótulo e, se houver, o valor em mono (a hora da última leitura).
    pub fn live(
        mut self,
        tone: Tone,
        label: impl Into<SharedString>,
        value: Option<SharedString>,
    ) -> Self {
        self.live = Some(Live {
            tone,
            label: label.into(),
            value,
        });
        self
    }

    /// Campo de busca de 240 px com o atalho à direita (ex.: "Buscar ou ir para…", "Ctrl K"). O clique chama
    /// `on_search`; o atalho de teclado é do app.
    pub fn search(
        mut self,
        placeholder: impl Into<SharedString>,
        shortcut: impl Into<SharedString>,
    ) -> Self {
        self.search = Some(Search {
            placeholder: placeholder.into(),
            shortcut: shortcut.into(),
            on_click: self.search.and_then(|s| s.on_click),
        });
        self
    }

    pub fn on_search(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        let handler: ClickHandler = Rc::new(f);
        match self.search.as_mut() {
            Some(s) => s.on_click = Some(handler),
            None => {
                self.search = Some(Search {
                    placeholder: "Buscar ou ir para…".into(),
                    shortcut: "Ctrl K".into(),
                    on_click: Some(handler),
                })
            }
        }
        self
    }

    /// Elemento extra à direita, antes do botão de tema (em geral um `IconButton`).
    pub fn action(mut self, element: impl IntoElement) -> Self {
        self.actions.push(element.into_any_element());
        self
    }

    /// Botão de tema: sol no claro, lua no escuro, como o `ThemeToggle` do painel. `dark` é o modo atual.
    pub fn theme_toggle(
        mut self,
        dark: bool,
        on_toggle: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.theme = Some(ThemeToggle {
            dark,
            on_toggle: Rc::new(on_toggle),
        });
        self
    }
}

fn search_field(search: Search, p: &Palette) -> impl IntoElement {
    let hover = p.hover;
    let fg = p.fg;
    let ring = focus_ring(p, p.bg);
    div()
        .id(ElementId::Name("hades-ui-busca".into()))
        .flex()
        .flex_none()
        .items_center()
        .gap(px(8.0))
        .h(px(32.0))
        .w(px(240.0))
        .px(px(10.0))
        .bg(p.popover)
        .text_color(p.muted_fg)
        .text_size(px(size::LABEL))
        .shadow(vec![Palette::baseline(p.line, 2.0)])
        .cursor_pointer()
        .tab_stop(true)
        .hover(move |s| s.bg(hover).text_color(fg))
        .focus_visible(move |s| s.shadow(ring.clone()))
        .child(Icon::new(IconName::Search).size(px(14.0)))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .truncate()
                .child(search.placeholder),
        )
        .child(Kbd::new(search.shortcut))
        .when_some(search.on_click, |el, f| {
            el.on_click(move |e, window, cx| f(e, window, cx))
        })
}

impl RenderOnce for TopBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let mut bar = div()
            .flex()
            .items_center()
            .gap(px(12.0))
            .px(px(16.0))
            .h(px(TOP_BAR_HEIGHT))
            .flex_none()
            .bg(p.bg.opacity(0.9))
            .text_color(p.muted_fg)
            .shadow(vec![Palette::baseline(p.primary, 2.0)])
            .text_size(px(size::LABEL));
        if let Some(status) = self.status {
            let mut line = div()
                .flex()
                .flex_none()
                .items_center()
                .gap(px(8.0))
                .child(Dot::new("hades-ui-estado", status.tone).pulse(false));
            if !status.label.is_empty() {
                line = line.child(div().text_color(p.fg).child(status.label));
            }
            if let Some(detail) = status.detail {
                line = line.child(div().child(format!("· {detail}")));
            }
            bar = bar.child(line);
        }
        let title = div()
            .flex_1()
            .min_w_0()
            .flex()
            .justify_center()
            .font_family(FONT_MONO)
            .text_size(px(size::META))
            .when(!self.title.is_empty(), |t| {
                t.child(div().truncate().child(self.title))
            });
        bar = bar.child(title);
        if let Some(live) = self.live {
            bar = bar.child(
                div()
                    .flex()
                    .flex_none()
                    .items_center()
                    .gap(px(8.0))
                    .text_size(px(size::META))
                    .child(Dot::new("hades-ui-ao-vivo", live.tone).pulse(false))
                    .child(live.label)
                    .when_some(live.value, |l, v| {
                        l.child(div().font_family(FONT_MONO).text_color(p.fg).child(v))
                    }),
            );
        }
        let mut right = div().flex().flex_none().items_center().gap(px(8.0));
        if let Some(search) = self.search {
            right = right.child(search_field(search, &p));
        }
        right = right.children(self.actions);
        if let Some(theme) = self.theme {
            let (icon, label) = if theme.dark {
                (IconName::Moon, "Mudar para o tema claro")
            } else {
                (IconName::Sun, "Mudar para o tema escuro")
            };
            let f = theme.on_toggle;
            right = right.child(
                IconButton::new("hades-ui-tema", icon, label)
                    .on_click(move |e, window, cx| f(e, window, cx)),
            );
        }
        if let Some(trailing) = self.trailing {
            right = right.child(
                div()
                    .font_family(FONT_MONO)
                    .text_size(px(size::META))
                    .child(trailing),
            );
        }
        bar.child(right)
    }
}
