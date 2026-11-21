//! Tokens do `DESIGN.md` do Hades: paleta nos dois modos, tipografia, espaçamento e raios.
use gpui::{Hsla, rgb};

pub const FONT_SANS: &str = "Figtree";
pub const FONT_HEADING: &str = "Marcellus";
pub const FONT_MONO: &str = "Geist Mono";

pub mod size {
    pub const MICRO: f32 = 11.0;
    pub const META: f32 = 12.0;
    pub const LABEL: f32 = 13.0;
    pub const BODY: f32 = 14.0;
    pub const PROSE: f32 = 15.0;
    pub const SECTION: f32 = 16.0;
    pub const SECTION_LG: f32 = 18.0;
    pub const DISPLAY: f32 = 24.0;
    pub const PAGE_TITLE: f32 = 32.0;
}

pub mod space {
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 12.0;
    pub const LG: f32 = 16.0;
    pub const XL: f32 = 24.0;
}

/// O design é quadrado: separação por fio e por tom, sem sombra.
pub const RADIUS: f32 = 0.0;
pub const TOP_BAR_HEIGHT: f32 = 34.0;
pub const STATUS_BAR_HEIGHT: f32 = 26.0;
pub const RAIL_WIDTH: f32 = 188.0;

/// Cor com função: o estado que um selo ou medidor comunica.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tone {
    Neutral,
    Vivo,
    Ok,
    Warn,
    Bad,
    Info,
}

/// Paleta completa de um modo. Cores de fio já trazem a transparência do CSS (18% e 40% do `line`).
#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub bg: Hsla,
    pub fg: Hsla,
    pub card: Hsla,
    pub muted: Hsla,
    pub muted_fg: Hsla,
    pub border: Hsla,
    pub border_strong: Hsla,
    pub vivo: Hsla,
    pub primary: Hsla,
    pub primary_fg: Hsla,
    pub rail: Hsla,
    pub rail_fg: Hsla,
    pub rail_mute: Hsla,
    pub rail_sel: Hsla,
    pub rail_line: Hsla,
    pub term: Hsla,
    pub term_fg: Hsla,
    pub term_mute: Hsla,
    pub ok: Hsla,
    pub warn: Hsla,
    pub bad: Hsla,
    pub info: Hsla,
}

fn hex(v: u32) -> Hsla {
    rgb(v).into()
}

fn alpha(v: u32, a: f32) -> Hsla {
    let mut c = hex(v);
    c.a = a;
    c
}

impl Palette {
    pub fn light() -> Self {
        Self {
            bg: hex(0xfbf3f6),
            fg: hex(0x301423),
            card: hex(0xfff8fb),
            muted: hex(0xf0dfe6),
            muted_fg: hex(0x684b5b),
            border: alpha(0x301423, 0.18),
            border_strong: alpha(0x301423, 0.40),
            vivo: hex(0xd6336c),
            primary: hex(0x391a2c),
            primary_fg: hex(0xf5e8dc),
            rail: hex(0x391a2c),
            rail_fg: hex(0xf5e8dc),
            rail_mute: hex(0xc9b0bd),
            rail_sel: hex(0xffc6df),
            rail_line: alpha(0xf5e8dc, 0.14),
            term: hex(0x1d0f17),
            term_fg: hex(0xf2e6d9),
            term_mute: hex(0xbca7b4),
            ok: hex(0x006b40),
            warn: hex(0x8a4c00),
            bad: hex(0xb71d00),
            info: hex(0x006585),
        }
    }

    pub fn dark() -> Self {
        Self {
            bg: hex(0x12090e),
            fg: hex(0xf2e6d9),
            card: hex(0x190d14),
            muted: hex(0x25161f),
            muted_fg: hex(0xbca7b4),
            border: alpha(0xf2e6d9, 0.18),
            border_strong: alpha(0xf2e6d9, 0.40),
            vivo: hex(0xff8fb8),
            primary: hex(0xffc6df),
            primary_fg: hex(0x391a2c),
            rail: hex(0x190d14),
            rail_fg: hex(0xf2e6d9),
            rail_mute: hex(0xa8909d),
            rail_sel: hex(0xffc6df),
            rail_line: alpha(0xf5e8dc, 0.14),
            term: hex(0x0d060a),
            term_fg: hex(0xf2e6d9),
            term_mute: hex(0xbca7b4),
            ok: hex(0x5fd39a),
            warn: hex(0xf6b84d),
            bad: hex(0xff6d53),
            info: hex(0x6cc6e6),
        }
    }

    pub fn tone(&self, tone: Tone) -> Hsla {
        match tone {
            Tone::Neutral => self.muted_fg,
            Tone::Vivo => self.vivo,
            Tone::Ok => self.ok,
            Tone::Warn => self.warn,
            Tone::Bad => self.bad,
            Tone::Info => self.info,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fio_traz_a_transparencia_do_design() {
        let p = Palette::light();
        assert!((p.border.a - 0.18).abs() < 1e-6);
        assert!((p.border_strong.a - 0.40).abs() < 1e-6);
    }

    #[test]
    fn dois_modos_diferem_no_fundo_e_o_trilho_segue_escuro() {
        let (l, d) = (Palette::light(), Palette::dark());
        assert_ne!(l.bg, d.bg);
        assert!(l.rail.l < 0.2 && d.rail.l < 0.2);
    }

    #[test]
    fn tom_neutro_usa_texto_secundario() {
        let p = Palette::dark();
        assert_eq!(p.tone(Tone::Neutral), p.muted_fg);
        assert_eq!(p.tone(Tone::Bad), p.bad);
    }
}
