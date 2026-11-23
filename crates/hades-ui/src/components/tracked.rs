use gpui::{App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px};

/// Texto com espaçamento entre letras (`tracking-[<em>em]` do painel). O gpui 0.3.11 não tem `letter-spacing`,
/// então cada caractere vira uma caixa e o espaço entre elas é `em × size`. Serve para rótulo curto de uma
/// linha: não quebra, não trunca e perde o kerning entre os pares de letras.
#[derive(IntoElement)]
pub struct Tracked {
    text: SharedString,
    size: f32,
    em: f32,
}

impl Tracked {
    /// `size` em px é o tamanho da fonte, que também fica aplicado ao texto; `em` é o espaçamento em em.
    pub fn new(text: impl Into<SharedString>, size: f32, em: f32) -> Self {
        Self {
            text: text.into(),
            size,
            em,
        }
    }
}

/// Espaço em px entre letras vizinhas.
pub(crate) fn tracking_gap(size: f32, em: f32) -> f32 {
    size * em
}

impl RenderOnce for Tracked {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let row = div()
            .flex()
            .flex_none()
            .text_size(px(self.size))
            .gap(px(tracking_gap(self.size, self.em)));
        row.children(self.text.chars().map(|c| {
            if c == ' ' {
                div().w(px(self.size * 0.25)).into_any_element()
            } else {
                div()
                    .child(SharedString::from(c.to_string()))
                    .into_any_element()
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::tracking_gap;

    #[test]
    fn espaco_e_em_vezes_o_tamanho() {
        assert!((tracking_gap(11.0, 0.14) - 1.54).abs() < 1e-6);
        assert!((tracking_gap(20.0, 0.22) - 4.4).abs() < 1e-6);
    }
}
