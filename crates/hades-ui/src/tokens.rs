//! Tokens do painel web do Hades (`kit/panel/ui`): paleta nos dois modos, tipografia, forma e movimento.
//! O inventário com o seletor de origem de cada valor está em `docs/identidade.md`.
use gpui::{BoxShadow, Hsla, Rgba, point, px, rgb};

pub const FONT_SANS: &str = "Figtree";
pub const FONT_HEADING: &str = "Marcellus";
pub const FONT_MONO: &str = "Geist Mono";

pub mod size {
    pub const MICRO: f32 = 11.0;
    pub const META: f32 = 12.0;
    pub const LABEL: f32 = 13.0;
    pub const BODY: f32 = 14.0;
    pub const PROSE: f32 = 15.0;
    pub const SECTION: f32 = 18.0;
    pub const SECTION_LG: f32 = 21.0;
    pub const DISPLAY: f32 = 24.0;
    pub const PAGE_TITLE: f32 = 40.0;
}

pub mod space {
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 12.0;
    pub const LG: f32 = 16.0;
    pub const XL: f32 = 24.0;
}

/// Alturas de controle do painel (`h-8`, `h-7`, `h-10`, `size-9`).
pub mod control {
    pub const MD: f32 = 32.0;
    pub const XS: f32 = 28.0;
    pub const LG: f32 = 40.0;
    pub const ICON: f32 = 36.0;
}

/// O design é quadrado: só ponto de estado e giro são redondos.
pub const RADIUS: f32 = 0.0;
pub const TOP_BAR_HEIGHT: f32 = 48.0;
pub const STATUS_BAR_HEIGHT: f32 = 26.0;
pub const RAIL_WIDTH: f32 = 248.0;
pub const RAIL_WIDTH_COLLAPSED: f32 = 64.0;
/// Colchete de canto (`.cantos`): braço e traço.
pub const CORNER_ARM: f32 = 10.0;
pub const CORNER_STROKE: f32 = 2.0;

/// Durações e curvas do painel (`@keyframes` e `transition` do `index.html`).
pub mod motion {
    use std::time::Duration;

    pub const COLOR: Duration = Duration::from_millis(150);
    pub const FADE_IN: Duration = Duration::from_millis(250);
    pub const EMERGE: Duration = Duration::from_millis(450);
    pub const UNDO: Duration = Duration::from_millis(600);
    pub const ZOOM_IN: Duration = Duration::from_millis(180);
    pub const SLIDE: Duration = Duration::from_millis(300);
    pub const TOAST_IN: Duration = Duration::from_millis(350);
    pub const SHIMMER: Duration = Duration::from_millis(1600);
    pub const SPIN: Duration = Duration::from_millis(1000);
    pub const PULSE: Duration = Duration::from_millis(2400);
    pub const SPIN_THREAD: Duration = Duration::from_millis(1800);
    pub const RAIL_LIGHT: Duration = Duration::from_millis(350);
    pub const FLASH: Duration = Duration::from_millis(1200);
    pub const PROGRESS: Duration = Duration::from_millis(700);
    /// Deslocamento vertical do `emergir`.
    pub const EMERGE_RISE: f32 = 8.0;
    /// Opacidade mínima do `pulso`.
    pub const PULSE_MIN: f32 = 0.35;

    /// `cubic-bezier(x1, y1, x2, y2)` do CSS: resolve `x(t) = progresso` por Newton e bissecção e devolve `y(t)`.
    pub fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32) -> impl Fn(f32) -> f32 + Clone {
        move |x: f32| {
            let x = x.clamp(0.0, 1.0);
            let bez = |t: f32, a: f32, b: f32| {
                let u = 1.0 - t;
                3.0 * u * u * t * a + 3.0 * u * t * t * b + t * t * t
            };
            let (mut lo, mut hi, mut t) = (0.0_f32, 1.0_f32, x);
            for _ in 0..24 {
                let v = bez(t, x1, x2) - x;
                if v.abs() < 1e-5 {
                    break;
                }
                if v > 0.0 {
                    hi = t;
                } else {
                    lo = t;
                }
                t = (lo + hi) / 2.0;
            }
            bez(t, y1, y2)
        }
    }

    /// `ease` do CSS.
    pub fn ease() -> impl Fn(f32) -> f32 + Clone {
        cubic_bezier(0.25, 0.1, 0.25, 1.0)
    }

    /// `ease-out` do CSS.
    pub fn ease_out() -> impl Fn(f32) -> f32 + Clone {
        cubic_bezier(0.0, 0.0, 0.58, 1.0)
    }

    /// `ease-in` do CSS.
    pub fn ease_in() -> impl Fn(f32) -> f32 + Clone {
        cubic_bezier(0.42, 0.0, 1.0, 1.0)
    }

    /// `ease-in-out` do CSS.
    pub fn ease_in_out() -> impl Fn(f32) -> f32 + Clone {
        cubic_bezier(0.42, 0.0, 0.58, 1.0)
    }

    /// Curva do `emergir` e do `óbolo`: `cubic-bezier(.2,.7,.2,1)`.
    pub fn emerge() -> impl Fn(f32) -> f32 + Clone {
        cubic_bezier(0.2, 0.7, 0.2, 1.0)
    }

    /// Curva do `zoom-in`, do slide e do `toast-in`: `cubic-bezier(.16,1,.3,1)`.
    pub fn snap() -> impl Fn(f32) -> f32 + Clone {
        cubic_bezier(0.16, 1.0, 0.3, 1.0)
    }

    /// Curva do fio fiando: `cubic-bezier(.45,0,.55,1)`.
    pub fn spin_thread() -> impl Fn(f32) -> f32 + Clone {
        cubic_bezier(0.45, 0.0, 0.55, 1.0)
    }

    /// Ida e volta do `pulso`: opacidade 1 → [`PULSE_MIN`] → 1 com `ease-in-out`.
    pub fn pulse(t: f32) -> f32 {
        let half = if t < 0.5 { t * 2.0 } else { (1.0 - t) * 2.0 };
        1.0 - (1.0 - PULSE_MIN) * ease_in_out()(half)
    }
}

/// Cor com função: o estado que um selo, ponto ou medidor comunica.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tone {
    Neutral,
    Vivo,
    Ok,
    Warn,
    Bad,
    Info,
    /// Em andamento (`--busy`): mesma cor do texto, com giro.
    Busy,
}

/// Paleta completa de um modo, com as misturas `color-mix` do CSS já resolvidas.
#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub bg: Hsla,
    pub fg: Hsla,
    pub card: Hsla,
    pub popover: Hsla,
    /// `--muted` = `--secondary`.
    pub muted: Hsla,
    pub muted_fg: Hsla,
    /// `--line` (= `--input`): base dos fios e da linha de base dos campos.
    pub line: Hsla,
    /// `line` a 32%.
    pub border: Hsla,
    /// `line` a 58%.
    pub border_strong: Hsla,
    /// Fundo translúcido de hover (`fg` a 9% no claro, 11% no escuro).
    pub hover: Hsla,
    /// Hover opaco sobre o card.
    pub accent: Hsla,
    pub primary: Hsla,
    pub primary_fg: Hsla,
    pub primary_hover: Hsla,
    /// Fundo do botão primário desabilitado (= `muted`): sobre ele a hachura aparece, o que não acontece
    /// sobre o `primary` esmaecido.
    pub primary_disabled: Hsla,
    /// Texto do botão primário desabilitado (= `muted_fg`).
    pub primary_disabled_fg: Hsla,
    pub secondary_hover: Hsla,
    pub danger_soft: Hsla,
    pub danger_soft_hover: Hsla,
    /// Trilho do switch desligado.
    pub control_off: Hsla,
    pub busy: Hsla,
    /// `--vivo` = `--primary` = `--ring`.
    pub vivo: Hsla,
    pub metal: Hsla,
    /// Metal legível sobre fundo claro.
    pub metal_ink: Hsla,
    pub rail: Hsla,
    pub rail_fg: Hsla,
    pub rail_accent: Hsla,
    pub rail_line: Hsla,
    pub rail_mute: Hsla,
    /// Item atual do trilho (= `metal`).
    pub rail_sel: Hsla,
    pub term: Hsla,
    pub term_fg: Hsla,
    pub term_mute: Hsla,
    pub term_bad: Hsla,
    pub term_warn: Hsla,
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

/// `color-mix(in srgb, a p, b)`: `p` de `a` sobre `b`, opaco.
fn mix(a: u32, p: f32, b: u32) -> Hsla {
    let (a, b) = (rgb(a), rgb(b));
    let ch = |x: f32, y: f32| x * p + y * (1.0 - p);
    Rgba {
        r: ch(a.r, b.r),
        g: ch(a.g, b.g),
        b: ch(a.b, b.b),
        a: 1.0,
    }
    .into()
}

struct Base {
    bg: u32,
    fg: u32,
    card: u32,
    popover: u32,
    line: u32,
    primary: u32,
    bad: u32,
    hover_pct: f32,
}

/// Misturas do `:root` que dependem só das cores base.
struct Derived {
    line: Hsla,
    border: Hsla,
    border_strong: Hsla,
    hover: Hsla,
    accent: Hsla,
    primary_hover: Hsla,
    secondary_hover: Hsla,
    danger_soft: Hsla,
    danger_soft_hover: Hsla,
    control_off: Hsla,
}

impl Base {
    fn derived(&self) -> Derived {
        Derived {
            line: hex(self.line),
            border: alpha(self.line, 0.32),
            border_strong: alpha(self.line, 0.58),
            hover: alpha(self.fg, self.hover_pct),
            accent: mix(self.fg, self.hover_pct, self.card),
            primary_hover: mix(self.primary, 0.84, self.bg),
            secondary_hover: mix(self.fg, 0.14, self.popover),
            danger_soft: mix(self.bad, 0.14, self.card),
            danger_soft_hover: mix(self.bad, 0.18, self.card),
            control_off: mix(self.line, 0.18, self.card),
        }
    }
}

impl Palette {
    pub fn light() -> Self {
        let base = Base {
            bg: 0xf8ecf1,
            fg: 0x301423,
            card: 0xfff8fb,
            popover: 0xf0dfe6,
            line: 0x926e7e,
            primary: 0x391a2c,
            bad: 0xb71d00,
            hover_pct: 0.09,
        };
        let d = base.derived();
        Self {
            bg: hex(base.bg),
            fg: hex(base.fg),
            card: hex(base.card),
            popover: hex(base.popover),
            muted: hex(base.popover),
            muted_fg: hex(0x684b5b),
            line: d.line,
            border: d.border,
            border_strong: d.border_strong,
            hover: d.hover,
            accent: d.accent,
            primary: hex(base.primary),
            primary_fg: hex(0xf5e8dc),
            primary_hover: d.primary_hover,
            primary_disabled: hex(base.popover),
            primary_disabled_fg: hex(0x684b5b),
            secondary_hover: d.secondary_hover,
            danger_soft: d.danger_soft,
            danger_soft_hover: d.danger_soft_hover,
            control_off: d.control_off,
            busy: hex(base.fg),
            vivo: hex(base.primary),
            metal: hex(0xffc6df),
            metal_ink: hex(0xad6689),
            rail: hex(0x391a2c),
            rail_fg: hex(0xf5e8dc),
            rail_accent: mix(0xf5e8dc, 0.12, 0x391a2c),
            rail_line: alpha(0xf5e8dc, 0.12),
            rail_mute: hex(0xd7bfcb),
            rail_sel: hex(0xffc6df),
            term: hex(0x1d0f17),
            term_fg: hex(0xf2e6d9),
            term_mute: hex(0xbca7b4),
            term_bad: hex(0xff6d53),
            term_warn: hex(0xf6b84d),
            ok: hex(0x006b40),
            warn: hex(0x8a4c00),
            bad: hex(base.bad),
            info: hex(0x006585),
        }
    }

    pub fn dark() -> Self {
        let base = Base {
            bg: 0x0e050a,
            fg: 0xf2e6d9,
            card: 0x190d14,
            popover: 0x25161f,
            line: 0x876678,
            primary: 0xffc6df,
            bad: 0xff6d53,
            hover_pct: 0.11,
        };
        let d = base.derived();
        Self {
            bg: hex(base.bg),
            fg: hex(base.fg),
            card: hex(base.card),
            popover: hex(base.popover),
            muted: hex(base.popover),
            muted_fg: hex(0xbca7b4),
            line: d.line,
            border: d.border,
            border_strong: d.border_strong,
            hover: d.hover,
            accent: d.accent,
            primary: hex(base.primary),
            primary_fg: hex(0x391a2c),
            primary_hover: d.primary_hover,
            primary_disabled: hex(base.popover),
            primary_disabled_fg: hex(0xbca7b4),
            secondary_hover: d.secondary_hover,
            danger_soft: d.danger_soft,
            danger_soft_hover: d.danger_soft_hover,
            control_off: d.control_off,
            busy: hex(base.fg),
            vivo: hex(base.primary),
            metal: hex(0xffc6df),
            metal_ink: hex(0xffc6df),
            rail: hex(0x190d14),
            rail_fg: hex(0xf2e6d9),
            rail_accent: hex(0x25161f),
            rail_line: alpha(base.line, 0.32),
            rail_mute: hex(0xbca7b4),
            rail_sel: hex(0xffc6df),
            term: hex(0x0b0408),
            term_fg: hex(0xf2e6d9),
            term_mute: hex(0xbca7b4),
            term_bad: hex(0xff6d53),
            term_warn: hex(0xf6b84d),
            ok: hex(0x61d19a),
            warn: hex(0xf6b84d),
            bad: hex(base.bad),
            info: hex(0x5bc1d9),
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
            Tone::Busy => self.busy,
        }
    }

    /// `--elev`: sombra de menu, tooltip e toast.
    pub fn elevation(&self) -> BoxShadow {
        BoxShadow::new(px(0.0), px(12.0), Hsla::black().opacity(0.7))
            .blur_radius(px(32.0))
            .spread_radius(px(-8.0))
    }

    /// Sombra do diálogo.
    pub fn dialog_shadow(&self) -> BoxShadow {
        BoxShadow::new(px(0.0), px(24.0), Hsla::black().opacity(0.8))
            .blur_radius(px(60.0))
            .spread_radius(px(-12.0))
    }

    /// Linha de base de campo (`inset 0 -2px 0 <cor>`).
    pub fn baseline(color: Hsla, width: f32) -> BoxShadow {
        BoxShadow {
            color,
            offset: point(px(0.0), px(-width)),
            blur_radius: px(0.0),
            spread_radius: px(0.0),
            inset: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: Hsla, b: u32) -> bool {
        let (a, b) = (Rgba::from(a), rgb(b));
        (a.r - b.r).abs() < 0.01 && (a.g - b.g).abs() < 0.01 && (a.b - b.b).abs() < 0.01
    }

    #[test]
    fn fio_traz_a_transparencia_do_painel() {
        let p = Palette::light();
        assert!((p.border.a - 0.32).abs() < 1e-6);
        assert!((p.border_strong.a - 0.58).abs() < 1e-6);
        assert!(close(p.line, 0x926e7e));
    }

    #[test]
    fn cores_base_batem_com_o_css() {
        let (l, d) = (Palette::light(), Palette::dark());
        assert!(close(l.bg, 0xf8ecf1) && close(d.bg, 0x0e050a));
        assert!(close(l.primary, 0x391a2c) && close(d.primary, 0xffc6df));
        assert!(close(l.metal_ink, 0xad6689));
        assert!(l.rail.l < 0.2 && d.rail.l < 0.2);
    }

    #[test]
    fn primario_desabilitado_usa_muted_nos_dois_modos() {
        for p in [Palette::light(), Palette::dark()] {
            assert_eq!(p.primary_disabled, p.muted);
            assert_eq!(p.primary_disabled_fg, p.muted_fg);
            assert_ne!(p.primary_disabled, p.primary);
        }
    }

    #[test]
    fn mistura_srgb_resolve_como_o_navegador() {
        // color-mix(in srgb, #391a2c 84%, #f8ecf1) → canal r = 0.84·0x39 + 0.16·0xf8.
        let p = Palette::light();
        let r = Rgba::from(p.primary_hover).r * 255.0;
        assert!((r - (0.84 * 57.0 + 0.16 * 248.0)).abs() < 1.0);
        assert!((p.hover.a - 0.09).abs() < 1e-6 && (Palette::dark().hover.a - 0.11).abs() < 1e-6);
    }

    #[test]
    fn tom_neutro_usa_texto_secundario_e_busy_o_texto() {
        let p = Palette::dark();
        assert_eq!(p.tone(Tone::Neutral), p.muted_fg);
        assert_eq!(p.tone(Tone::Bad), p.bad);
        assert_eq!(p.tone(Tone::Busy), p.fg);
    }

    #[test]
    fn curvas_cubic_bezier_tem_extremos_fixos_e_sao_monotonas() {
        let curves: [Box<dyn Fn(f32) -> f32>; 4] = [
            Box::new(motion::ease()),
            Box::new(motion::emerge()),
            Box::new(motion::snap()),
            Box::new(motion::spin_thread()),
        ];
        for f in curves {
            assert!(f(0.0).abs() < 1e-3 && (f(1.0) - 1.0).abs() < 1e-3);
            assert!(f(0.25) <= f(0.5) && f(0.5) <= f(0.75));
        }
        assert!((motion::ease_in_out()(0.5) - 0.5).abs() < 1e-3);
    }

    #[test]
    fn pulso_vai_ao_minimo_no_meio_e_volta() {
        assert!((motion::pulse(0.0) - 1.0).abs() < 1e-3);
        assert!((motion::pulse(0.5) - motion::PULSE_MIN).abs() < 1e-3);
        assert!((motion::pulse(1.0) - 1.0).abs() < 1e-3);
    }
}
