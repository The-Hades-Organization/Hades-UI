use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, KeyDownEvent, ParentElement,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, Window, div, px,
};

use super::IconButton;
use crate::icons::IconName;
use crate::theme::palette;
use crate::tokens::{FONT_HEADING, FONT_MONO, size};

type CloseHandler = Rc<dyn Fn(&mut Window, &mut App)>;

/// Largura padrão do painel, em px.
pub const SHEET_WIDTH: f32 = 440.0;
/// Folga mínima à esquerda do painel, para a página continuar visível atrás do véu.
const SHEET_MIN_GAP: f32 = 48.0;
const SHEET_MIN_WIDTH: f32 = 280.0;

/// Largura do painel numa janela de `window_width`: a pedida, sem passar da janela menos a folga e sem ficar
/// abaixo do mínimo legível.
pub fn sheet_width(requested: f32, window_width: f32) -> f32 {
    requested
        .min(window_width - SHEET_MIN_GAP)
        .max(SHEET_MIN_WIDTH.min(window_width))
}

/// Tecla que fecha o painel.
pub fn closes_sheet(key: &str) -> bool {
    key == "escape"
}

/// Painel lateral de detalhe (agente, gate, job) à direita, sobre a janela inteira: título, meta em mono, corpo
/// rolável e rodapé opcional. Esc, o botão de fechar e o clique no véu chamam `on_close`; renderize-o por
/// último, só enquanto estiver aberto, e feche-o no handler, como em [`super::Confirm`].
#[derive(IntoElement)]
pub struct Sheet {
    id: ElementId,
    title: SharedString,
    meta: Option<SharedString>,
    width: f32,
    body: Vec<AnyElement>,
    footer: Option<AnyElement>,
    on_close: Option<CloseHandler>,
}

impl Sheet {
    pub fn new(id: impl Into<ElementId>, title: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            meta: None,
            width: SHEET_WIDTH,
            body: Vec::new(),
            footer: None,
            on_close: None,
        }
    }

    /// Linha em mono sob o título (id, branch, duração).
    pub fn meta(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Ações fixas no pé, fora da rolagem.
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    pub fn on_close(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Rc::new(f));
        self
    }
}

impl ParentElement for Sheet {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.body.extend(elements);
    }
}

impl RenderOnce for Sheet {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let focus = window
            .use_keyed_state((self.id.clone(), "foco"), cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        if !focus.contains_focused(window, cx) {
            let f = focus.clone();
            window.defer(cx, move |window, cx| window.focus(&f, cx));
        }
        let width = sheet_width(self.width, f32::from(window.viewport_size().width));

        let mut heading = div()
            .flex()
            .flex_col()
            .gap(px(4.0))
            .flex_1()
            .min_w_0()
            .child(
                div()
                    .font_family(FONT_HEADING)
                    .text_size(px(size::SECTION))
                    .truncate()
                    .child(self.title),
            );
        if let Some(meta) = self.meta {
            heading = heading.child(
                div()
                    .font_family(FONT_MONO)
                    .text_size(px(size::META))
                    .text_color(p.muted_fg)
                    .truncate()
                    .child(meta),
            );
        }
        let mut close = IconButton::new((self.id.clone(), "fechar"), IconName::X, "Fechar (Esc)");
        if let Some(f) = self.on_close.clone() {
            close = close.on_click(move |_, w, cx| f(w, cx));
        }
        let header = div()
            .flex()
            .items_start()
            .gap(px(12.0))
            .px(px(20.0))
            .pt(px(18.0))
            .pb(px(14.0))
            .border_b_1()
            .border_color(p.border)
            .child(heading)
            .child(close);

        let mut panel = div()
            .id((self.id.clone(), "painel"))
            .absolute()
            .top_0()
            .right_0()
            .h_full()
            .w(px(width))
            .flex()
            .flex_col()
            .bg(p.card)
            .text_color(p.fg)
            .border_l_2()
            .border_color(p.primary)
            .shadow(vec![p.dialog_shadow()])
            .occlude()
            .child(header)
            .child(
                div()
                    .id((self.id.clone(), "corpo"))
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .px(px(20.0))
                    .py(px(16.0))
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .text_size(px(size::BODY))
                    .children(self.body),
            );
        if let Some(footer) = self.footer {
            panel = panel.child(
                div()
                    .flex()
                    .justify_end()
                    .gap(px(8.0))
                    .px(px(20.0))
                    .py(px(12.0))
                    .border_t_1()
                    .border_color(p.border)
                    .child(footer),
            );
        }

        let on_key = self.on_close.clone();
        let mut scrim = div()
            .id(self.id)
            .track_focus(&focus)
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .bg(gpui::black().opacity(0.35))
            .occlude()
            .on_key_down(move |e: &KeyDownEvent, window, cx| {
                if closes_sheet(&e.keystroke.key)
                    && let Some(f) = &on_key
                {
                    cx.stop_propagation();
                    f(window, cx);
                }
            });
        if let Some(f) = self.on_close {
            scrim = scrim.on_click(move |_, w, cx| f(w, cx));
        }
        scrim.child(panel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largura_pedida_cabe_em_janela_larga() {
        assert_eq!(sheet_width(SHEET_WIDTH, 1440.0), SHEET_WIDTH);
    }

    #[test]
    fn janela_estreita_encolhe_o_painel_e_deixa_a_folga() {
        assert_eq!(sheet_width(SHEET_WIDTH, 400.0), 352.0);
    }

    #[test]
    fn painel_nao_fica_abaixo_do_minimo_nem_passa_da_janela() {
        assert_eq!(sheet_width(SHEET_WIDTH, 300.0), 280.0);
        assert_eq!(sheet_width(SHEET_WIDTH, 200.0), 200.0);
    }

    #[test]
    fn so_esc_fecha() {
        assert!(closes_sheet("escape"));
        assert!(!closes_sheet("enter"));
        assert!(!closes_sheet("q"));
    }
}
