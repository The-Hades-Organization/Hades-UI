use gpui::{
    App, Entity, Focusable, FontWeight, Hsla, InteractiveElement, IntoElement, ParentElement,
    RenderOnce, SharedString, Styled, Window, div, px,
};
use gpui_component::input::{Input, InputState};

use crate::theme::palette;
use crate::tokens::{Palette, control, size};

/// Campo de texto sobre o `InputState` do gpui-component, no desenho do `campo` do painel: fundo `popover`
/// com linha de base de 2 px (`line`; `primary` com foco; `bad` com fundo `bad` 12% quando inválido),
/// rótulo e mensagem de erro inline.
/// Validar é do chamador: ele guarda o estado e passa o erro (ou `None`) a cada render.
#[derive(IntoElement)]
pub struct TextField {
    state: Entity<InputState>,
    label: Option<SharedString>,
    error: Option<SharedString>,
    disabled: bool,
}

impl TextField {
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(),
            label: None,
            error: None,
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn error(mut self, message: impl Into<SharedString>) -> Self {
        self.error = Some(message.into());
        self
    }

    pub fn error_opt(mut self, message: Option<impl Into<SharedString>>) -> Self {
        self.error = message.map(Into::into);
        self
    }
}

impl RenderOnce for TextField {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let focused = self.state.read(cx).focus_handle(cx).is_focused(window);
        let mut col = div().flex().flex_col().gap(px(6.0));
        if let Some(label) = self.label {
            col = col.child(
                div()
                    .text_color(p.fg)
                    .text_size(px(size::LABEL))
                    .font_weight(FontWeight::MEDIUM)
                    .child(label),
            );
        }
        let (line, bg) = match (&self.error, focused) {
            (Some(_), _) => (p.bad, mix_over(p.bad, 0.12, p.popover)),
            (None, true) => (p.primary, p.popover),
            (None, false) => (p.line, p.popover),
        };
        let hover_bg = mix_over(p.fg, 0.09, bg);
        let mut field = div()
            .h(px(control::MD))
            .flex()
            .items_center()
            .px(px(4.0))
            .bg(bg)
            .shadow(vec![Palette::baseline(line, 2.0)])
            .child(
                Input::new(&self.state)
                    .appearance(false)
                    .disabled(self.disabled),
            );
        if self.disabled {
            field = field.opacity(0.55);
        } else {
            field = field.hover(move |s| s.bg(hover_bg));
        }
        col = col.child(field);
        if let Some(error) = self.error {
            col = col.child(
                div()
                    .text_color(p.bad)
                    .text_size(px(size::META))
                    .child(error),
            );
        }
        col
    }
}

/// Mistura opaca `color-mix(in srgb, top pct, under)` entre duas cores já resolvidas.
fn mix_over(top: Hsla, pct: f32, under: Hsla) -> Hsla {
    let mut c = top;
    c.a = pct;
    under.blend(c)
}
