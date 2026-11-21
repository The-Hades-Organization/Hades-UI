//! Fontes da marca embutidas no binário (OFL 1.1; licenças em `assets/fonts`).
use std::borrow::Cow;

use gpui::App;

/// Arquivo TTF embutido e a família que ele deve registrar.
pub struct EmbeddedFont {
    pub family: &'static str,
    pub bytes: &'static [u8],
}

pub const EMBEDDED: [EmbeddedFont; 5] = [
    EmbeddedFont {
        family: "Figtree",
        bytes: include_bytes!("../assets/fonts/Figtree-Regular.ttf"),
    },
    EmbeddedFont {
        family: "Figtree",
        bytes: include_bytes!("../assets/fonts/Figtree-SemiBold.ttf"),
    },
    EmbeddedFont {
        family: "Marcellus",
        bytes: include_bytes!("../assets/fonts/Marcellus-Regular.ttf"),
    },
    EmbeddedFont {
        family: "Geist Mono",
        bytes: include_bytes!("../assets/fonts/GeistMono-Regular.ttf"),
    },
    EmbeddedFont {
        family: "Geist Mono",
        bytes: include_bytes!("../assets/fonts/GeistMono-SemiBold.ttf"),
    },
];

/// Registra as fontes embutidas no text system do gpui.
pub fn register(cx: &App) {
    let fonts = EMBEDDED.iter().map(|f| Cow::Borrowed(f.bytes)).collect();
    if let Err(err) = cx.text_system().add_fonts(fonts) {
        eprintln!("hades-ui: falha ao registrar as fontes embutidas: {err}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn be16(b: &[u8], at: usize) -> usize {
        u16::from_be_bytes([b[at], b[at + 1]]) as usize
    }

    fn be32(b: &[u8], at: usize) -> usize {
        u32::from_be_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]]) as usize
    }

    /// Nome da família (ID 1) da tabela `name`, plataforma Windows (UTF-16BE).
    fn family_name(b: &[u8]) -> Option<String> {
        let tables = be16(b, 4);
        let name_off = (0..tables)
            .map(|i| 12 + i * 16)
            .find(|&r| &b[r..r + 4] == b"name")
            .map(|r| be32(b, r + 8))?;
        let count = be16(b, name_off + 2);
        let strings = name_off + be16(b, name_off + 4);
        (0..count)
            .map(|i| name_off + 6 + i * 12)
            .find(|&r| be16(b, r) == 3 && be16(b, r + 6) == 1)
            .map(|r| {
                let (len, off) = (be16(b, r + 8), be16(b, r + 10));
                let units: Vec<u16> = b[strings + off..strings + off + len]
                    .chunks(2)
                    .map(|c| u16::from_be_bytes([c[0], c[1]]))
                    .collect();
                String::from_utf16_lossy(&units)
            })
    }

    #[test]
    fn bytes_sao_ttf_e_familia_bate() {
        for f in &EMBEDDED {
            assert_eq!(&f.bytes[..4], &[0, 1, 0, 0], "{} não é TTF", f.family);
            assert_eq!(family_name(f.bytes).as_deref(), Some(f.family));
        }
    }

    #[test]
    fn familias_dos_tokens_estao_embutidas() {
        use crate::tokens::{FONT_HEADING, FONT_MONO, FONT_SANS};
        for fam in [FONT_SANS, FONT_HEADING, FONT_MONO] {
            assert!(
                EMBEDDED.iter().any(|f| f.family == fam),
                "{fam} sem arquivo"
            );
        }
    }
}
