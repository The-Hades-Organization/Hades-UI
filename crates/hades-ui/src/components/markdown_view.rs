use std::ops::Range;

use gpui::{
    App, HighlightStyle, IntoElement, ParentElement, RenderOnce, SharedString, Styled, StyledText,
    Window, div, px,
};

use crate::theme::palette;
use crate::tokens::{FONT_HEADING, size};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Block {
    Heading(u8, Inline),
    Item(Inline),
    Paragraph(Inline),
}

/// Texto sem as crases e as faixas (em bytes) que eram código inline.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Inline {
    pub text: String,
    pub code: Vec<Range<usize>>,
}

pub(crate) fn parse_inline(src: &str) -> Inline {
    let mut text = String::new();
    let mut code = Vec::new();
    let mut rest = src;
    while let Some(open) = rest.find('`') {
        let after = &rest[open + 1..];
        let Some(close) = after.find('`') else { break };
        if close == 0 {
            text.push_str(&rest[..open + 2]);
            rest = &after[1..];
            continue;
        }
        text.push_str(&rest[..open]);
        let start = text.len();
        text.push_str(&after[..close]);
        code.push(start..text.len());
        rest = &after[close + 1..];
    }
    text.push_str(rest);
    Inline { text, code }
}

/// Cabeçalho (`#` a `###`), item de lista (`-` ou `*`) e parágrafo; linhas seguidas de parágrafo viram um só.
pub(crate) fn parse_blocks(src: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut para: Vec<&str> = Vec::new();
    let flush = |para: &mut Vec<&str>, blocks: &mut Vec<Block>| {
        if !para.is_empty() {
            blocks.push(Block::Paragraph(parse_inline(&para.join(" "))));
            para.clear();
        }
    };
    for raw in src.lines() {
        let line = raw.trim();
        if line.is_empty() {
            flush(&mut para, &mut blocks);
            continue;
        }
        let hashes = line.chars().take_while(|c| *c == '#').count();
        if (1..=3).contains(&hashes) && line[hashes..].starts_with(' ') {
            flush(&mut para, &mut blocks);
            blocks.push(Block::Heading(
                hashes as u8,
                parse_inline(line[hashes..].trim()),
            ));
        } else if let Some(item) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")) {
            flush(&mut para, &mut blocks);
            blocks.push(Block::Item(parse_inline(item.trim())));
        } else {
            para.push(line);
        }
    }
    flush(&mut para, &mut blocks);
    blocks
}

/// Documento markdown somente leitura: cabeçalhos, listas e código inline. Nada além disso é interpretado.
#[derive(IntoElement)]
pub struct MarkdownView {
    source: SharedString,
}

impl MarkdownView {
    pub fn new(source: impl Into<SharedString>) -> Self {
        Self {
            source: source.into(),
        }
    }
}

impl RenderOnce for MarkdownView {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let styled = |inline: Inline| {
            let style = HighlightStyle {
                background_color: Some(p.muted),
                color: Some(p.fg),
                ..Default::default()
            };
            StyledText::new(inline.text)
                .with_highlights(inline.code.into_iter().map(|r| (r, style)))
        };
        let mut doc = div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .text_color(p.fg)
            .text_size(px(size::PROSE));
        for block in parse_blocks(&self.source) {
            doc = doc.child(match block {
                Block::Heading(level, inline) => {
                    let px_size = match level {
                        1 => size::SECTION_LG,
                        2 => size::SECTION,
                        _ => size::BODY,
                    };
                    div()
                        .pt(px(4.0))
                        .font_family(FONT_HEADING)
                        .text_size(px(px_size))
                        .child(styled(inline))
                }
                Block::Item(inline) => div()
                    .flex()
                    .gap(px(8.0))
                    .pl(px(8.0))
                    .child(div().flex_none().text_color(p.muted_fg).child("•"))
                    .child(div().flex_1().min_w_0().child(styled(inline))),
                Block::Paragraph(inline) => div().child(styled(inline)),
            });
        }
        doc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_extrai_codigo_sem_as_crases() {
        let i = parse_inline("o `/health/ready` caía e `x` voltou");
        assert_eq!(i.text, "o /health/ready caía e x voltou");
        assert_eq!(&i.text[i.code[0].clone()], "/health/ready");
        assert_eq!(&i.text[i.code[1].clone()], "x");
    }

    #[test]
    fn crase_sem_fechar_fica_literal() {
        let i = parse_inline("sobrou ` aberta");
        assert_eq!(i.text, "sobrou ` aberta");
        assert!(i.code.is_empty());
    }

    #[test]
    fn crases_vazias_ficam_literais() {
        assert_eq!(parse_inline("a `` b").text, "a `` b");
    }

    #[test]
    fn blocos_cabecalho_lista_e_paragrafo() {
        let b =
            parse_blocks("# Título\n\nlinha um\nlinha dois\n\n- a\n* b\n###### não\n#sem espaço");
        assert_eq!(b.len(), 5);
        assert!(matches!(&b[0], Block::Heading(1, i) if i.text == "Título"));
        assert!(matches!(&b[1], Block::Paragraph(i) if i.text == "linha um linha dois"));
        assert!(matches!(&b[2], Block::Item(i) if i.text == "a"));
        assert!(matches!(&b[3], Block::Item(i) if i.text == "b"));
        assert!(matches!(&b[4], Block::Paragraph(i) if i.text == "###### não #sem espaço"));
    }

    #[test]
    fn lista_interrompe_paragrafo() {
        let b = parse_blocks("texto\n- item");
        assert!(matches!(&b[0], Block::Paragraph(_)));
        assert!(matches!(&b[1], Block::Item(_)));
    }
}
