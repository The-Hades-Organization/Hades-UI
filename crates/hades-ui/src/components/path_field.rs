use std::path::PathBuf;
use std::rc::Rc;

use gpui::{
    App, ElementId, InteractiveElement, IntoElement, ParentElement, PathPromptOptions, RenderOnce,
    SharedString, Styled, Window, div, px,
};

use crate::components::Button;
use crate::theme::palette;
use crate::tokens::{FONT_MONO, size};

type PickHandler = Rc<dyn Fn(PathBuf, &mut App)>;

/// Campo de caminho somente leitura com botão "Escolher" que abre o diálogo de arquivo do sistema.
/// O caminho escolhido volta por `on_pick`; o chamador guarda o valor e o passa de novo em `value`.
#[derive(IntoElement)]
pub struct PathField {
    id: ElementId,
    value: Option<SharedString>,
    placeholder: SharedString,
    directories: bool,
    prompt: Option<SharedString>,
    on_pick: Option<PickHandler>,
}

impl PathField {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: None,
            placeholder: "Nenhum arquivo escolhido".into(),
            directories: false,
            prompt: None,
            on_pick: None,
        }
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Escolhe pasta em vez de arquivo.
    pub fn directories(mut self) -> Self {
        self.directories = true;
        self
    }

    /// Texto do botão de confirmar do diálogo do sistema.
    pub fn prompt(mut self, prompt: impl Into<SharedString>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    pub fn on_pick(mut self, f: impl Fn(PathBuf, &mut App) + 'static) -> Self {
        self.on_pick = Some(Rc::new(f));
        self
    }
}

impl RenderOnce for PathField {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let (text, color) = match self.value {
            Some(v) => (v, p.fg),
            None => (self.placeholder, p.muted_fg),
        };
        let options = PathPromptOptions {
            files: !self.directories,
            directories: self.directories,
            multiple: false,
            prompt: self.prompt,
        };
        let on_pick = self.on_pick;
        div()
            .id(self.id.clone())
            .flex()
            .items_center()
            .gap(px(8.0))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .overflow_hidden()
                    .px(px(10.0))
                    .py(px(7.0))
                    .border_1()
                    .border_color(p.border_strong)
                    .bg(p.card)
                    .text_color(color)
                    .font_family(FONT_MONO)
                    .text_size(px(size::META))
                    .child(text),
            )
            .child(
                Button::new(ElementId::from((self.id, "escolher")), "Escolher").on_click(
                    move |_, _, cx| {
                        let Some(on_pick) = on_pick.clone() else {
                            return;
                        };
                        let picked = cx.prompt_for_paths(options.clone());
                        cx.spawn(async move |cx| {
                            if let Ok(Ok(Some(mut paths))) = picked.await
                                && let Some(path) = paths.pop()
                            {
                                cx.update(|cx| on_pick(path, cx));
                            }
                        })
                        .detach();
                    },
                ),
            )
    }
}
