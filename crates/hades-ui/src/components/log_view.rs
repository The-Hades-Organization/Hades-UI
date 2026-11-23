use std::ops::Range;
use std::rc::Rc;

use gpui::{
    App, ElementId, Entity, Focusable, HighlightStyle, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, ScrollHandle, SharedString, StatefulInteractiveElement, Styled,
    StyledText, Window, div, px,
};
use gpui_component::input::{Input, InputState};

use super::Button;
use crate::icons::{Icon, IconName};
use crate::theme::palette;
use crate::tokens::{FONT_MONO, Palette, Tone, size};

type ToggleHandler = Rc<dyn Fn(bool, &mut Window, &mut App)>;

/// Texto de um log lido aos pedaços, com o offset do que já veio.
///
/// Porta o `poll` do `Terminal` (`kit/panel/ui/app.tsx:1534-1541`): com offset zero o pedaço substitui o texto,
/// senão se junta ao fim; o offset passa a ser o que o servidor devolveu.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LogBuffer {
    pub text: String,
    pub offset: u64,
}

impl LogBuffer {
    /// Junta `chunk`, lido a partir de `self.offset`, e guarda `next_offset`. Offset que volta atrás (arquivo
    /// truncado ou trocado) recomeça do pedaço.
    pub fn apply(&mut self, chunk: &str, next_offset: u64) {
        if self.offset == 0 || next_offset < self.offset {
            self.text = chunk.to_string();
        } else {
            self.text.push_str(chunk);
        }
        self.offset = next_offset;
    }

    /// Volta ao começo (outra fonte escolhida).
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Tom da linha: erro (`level` 50/60, ERROR, FATAL, "erro", FALHOU, ✗), aviso (`level` 40, WARN) ou nenhum.
///
/// Porta `logColor` (`kit/panel/ui/app.tsx:1573`).
pub fn log_tone(line: &str) -> Option<Tone> {
    let lower = line.to_lowercase();
    let level = json_level(line);
    let bad = matches!(level.as_deref(), Some("50" | "60"))
        || matches!(
            level.as_deref().map(str::to_uppercase).as_deref(),
            Some("ERROR" | "FATAL")
        )
        || has_word(&lower, "error")
        || has_word(&lower, "erro")
        || lower.contains("falhou")
        || line.contains('✗');
    if bad {
        return Some(Tone::Bad);
    }
    let warn = matches!(level.as_deref(), Some("40"))
        || matches!(
            level.as_deref().map(str::to_uppercase).as_deref(),
            Some("WARN")
        )
        || has_word_start(&lower, "warn");
    warn.then_some(Tone::Warn)
}

/// Valor do campo `"level"` de uma linha JSON (pino), sem aspas: `50`, `"ERROR"` → `ERROR`.
fn json_level(line: &str) -> Option<String> {
    let at = line.find("\"level\":")? + "\"level\":".len();
    let rest = line[at..].trim_start();
    let value: String = if let Some(quoted) = rest.strip_prefix('"') {
        quoted.chars().take_while(|c| *c != '"').collect()
    } else {
        rest.chars().take_while(char::is_ascii_digit).collect()
    };
    (!value.is_empty()).then_some(value)
}

fn is_word_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// `word` como palavra inteira (`\bword\b`).
fn has_word(text: &str, word: &str) -> bool {
    text.match_indices(word).any(|(i, _)| {
        let before = text[..i]
            .chars()
            .next_back()
            .is_none_or(|c| !is_word_char(c));
        let after = text[i + word.len()..]
            .chars()
            .next()
            .is_none_or(|c| !is_word_char(c));
        before && after
    })
}

/// `word` no começo de uma palavra (`\bword`).
fn has_word_start(text: &str, word: &str) -> bool {
    text.match_indices(word).any(|(i, _)| {
        text[..i]
            .chars()
            .next_back()
            .is_none_or(|c| !is_word_char(c))
    })
}

/// Faixas em bytes de cada ocorrência de `query` em `line`, sem caixa e sem sobreposição. Busca vazia não acha nada.
pub fn match_ranges(line: &str, query: &str) -> Vec<Range<usize>> {
    let needle: Vec<char> = query.chars().flat_map(char::to_lowercase).collect();
    if needle.is_empty() {
        return Vec::new();
    }
    let chars: Vec<(usize, char)> = line
        .char_indices()
        .map(|(i, c)| (i, c.to_lowercase().next().unwrap_or(c)))
        .collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i + needle.len() <= chars.len() {
        if chars[i..i + needle.len()]
            .iter()
            .zip(&needle)
            .all(|((_, a), b)| a == b)
        {
            let start = chars[i].0;
            let end = chars
                .get(i + needle.len())
                .map(|(b, _)| *b)
                .unwrap_or(line.len());
            out.push(start..end);
            i += needle.len();
        } else {
            i += 1;
        }
    }
    out
}

/// Linhas que aparecem, com o número (a partir de 1) na fonte: todas, ou só as que contêm `query` sem caixa.
///
/// Porta o filtro de `LogViewer` (`kit/panel/ui/app.tsx:1603-1606`).
pub fn visible_lines<'a>(text: &'a str, query: &str) -> Vec<(usize, &'a str)> {
    let lines = text.split('\n').enumerate().map(|(i, l)| (i + 1, l));
    if query.is_empty() {
        return lines.collect();
    }
    let q = query.to_lowercase();
    lines
        .filter(|(_, l)| l.to_lowercase().contains(&q))
        .collect()
}

/// "N linhas · M com erro", contadas sobre as linhas visíveis.
pub fn log_summary(lines: &[(usize, &str)]) -> String {
    let n = lines.len();
    let errors = lines
        .iter()
        .filter(|(_, l)| log_tone(l) == Some(Tone::Bad))
        .count();
    let mut out = format!("{n} {}", if n == 1 { "linha" } else { "linhas" });
    if errors > 0 {
        out.push_str(&format!(" · {errors} com erro"));
    }
    out
}

/// Visor de log no fundo do terminal: cabeçalho com título, contagem, busca e o botão Seguir; linhas em mono com
/// erro e aviso coloridos e as ocorrências da busca destacadas. Seguindo, a rolagem fica presa no fim a cada
/// render. O texto, o offset ([`LogBuffer`]) e o estado de seguir são do chamador.
///
/// Porta `LogViewer` (`kit/panel/ui/app.tsx:1576-1660`), com destaque da busca em vez de só filtrar.
#[derive(IntoElement)]
pub struct LogView {
    id: ElementId,
    text: SharedString,
    title: Option<SharedString>,
    search: Option<Entity<InputState>>,
    follow: bool,
    on_follow: Option<ToggleHandler>,
    placeholder: SharedString,
}

impl LogView {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            title: None,
            search: None,
            follow: false,
            on_follow: None,
            placeholder: "carregando…".into(),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Campo de busca: filtra as linhas e destaca as ocorrências.
    pub fn search(mut self, state: &Entity<InputState>) -> Self {
        self.search = Some(state.clone());
        self
    }

    /// Presa no fim; o botão Seguir aparece quando há `on_follow`.
    pub fn follow(mut self, follow: bool) -> Self {
        self.follow = follow;
        self
    }

    pub fn on_follow(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_follow = Some(Rc::new(f));
        self
    }

    /// Texto no lugar do log vazio.
    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = text.into();
        self
    }
}

impl RenderOnce for LogView {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let scroll = window
            .use_keyed_state((self.id.clone(), "rolagem"), cx, |_, _| ScrollHandle::new())
            .read(cx)
            .clone();
        let query = self
            .search
            .as_ref()
            .map(|s| s.read(cx).value().to_string())
            .unwrap_or_default();
        let lines = visible_lines(&self.text, &query);

        let mut head = div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .pb(px(10.0))
            .border_b_1()
            .border_color(p.border);
        let mut heading = div()
            .flex()
            .items_baseline()
            .gap(px(12.0))
            .flex_1()
            .min_w_0();
        if let Some(title) = self.title {
            heading = heading.child(div().truncate().text_size(px(size::SECTION)).child(title));
        }
        if !self.text.is_empty() {
            heading = heading.child(
                div()
                    .flex_none()
                    .text_size(px(size::META))
                    .text_color(p.muted_fg)
                    .child(log_summary(&lines)),
            );
        }
        head = head.child(heading);
        if let Some(state) = &self.search {
            let focused = state.read(cx).focus_handle(cx).is_focused(window);
            head = head.child(
                div()
                    .w(px(224.0))
                    .h(px(28.0))
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .px(px(6.0))
                    .bg(p.popover)
                    .shadow(vec![Palette::baseline(
                        if focused { p.primary } else { p.line },
                        2.0,
                    )])
                    .child(
                        Icon::new(IconName::TextSearch)
                            .size(px(14.0))
                            .color(p.muted_fg),
                    )
                    .child(div().flex_1().child(Input::new(state).appearance(false))),
            );
        }
        if let Some(f) = self.on_follow {
            let follow = self.follow;
            let mut btn = Button::new(
                (self.id.clone(), "seguir"),
                if follow { "Seguindo" } else { "Seguir" },
            )
            .icon(if follow {
                IconName::Pause
            } else {
                IconName::Play
            })
            .on_click(move |_, w, cx| f(!follow, w, cx));
            if !follow {
                btn = btn.kind(super::ButtonKind::Ghost);
            }
            head = head.child(btn);
        }

        let body: Vec<gpui::AnyElement> = if self.text.is_empty() {
            vec![
                div()
                    .text_color(p.term_mute)
                    .child(self.placeholder)
                    .into_any_element(),
            ]
        } else {
            lines
                .iter()
                .map(|(_, line)| {
                    let color = match log_tone(line) {
                        Some(Tone::Bad) => p.term_bad,
                        Some(Tone::Warn) => p.term_warn,
                        _ => p.term_fg,
                    };
                    let mark = HighlightStyle {
                        background_color: Some(p.warn.opacity(0.45)),
                        color: Some(p.term_fg),
                        ..Default::default()
                    };
                    let text: SharedString = if line.is_empty() {
                        " ".into()
                    } else {
                        SharedString::from(line.to_string())
                    };
                    let ranges = match_ranges(&text, &query);
                    div()
                        .text_color(color)
                        .child(
                            StyledText::new(text)
                                .with_highlights(ranges.into_iter().map(|r| (r, mark))),
                        )
                        .into_any_element()
                })
                .collect()
        };
        if self.follow {
            scroll.scroll_to_bottom();
        }

        div()
            .flex()
            .flex_col()
            .min_w_0()
            .min_h_0()
            .size_full()
            .child(head)
            .child(
                div()
                    .id((self.id.clone(), "linhas"))
                    .track_scroll(&scroll)
                    .mt(px(12.0))
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .p(px(12.0))
                    .bg(p.term)
                    .border_1()
                    .border_color(p.border)
                    .font_family(FONT_MONO)
                    .text_size(px(size::META))
                    .children(body),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primeiro_pedaco_substitui_e_os_seguintes_juntam() {
        let mut b = LogBuffer::default();
        b.apply("a\n", 2);
        b.apply("b\n", 4);
        assert_eq!((b.text.as_str(), b.offset), ("a\nb\n", 4));
    }

    #[test]
    fn offset_que_volta_recomeca() {
        let mut b = LogBuffer {
            text: "velho\n".into(),
            offset: 6,
        };
        b.apply("novo\n", 5);
        assert_eq!((b.text.as_str(), b.offset), ("novo\n", 5));
        b.reset();
        assert_eq!(b, LogBuffer::default());
    }

    #[test]
    fn tom_de_erro_e_de_aviso() {
        assert_eq!(log_tone(r#"{"level":50,"msg":"x"}"#), Some(Tone::Bad));
        assert_eq!(log_tone(r#"{"level": 60}"#), Some(Tone::Bad));
        assert_eq!(log_tone(r#"{"level":"ERROR"}"#), Some(Tone::Bad));
        assert_eq!(log_tone("build FALHOU em 3 s"), Some(Tone::Bad));
        assert_eq!(log_tone("[ERROR] sem conexão"), Some(Tone::Bad));
        assert_eq!(log_tone("deu erro no passo 2"), Some(Tone::Bad));
        assert_eq!(log_tone("✗ teste"), Some(Tone::Bad));
        assert_eq!(log_tone(r#"{"level":40}"#), Some(Tone::Warn));
        assert_eq!(log_tone("WARNING: disco em 90%"), Some(Tone::Warn));
        assert_eq!(log_tone(r#"{"level":30,"msg":"ok"}"#), None);
        assert_eq!(log_tone("errors_total=0 terror"), None);
    }

    #[test]
    fn busca_acha_sem_caixa_em_bytes() {
        assert_eq!(match_ranges("Gate ok, gate fim", "GATE"), vec![0..4, 9..13]);
        assert_eq!(match_ranges("ação Ação", "ação"), vec![0..6, 7..13]);
        assert_eq!(match_ranges("aaaa", "aa"), vec![0..2, 2..4]);
        assert!(match_ranges("abc", "").is_empty());
        assert!(match_ranges("abc", "x").is_empty());
    }

    #[test]
    fn filtro_mantem_o_numero_da_linha() {
        let text = "inicio\nGate A\nfim\ngate b";
        assert_eq!(
            visible_lines(text, "gate"),
            vec![(2, "Gate A"), (4, "gate b")]
        );
        assert_eq!(visible_lines(text, "").len(), 4);
    }

    #[test]
    fn resumo_conta_linhas_e_erros() {
        assert_eq!(log_summary(&[(1, "ok")]), "1 linha");
        assert_eq!(
            log_summary(&[(1, "ok"), (2, "ERROR x"), (3, "ok")]),
            "3 linhas · 1 com erro"
        );
    }
}
