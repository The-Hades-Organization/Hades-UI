use std::rc::Rc;

use gpui::{
    App, ElementId, FontWeight, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    SharedString, StatefulInteractiveElement, Styled, Window, anchored, deferred, div, px,
};

use crate::icons::{Icon, IconName};
use crate::theme::palette;
use crate::tokens::{Palette, control, size};

type ChangeHandler = Rc<dyn Fn(&SharedString, &mut Window, &mut App)>;

/// Opção de [`Select`]: o valor gravado e o rótulo mostrado (que pode ser o próprio valor).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelectOption {
    pub value: SharedString,
    pub label: SharedString,
}

impl SelectOption {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

impl From<SharedString> for SelectOption {
    fn from(value: SharedString) -> Self {
        Self {
            label: value.clone(),
            value,
        }
    }
}

impl From<&'static str> for SelectOption {
    fn from(value: &'static str) -> Self {
        SharedString::from(value).into()
    }
}

impl From<String> for SelectOption {
    fn from(value: String) -> Self {
        SharedString::from(value).into()
    }
}

/// Texto do gatilho: o rótulo da opção escolhida; um valor fora da lista aparece cru, para não esconder o que
/// está no arquivo; sem valor, o `placeholder`. O `bool` diz se o texto é de uma opção da lista.
pub fn select_display(
    options: &[SelectOption],
    value: Option<&str>,
    placeholder: &str,
) -> (SharedString, bool) {
    match value {
        None => (placeholder.to_string().into(), false),
        Some(v) => match options.iter().find(|o| o.value.as_ref() == v) {
            Some(o) => (o.label.clone(), true),
            None => (v.to_string().into(), false),
        },
    }
}

/// Campo de escolha numa lista fechada, para chaves do catálogo de Configurações com valores enumerados.
/// Sem estado de valor: o chamador guarda o valor e o passa a cada render; o `Select` só guarda se a lista
/// está aberta. Clique fora fecha a lista.
#[derive(IntoElement)]
pub struct Select {
    id: ElementId,
    options: Vec<SelectOption>,
    value: Option<SharedString>,
    placeholder: SharedString,
    label: Option<SharedString>,
    error: Option<SharedString>,
    disabled: bool,
    on_change: Option<ChangeHandler>,
}

impl Select {
    pub fn new(
        id: impl Into<ElementId>,
        options: impl IntoIterator<Item = impl Into<SelectOption>>,
    ) -> Self {
        Self {
            id: id.into(),
            options: options.into_iter().map(Into::into).collect(),
            value: None,
            placeholder: "Escolha".into(),
            label: None,
            error: None,
            disabled: false,
            on_change: None,
        }
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Chamado com o valor da opção escolhida; escolher a opção já marcada também chama.
    pub fn on_change(mut self, f: impl Fn(&SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(f));
        self
    }
}

impl RenderOnce for Select {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let open_state = window.use_keyed_state(self.id.clone(), cx, |_, _| false);
        let open = *open_state.read(cx) && !self.disabled;
        let (text, known) = select_display(&self.options, self.value.as_deref(), &self.placeholder);

        let mut col = div().flex().flex_col().gap(px(6.0)).relative();
        if let Some(label) = self.label {
            col = col.child(
                div()
                    .text_color(p.fg)
                    .text_size(px(size::LABEL))
                    .font_weight(FontWeight::MEDIUM)
                    .child(label),
            );
        }
        let line = match (&self.error, open) {
            (Some(_), _) => p.bad,
            (None, true) => p.primary,
            (None, false) => p.line,
        };
        let mut trigger = div()
            .id((self.id.clone(), "gatilho"))
            .h(px(control::MD))
            .min_w(px(160.0))
            .flex()
            .items_center()
            .justify_between()
            .gap(px(8.0))
            .px(px(8.0))
            .bg(p.popover)
            .shadow(vec![Palette::baseline(line, 2.0)])
            .text_size(px(size::BODY))
            .text_color(if known { p.fg } else { p.muted_fg })
            .child(div().truncate().child(text))
            .child(
                Icon::new(IconName::ChevronDown)
                    .size(px(14.0))
                    .color(p.muted_fg),
            );
        if self.disabled {
            trigger = trigger.opacity(0.55).cursor_not_allowed();
        } else {
            let toggle = open_state.clone();
            trigger = trigger
                .cursor_pointer()
                .hover(move |s| s.bg(p.accent))
                .on_click(move |_, _, cx| {
                    toggle.update(cx, |o, cx| {
                        *o = !*o;
                        cx.notify();
                    })
                });
        }

        let mut anchor = div().relative().child(trigger);
        if open {
            anchor = anchor.child(list(
                &p,
                &self.id,
                self.options,
                self.value,
                open_state,
                self.on_change,
            ));
        }
        col = col.child(anchor);
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

fn list(
    p: &Palette,
    id: &ElementId,
    options: Vec<SelectOption>,
    value: Option<SharedString>,
    open_state: gpui::Entity<bool>,
    on_change: Option<ChangeHandler>,
) -> impl IntoElement {
    let close_out = open_state.clone();
    let mut panel = div()
        .id((id.clone(), "lista"))
        .flex()
        .flex_col()
        .min_w(px(160.0))
        .max_h(px(260.0))
        .overflow_y_scroll()
        .p(px(4.0))
        .bg(p.popover)
        .text_color(p.fg)
        .text_size(px(size::BODY))
        .shadow(vec![Palette::baseline(p.primary, 2.0), p.elevation()])
        .on_mouse_down_out(move |_, _, cx| {
            close_out.update(cx, |o, cx| {
                *o = false;
                cx.notify();
            })
        });
    for (i, option) in options.into_iter().enumerate() {
        let selected = value.as_ref() == Some(&option.value);
        let close = open_state.clone();
        let change = on_change.clone();
        let hover = p.hover;
        let picked = option.value;
        panel = panel.child(
            div()
                .id((id.clone(), format!("op-{i}")))
                .h(px(control::XS))
                .flex()
                .items_center()
                .gap(px(8.0))
                .px(px(8.0))
                .cursor_pointer()
                .hover(move |s| s.bg(hover))
                .child(
                    div()
                        .w(px(14.0))
                        .children(selected.then(|| Icon::new(IconName::Check).size(px(14.0)))),
                )
                .child(option.label)
                .on_click(move |_, window, cx| {
                    close.update(cx, |o, cx| {
                        *o = false;
                        cx.notify();
                    });
                    if let Some(f) = &change {
                        f(&picked, window, cx);
                    }
                }),
        );
    }
    div()
        .absolute()
        .top(px(control::MD + 4.0))
        .left_0()
        .child(deferred(anchored().snap_to_window().child(panel)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opcoes() -> Vec<SelectOption> {
        vec![
            SelectOption::new("auto", "Automático"),
            SelectOption::new("on", "Ligado"),
            "off".into(),
        ]
    }

    #[test]
    fn valor_da_lista_mostra_o_rotulo() {
        assert_eq!(
            select_display(&opcoes(), Some("on"), "Escolha"),
            ("Ligado".into(), true)
        );
        assert_eq!(
            select_display(&opcoes(), Some("off"), "Escolha"),
            ("off".into(), true)
        );
    }

    #[test]
    fn valor_fora_da_lista_aparece_cru() {
        assert_eq!(
            select_display(&opcoes(), Some("talvez"), "Escolha"),
            ("talvez".into(), false)
        );
    }

    #[test]
    fn sem_valor_mostra_o_placeholder() {
        assert_eq!(
            select_display(&opcoes(), None, "Escolha"),
            ("Escolha".into(), false)
        );
    }
}
