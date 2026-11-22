use gpui::{
    App, Entity, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};
use gpui_component::input::{Input, InputState};

use crate::theme::palette;
use crate::tokens::size;

/// Campo de texto sobre o `InputState` do gpui-component, com rótulo e mensagem de erro inline.
/// Validar é do chamador: ele guarda o estado e passa o erro (ou `None`) a cada render.
#[derive(IntoElement)]
pub struct TextField {
    state: Entity<InputState>,
    label: Option<SharedString>,
    error: Option<SharedString>,
}

impl TextField {
    pub fn new(state: &Entity<InputState>) -> Self {
        Self {
            state: state.clone(),
            label: None,
            error: None,
        }
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
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let mut col = div().flex().flex_col().gap(px(4.0));
        if let Some(label) = self.label {
            col = col.child(
                div()
                    .text_color(p.muted_fg)
                    .text_size(px(size::MICRO))
                    .child(label.to_uppercase()),
            );
        }
        let mut input = div().border_1().border_color(p.border_strong);
        if self.error.is_some() {
            input = input.border_color(p.bad);
        }
        col = col.child(input.child(Input::new(&self.state)));
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
