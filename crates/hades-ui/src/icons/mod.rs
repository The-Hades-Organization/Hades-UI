//! Ícones do painel web do Hades, embutidos no binário: os `I.<Nome>` que o painel usa, do lucide (ISC) ou dos
//! desenhos próprios do Hades, com o traço do painel (2 px, ponta seca, quina viva). Licença em `assets/icons/LICENSE`.
//! O SVG é desenhado pelo `svg()` do gpui como máscara, na cor do texto herdada ou na que o chamador passar.
use gpui::{App, Hsla, IntoElement, Pixels, RenderOnce, Styled, Window, px, svg};

/// De onde vem o desenho de um [`IconName`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IconOrigin {
    /// lucide v0.460.0, ISC.
    Lucide,
    /// Desenho próprio do painel do Hades.
    Hades,
}

macro_rules! icon_set {
    ($($name:ident => $file:literal, $origin:ident;)*) => {
        /// Nome de um ícone do painel; o mesmo do `I.<Nome>` em `kit/panel/ui`.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum IconName {
            $($name,)*
        }

        impl IconName {
            /// Todos os ícones, em ordem alfabética.
            pub const ALL: &'static [IconName] = &[$(IconName::$name,)*];

            /// Conteúdo SVG embutido.
            pub fn svg(self) -> &'static [u8] {
                match self {
                    $(IconName::$name => include_bytes!(concat!("../../assets/icons/", $file)),)*
                }
            }

            /// Nome do arquivo em `assets/icons`.
            pub fn file_name(self) -> &'static str {
                match self {
                    $(IconName::$name => $file,)*
                }
            }

            /// Nome como o painel o escreve (`Trash2`, `CircleCheck`).
            pub fn name(self) -> &'static str {
                match self {
                    $(IconName::$name => stringify!($name),)*
                }
            }

            pub fn origin(self) -> IconOrigin {
                match self {
                    $(IconName::$name => icon_origin!($origin),)*
                }
            }
        }
    };
}

macro_rules! icon_origin {
    (lucide) => {
        IconOrigin::Lucide
    };
    (hades) => {
        IconOrigin::Hades
    };
}

include!("set.rs");

/// Glifo da marca do Hades (`kit/panel/ui/brand/hades-mark.svg`), desenhado como máscara na cor do metal.
pub const HADES_MARK: &[u8] = include_bytes!("../../assets/brand/hades-mark.svg");

/// Tamanho padrão do ícone, o `size-4` do painel.
pub const ICON_SIZE: f32 = 16.0;

/// Ícone renderizado: quadrado de `size`, na cor do texto herdada a menos que `color` diga outra.
#[derive(IntoElement)]
pub struct Icon {
    name: IconName,
    size: Pixels,
    color: Option<Hsla>,
}

impl Icon {
    pub fn new(name: IconName) -> Self {
        Self {
            name,
            size: px(ICON_SIZE),
            color: None,
        }
    }

    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into();
        self
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl From<IconName> for Icon {
    fn from(name: IconName) -> Self {
        Icon::new(name)
    }
}

impl RenderOnce for Icon {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        let color = self.color.unwrap_or_else(|| window.text_style().color);
        svg()
            .data(self.name.svg())
            .size(self.size)
            .flex_none()
            .text_color(color)
    }
}

/// Glifo da marca em `size`, na cor `color`.
pub fn brand_mark(size: impl Into<Pixels>, color: impl Into<Hsla>) -> impl IntoElement {
    svg()
        .data(HADES_MARK)
        .size(size.into())
        .flex_none()
        .text_color(color.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn todo_icone_tem_svg_valido_e_nome_unico() {
        let mut files = std::collections::HashSet::new();
        for icon in IconName::ALL {
            let text = std::str::from_utf8(icon.svg()).expect("utf-8");
            assert!(
                text.starts_with("<svg") && text.trim_end().ends_with("</svg>"),
                "{}",
                icon.name()
            );
            assert!(text.contains("viewBox="), "{} sem viewBox", icon.name());
            assert!(
                files.insert(icon.file_name()),
                "{} repetido",
                icon.file_name()
            );
        }
        assert!(IconName::ALL.len() >= 170);
    }

    #[test]
    fn desenhos_proprios_substituem_o_lucide_onde_o_painel_substitui() {
        for icon in [
            IconName::Search,
            IconName::X,
            IconName::Bell,
            IconName::Argos,
        ] {
            assert_eq!(icon.origin(), IconOrigin::Hades, "{}", icon.name());
        }
        assert_eq!(IconName::CircleX.origin(), IconOrigin::Lucide);
        assert_eq!(IconName::Trash2.origin(), IconOrigin::Lucide);
    }

    #[test]
    fn traco_do_painel_em_todo_icone_da_grade_24() {
        for icon in IconName::ALL {
            let text = std::str::from_utf8(icon.svg()).unwrap();
            if text.contains("viewBox=\"0 0 24 24\"") {
                assert!(
                    text.contains("stroke-linecap=\"square\""),
                    "{}",
                    icon.name()
                );
                assert!(
                    text.contains("stroke-linejoin=\"miter\""),
                    "{}",
                    icon.name()
                );
            }
        }
    }

    #[test]
    fn marca_embutida() {
        assert!(std::str::from_utf8(HADES_MARK).unwrap().contains("<svg"));
    }
}
