use std::rc::Rc;

use gpui::{
    App, ElementId, FontWeight, InteractiveElement, IntoElement, KeyDownEvent, ParentElement,
    RenderOnce, ScrollHandle, SharedString, StatefulInteractiveElement, Styled, Window, div, px,
};

use super::Kbd;
use crate::embed::Comando;
use crate::icons::{Icon, IconName};
use crate::theme::palette;
use crate::tokens::{Palette, size};

type PickHandler = Rc<dyn Fn(&SharedString, &mut Window, &mut App)>;
type CloseHandler = Rc<dyn Fn(&mut Window, &mut App)>;

/// Largura da caixa (`sm:max-w-xl`), em px.
const PALETTE_WIDTH: f32 = 576.0;
/// Altura máxima da lista antes de rolar (`max-h-[360px]`).
const LIST_MAX_HEIGHT: f32 = 360.0;
/// Topo fixo (`top-[12vh]`): centrada pela própria altura, a caixa pularia a cada letra que filtra a lista.
const TOP_FRACTION: f32 = 0.12;

/// Um item da paleta: grupo (cabeçalho), rótulo, descrição que também entra na busca, ícone e atalho.
#[derive(Clone, Debug, PartialEq)]
pub struct PaletteItem {
    pub id: SharedString,
    pub group: SharedString,
    pub label: SharedString,
    pub desc: Option<SharedString>,
    pub icon: IconName,
    pub shortcut: Option<SharedString>,
}

impl PaletteItem {
    pub fn new(
        id: impl Into<SharedString>,
        group: impl Into<SharedString>,
        label: impl Into<SharedString>,
        icon: IconName,
    ) -> Self {
        Self {
            id: id.into(),
            group: group.into(),
            label: label.into(),
            desc: None,
            icon,
            shortcut: None,
        }
    }

    pub fn desc(mut self, desc: impl Into<SharedString>) -> Self {
        self.desc = Some(desc.into());
        self
    }

    pub fn shortcut(mut self, keys: impl Into<SharedString>) -> Self {
        self.shortcut = Some(keys.into());
        self
    }
}

impl From<&Comando> for PaletteItem {
    fn from(c: &Comando) -> Self {
        Self {
            id: c.id.into(),
            group: c.grupo.into(),
            label: c.rotulo.into(),
            desc: c.descricao.map(Into::into),
            icon: c.icone,
            shortcut: c.atalho.map(Into::into),
        }
    }
}

/// Minúsculas sem acento, para a busca casar "execucao" com "Execução".
///
/// Porta `cfgNorm` (`kit/panel/ui/app.tsx:1692`): NFD sem as marcas combinantes, em minúsculas. Sem a tabela
/// Unicode inteira, cobre as letras latinas acentuadas; o resto passa só pelo `to_lowercase`.
pub fn fold_text(text: &str) -> String {
    text.chars()
        .flat_map(char::to_lowercase)
        .filter(|c| !('\u{300}'..='\u{36f}').contains(c))
        .map(|c| match c {
            'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => 'a',
            'ç' => 'c',
            'è' | 'é' | 'ê' | 'ë' => 'e',
            'ì' | 'í' | 'î' | 'ï' => 'i',
            'ñ' => 'n',
            'ò' | 'ó' | 'ô' | 'õ' | 'ö' => 'o',
            'ù' | 'ú' | 'û' | 'ü' => 'u',
            'ý' | 'ÿ' => 'y',
            c => c,
        })
        .collect()
}

/// Índices dos itens que casam com `query`, na ordem da lista: cada termo (separado por espaço) aparece em
/// "grupo rótulo descrição", sem caixa nem acento. Busca vazia devolve todos.
///
/// Porta o filtro de `CommandPalette` (`kit/panel/ui/app.tsx:1763-1764`).
pub fn filter_palette(items: &[PaletteItem], query: &str) -> Vec<usize> {
    let folded = fold_text(query);
    let terms: Vec<&str> = folded.split_whitespace().collect();
    items
        .iter()
        .enumerate()
        .filter(|(_, i)| {
            if terms.is_empty() {
                return true;
            }
            let hay = fold_text(&format!(
                "{} {} {}",
                i.group,
                i.label,
                i.desc.as_deref().unwrap_or("")
            ));
            terms.iter().all(|t| hay.contains(t))
        })
        .map(|(ix, _)| ix)
        .collect()
}

/// O que uma tecla fez na paleta.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaletteKey {
    /// A tecla mudou a busca ou a seleção.
    Handled,
    /// Enter sobre o item `n` da lista filtrada.
    Pick(usize),
    Close,
    /// A tecla não é da paleta (atalho com Ctrl, tecla de função): deixe passar.
    Ignored,
}

/// Busca e item marcado da paleta. Puro: o componente guarda um por paleta aberta e passa cada tecla.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PaletteState {
    pub query: String,
    pub selected: usize,
}

impl PaletteState {
    /// Aplica a tecla `key` (nome do gpui), com o texto que ela digita em `key_char`, sobre uma lista filtrada de
    /// `shown` itens. Setas param nas pontas; digitar ou apagar volta a marca ao primeiro item.
    ///
    /// Porta `key` e o `onInput` de `CommandPalette` (`kit/panel/ui/app.tsx:1766-1770`, `:1782`).
    pub fn key(
        &mut self,
        key: &str,
        key_char: Option<&str>,
        modifier: bool,
        shown: usize,
    ) -> PaletteKey {
        match key {
            "escape" => return PaletteKey::Close,
            "up" => {
                self.selected = self.selected.saturating_sub(1);
                return PaletteKey::Handled;
            }
            "down" => {
                self.selected = (self.selected + 1).min(shown.saturating_sub(1));
                return PaletteKey::Handled;
            }
            "enter" => {
                return if self.selected < shown {
                    PaletteKey::Pick(self.selected)
                } else {
                    PaletteKey::Handled
                };
            }
            _ => {}
        }
        if modifier {
            return PaletteKey::Ignored;
        }
        if key == "backspace" {
            self.query.pop();
            self.selected = 0;
            return PaletteKey::Handled;
        }
        match key_char {
            Some(text) if !text.is_empty() && !text.chars().any(char::is_control) => {
                self.query.push_str(text);
                self.selected = 0;
                PaletteKey::Handled
            }
            _ => PaletteKey::Ignored,
        }
    }
}

/// Paleta de comandos (Ctrl K): busca por texto sobre a lista, itens agrupados com ícone e atalho; setas movem a
/// marca, Enter escolhe, Esc e o clique fora fecham. Renderize-a por último, só enquanto estiver aberta, como o
/// [`super::Sheet`]; cada abertura começa com a busca vazia. Abrir com Ctrl K é do chamador.
#[derive(IntoElement)]
pub struct CommandPalette {
    id: ElementId,
    items: Vec<PaletteItem>,
    placeholder: SharedString,
    on_pick: Option<PickHandler>,
    on_close: Option<CloseHandler>,
}

impl CommandPalette {
    pub fn new(id: impl Into<ElementId>, items: impl IntoIterator<Item = PaletteItem>) -> Self {
        Self {
            id: id.into(),
            items: items.into_iter().collect(),
            placeholder: "Buscar ação, serviço ou página…".into(),
            on_pick: None,
            on_close: None,
        }
    }

    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Recebe o id do item escolhido, depois do `on_close`.
    pub fn on_pick(mut self, f: impl Fn(&SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_pick = Some(Rc::new(f));
        self
    }

    pub fn on_close(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Rc::new(f));
        self
    }
}

fn pick(
    id: &SharedString,
    on_close: &Option<CloseHandler>,
    on_pick: &Option<PickHandler>,
    window: &mut Window,
    cx: &mut App,
) {
    if let Some(f) = on_close {
        f(window, cx);
    }
    if let Some(f) = on_pick {
        f(id, window, cx);
    }
}

fn hint(keys: &[&'static str], label: &'static str) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .gap(px(4.0))
        .children(keys.iter().map(|k| Kbd::new(*k)))
        .child(label)
}

impl RenderOnce for CommandPalette {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let state = window.use_keyed_state((self.id.clone(), "estado"), cx, |_, _| {
            PaletteState::default()
        });
        let focus = window
            .use_keyed_state((self.id.clone(), "foco"), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let scroll = window
            .use_keyed_state((self.id.clone(), "rolagem"), cx, |_, _| ScrollHandle::new())
            .read(cx)
            .clone();
        if !focus.contains_focused(window, cx) {
            let f = focus.clone();
            window.defer(cx, move |window, cx| window.focus(&f, cx));
        }
        let PaletteState { query, selected } = state.read(cx).clone();
        let shown = filter_palette(&self.items, &query);
        let shown_ids: Rc<Vec<SharedString>> =
            Rc::new(shown.iter().map(|&i| self.items[i].id.clone()).collect());

        let mut rows = Vec::new();
        let mut row_of_selected = 0;
        let mut last_group: Option<&SharedString> = None;
        for (n, &ix) in shown.iter().enumerate() {
            let item = &self.items[ix];
            if last_group != Some(&item.group) {
                last_group = Some(&item.group);
                rows.push(
                    div()
                        .px(px(8.0))
                        .pt(px(12.0))
                        .pb(px(4.0))
                        .text_size(px(size::MICRO))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(p.muted_fg)
                        .child(item.group.to_uppercase())
                        .into_any_element(),
                );
            }
            let active = n == selected;
            if active {
                row_of_selected = rows.len();
            }
            let mut row = div()
                .id((self.id.clone(), SharedString::from(format!("item-{n}"))))
                .flex()
                .items_center()
                .gap(px(8.0))
                .px(px(8.0))
                .py(px(6.0))
                .cursor_pointer()
                .text_size(px(size::BODY))
                .child(Icon::new(item.icon).size(px(16.0)).color(if active {
                    p.fg
                } else {
                    p.muted_fg
                }))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .truncate()
                        .child(item.label.clone()),
                )
                .on_mouse_move({
                    let state = state.clone();
                    move |_, _, cx| {
                        state.update(cx, |s, cx| {
                            if s.selected != n {
                                s.selected = n;
                                cx.notify();
                            }
                        })
                    }
                })
                .on_click({
                    let id = item.id.clone();
                    let on_close = self.on_close.clone();
                    let on_pick = self.on_pick.clone();
                    move |_, w, cx| pick(&id, &on_close, &on_pick, w, cx)
                });
            if let Some(keys) = item.shortcut.clone() {
                row = row.child(Kbd::new(keys));
            }
            if active {
                row = row
                    .bg(p.accent)
                    .child(Icon::new(IconName::CornerDownLeft).size(px(14.0)));
            }
            rows.push(row.into_any_element());
        }
        if !shown.is_empty() {
            scroll.scroll_to_item(row_of_selected);
        }

        let field = div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .h(px(48.0))
            .pl(px(12.0))
            .pr(px(12.0))
            .shadow(vec![Palette::baseline(p.primary, 2.0)])
            .child(Icon::new(IconName::Search).color(p.muted_fg))
            .child(if query.is_empty() {
                div()
                    .flex()
                    .items_center()
                    .child(div().w(px(1.5)).h(px(18.0)).bg(p.fg))
                    .child(div().text_color(p.muted_fg).child(self.placeholder))
            } else {
                div()
                    .flex()
                    .items_center()
                    .child(query.clone())
                    .child(div().w(px(1.5)).h(px(18.0)).bg(p.fg))
            });

        let list = div()
            .id((self.id.clone(), "lista"))
            .track_scroll(&scroll)
            .max_h(px(LIST_MAX_HEIGHT))
            .overflow_y_scroll()
            .p(px(4.0))
            .flex()
            .flex_col()
            .children(rows)
            .when_empty(shown.is_empty(), p);

        let footer = div()
            .flex()
            .items_center()
            .gap(px(12.0))
            .px(px(12.0))
            .py(px(8.0))
            .border_t_1()
            .border_color(p.border)
            .text_size(px(size::META))
            .text_color(p.muted_fg)
            .child(hint(&["↑", "↓"], "navegar"))
            .child(hint(&["↵"], "executar"))
            .child(hint(&["Esc"], "fechar"));

        let width = PALETTE_WIDTH.min(f32::from(window.viewport_size().width) - 24.0);
        let top = f32::from(window.viewport_size().height) * TOP_FRACTION;
        let body = div()
            .id((self.id.clone(), "caixa"))
            .absolute()
            .top(px(top))
            .left_0()
            .right_0()
            .mx_auto()
            .w(px(width))
            .flex()
            .flex_col()
            .bg(p.card)
            .text_color(p.fg)
            .shadow(vec![p.dialog_shadow()])
            .occlude()
            .on_click(|_, _, cx| cx.stop_propagation())
            .child(field)
            .child(list)
            .child(footer);

        let on_key = {
            let state = state.clone();
            let on_close = self.on_close.clone();
            let on_pick = self.on_pick.clone();
            let ids = shown_ids.clone();
            move |e: &KeyDownEvent, window: &mut Window, cx: &mut App| {
                let m = &e.keystroke.modifiers;
                let modifier = m.control || m.alt || m.platform;
                let result = state.update(cx, |s, cx| {
                    let r = s.key(
                        &e.keystroke.key,
                        e.keystroke.key_char.as_deref(),
                        modifier,
                        ids.len(),
                    );
                    if r == PaletteKey::Handled {
                        cx.notify();
                    }
                    r
                });
                match result {
                    PaletteKey::Ignored => {}
                    PaletteKey::Handled => cx.stop_propagation(),
                    PaletteKey::Close => {
                        cx.stop_propagation();
                        if let Some(f) = &on_close {
                            f(window, cx);
                        }
                    }
                    PaletteKey::Pick(n) => {
                        cx.stop_propagation();
                        pick(&ids[n], &on_close, &on_pick, window, cx);
                    }
                }
            }
        };
        let mut scrim = div()
            .id(self.id)
            .track_focus(&focus)
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .bg(gpui::black().opacity(0.5))
            .occlude()
            .on_key_down(on_key);
        if let Some(f) = self.on_close {
            scrim = scrim.on_click(move |_, w, cx| f(w, cx));
        }
        scrim.child(body)
    }
}

trait EmptyList {
    fn when_empty(self, empty: bool, p: Palette) -> Self;
}

impl EmptyList for gpui::Stateful<gpui::Div> {
    fn when_empty(self, empty: bool, p: Palette) -> Self {
        if !empty {
            return self;
        }
        self.child(
            div()
                .py(px(24.0))
                .flex()
                .justify_center()
                .text_size(px(size::BODY))
                .text_color(p.muted_fg)
                .child("Nada encontrado."),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn itens() -> Vec<PaletteItem> {
        vec![
            PaletteItem::new("ir-fila", "Ir para", "Fila", IconName::ListOrdered)
                .desc("gates na vez"),
            PaletteItem::new("subir", "Ambiente", "Subir o ambiente", IconName::Rocket),
            PaletteItem::new(
                "log-api",
                "Serviços",
                "Log de api-demo",
                IconName::ScrollText,
            ),
            PaletteItem::new("execucoes", "Ir para", "Execuções", IconName::History),
        ]
    }

    #[test]
    fn busca_vazia_mostra_tudo_na_ordem() {
        assert_eq!(filter_palette(&itens(), ""), vec![0, 1, 2, 3]);
        assert_eq!(filter_palette(&itens(), "   "), vec![0, 1, 2, 3]);
    }

    #[test]
    fn busca_ignora_caixa_e_acento() {
        assert_eq!(filter_palette(&itens(), "EXECUCOES"), vec![3]);
        assert_eq!(filter_palette(&itens(), "serviços"), vec![2]);
    }

    #[test]
    fn todos_os_termos_precisam_casar_em_grupo_rotulo_ou_descricao() {
        assert_eq!(filter_palette(&itens(), "ir gates"), vec![0]);
        assert_eq!(filter_palette(&itens(), "ir para"), vec![0, 3]);
        assert!(filter_palette(&itens(), "fila subir").is_empty());
    }

    #[test]
    fn fold_tira_acento_e_marca_combinante() {
        assert_eq!(fold_text("Ação ÉPICA"), "acao epica");
        assert_eq!(fold_text("e\u{301}"), "e");
    }

    #[test]
    fn setas_param_nas_pontas() {
        let mut s = PaletteState::default();
        assert_eq!(s.key("up", None, false, 3), PaletteKey::Handled);
        assert_eq!(s.selected, 0);
        s.key("down", None, false, 3);
        s.key("down", None, false, 3);
        s.key("down", None, false, 3);
        assert_eq!(s.selected, 2);
        s.key("up", None, false, 3);
        assert_eq!(s.selected, 1);
    }

    #[test]
    fn enter_escolhe_o_marcado_e_lista_vazia_nao_escolhe() {
        let mut s = PaletteState {
            selected: 1,
            ..Default::default()
        };
        assert_eq!(s.key("enter", None, false, 2), PaletteKey::Pick(1));
        assert_eq!(s.key("enter", None, false, 0), PaletteKey::Handled);
    }

    #[test]
    fn digitar_e_apagar_voltam_a_marca_ao_topo() {
        let mut s = PaletteState {
            query: "fi".into(),
            selected: 2,
        };
        assert_eq!(s.key("l", Some("l"), false, 4), PaletteKey::Handled);
        assert_eq!((s.query.as_str(), s.selected), ("fil", 0));
        s.selected = 1;
        s.key("backspace", None, false, 4);
        assert_eq!((s.query.as_str(), s.selected), ("fi", 0));
        s.key("space", Some(" "), false, 4);
        assert_eq!(s.query, "fi ");
    }

    #[test]
    fn esc_fecha_e_atalho_com_ctrl_passa() {
        let mut s = PaletteState::default();
        assert_eq!(s.key("escape", None, false, 1), PaletteKey::Close);
        assert_eq!(s.key("k", Some("k"), true, 1), PaletteKey::Ignored);
        assert_eq!(s.key("f5", None, false, 1), PaletteKey::Ignored);
        assert!(s.query.is_empty());
    }

    #[test]
    fn comando_do_contrato_vira_item() {
        let c = Comando {
            id: "recarregar",
            grupo: "Fila",
            rotulo: "Recarregar",
            descricao: Some("lê a fila de novo"),
            icone: IconName::RefreshCw,
            atalho: Some("Ctrl R"),
        };
        let item = PaletteItem::from(&c);
        assert_eq!(item.id.as_ref(), "recarregar");
        assert_eq!(item.shortcut.as_deref(), Some("Ctrl R"));
        assert_eq!(filter_palette(&[item], "fila de novo"), vec![0]);
    }
}
