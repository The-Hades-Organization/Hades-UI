use gpui::{App, ElementId, IntoElement, RenderOnce, Window};

use super::IconButton;
use crate::icons::IconName;
use crate::theme::{is_dark, toggle_mode};

/// Botão de tema (`ThemeToggle` do painel): sol no escuro, lua no claro; o clique alterna o modo da janela inteira
/// por [`toggle_mode`]. Serve sem barra superior: vai no `PageHeader::action` ou no pé do `Rail` (`Rail::theme_toggle`).
#[derive(IntoElement)]
pub struct ThemeToggle {
    id: ElementId,
    on_rail: bool,
}

impl ThemeToggle {
    pub fn new() -> Self {
        Self {
            id: "hades-ui-tema".into(),
            on_rail: false,
        }
    }

    /// Identificador; só precisa mudar se houver dois na janela.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    /// Cores do trilho escuro, para o pé do `Rail`.
    pub fn on_rail(mut self) -> Self {
        self.on_rail = true;
        self
    }
}

impl Default for ThemeToggle {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ThemeToggle {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let (icon, label) = if is_dark(cx) {
            (IconName::Sun, "Tema claro")
        } else {
            (IconName::Moon, "Tema escuro")
        };
        let button = IconButton::new(self.id, icon, label).on_click(|_, window, cx| {
            toggle_mode(cx);
            window.refresh();
        });
        if self.on_rail {
            button.on_rail()
        } else {
            button
        }
    }
}
