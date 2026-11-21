//! Liga a paleta do Hades ao tema do `gpui-component`: só sobrescreve o que o tema dele já tem lugar para guardar.
use gpui::{App, Global, SharedString};
use gpui_component::{Theme, ThemeMode};

use crate::tokens::{FONT_MONO, FONT_SANS, Palette, RADIUS, size};

/// Paleta ativa; os componentes do `hades-ui` leem daqui.
#[derive(Clone, Copy)]
struct ActivePalette(Palette);

impl Global for ActivePalette {}

/// Paleta do modo ativo. Antes de [`init`] devolve a clara.
pub fn palette(cx: &App) -> Palette {
    cx.try_global::<ActivePalette>()
        .map(|p| p.0)
        .unwrap_or_else(Palette::light)
}

/// Inicializa o `gpui-component` e instala o tema do Hades no modo pedido.
pub fn init(cx: &mut App, mode: ThemeMode) {
    gpui_component::init(cx);
    set_mode(cx, mode);
}

/// Troca de modo e reaplica a paleta (o `Theme::change` recarrega o tema registrado por cima).
pub fn set_mode(cx: &mut App, mode: ThemeMode) {
    Theme::change(mode, None, cx);
    let palette = if mode.is_dark() {
        Palette::dark()
    } else {
        Palette::light()
    };
    cx.set_global(ActivePalette(palette));
    Theme::update(cx, |theme| apply(theme, &palette));
}

/// Nomeia Figtree e Geist Mono. Só chame com as fontes instaladas ou embutidas: nome ausente cai no fallback do gpui.
pub fn use_brand_fonts(cx: &mut App) {
    Theme::update(cx, |theme| {
        theme.font_family = SharedString::from(FONT_SANS);
        theme.mono_font_family = SharedString::from(FONT_MONO);
    });
}

fn apply(theme: &mut Theme, p: &Palette) {
    let c = &mut theme.colors;
    c.background = p.bg;
    c.foreground = p.fg;
    c.border = p.border;
    c.input = p.border_strong;
    c.ring = p.vivo;
    c.caret = p.vivo;
    c.selection = p.vivo.opacity(0.25);
    c.muted = p.muted;
    c.muted_foreground = p.muted_fg;
    c.accent = p.muted;
    c.accent_foreground = p.fg;
    c.popover = p.card;
    c.popover_foreground = p.fg;
    c.group_box = p.card;
    c.group_box_foreground = p.fg;
    c.primary = p.primary;
    c.primary_hover = p.primary.opacity(0.9);
    c.primary_active = p.primary.opacity(0.8);
    c.primary_foreground = p.primary_fg;
    c.secondary = p.muted;
    c.secondary_hover = p.muted.opacity(0.8);
    c.secondary_active = p.muted.opacity(0.7);
    c.secondary_foreground = p.fg;
    c.link = p.info;
    c.link_hover = p.info.opacity(0.85);
    c.link_active = p.info.opacity(0.7);
    c.success = p.ok;
    c.warning = p.warn;
    c.danger = p.bad;
    c.info = p.info;
    c.sidebar = p.rail;
    c.sidebar_foreground = p.rail_fg;
    c.sidebar_border = p.rail_line;
    c.sidebar_accent = p.rail_sel.opacity(0.14);
    c.sidebar_accent_foreground = p.rail_sel;
    c.sidebar_primary = p.rail_sel;
    c.sidebar_primary_foreground = p.rail;
    c.title_bar = p.rail;
    c.title_bar_border = p.rail_line;
    c.status_bar = p.bg;
    c.status_bar_border = p.border;
    c.table = p.card;
    c.table_even = p.card;
    c.table_head = p.card;
    c.table_head_foreground = p.muted_fg;
    c.table_hover = p.muted;
    c.table_active = p.muted;
    c.table_row_border = p.border;
    c.list = p.card;
    c.list_hover = p.muted;
    c.list_active = p.muted;
    c.list_active_border = p.vivo;
    c.progress_bar = p.fg;
    c.skeleton = p.muted;

    theme.radius = gpui::px(RADIUS);
    theme.radius_lg = gpui::px(RADIUS);
    theme.shadow = false;
    theme.font_size = gpui::px(size::BODY);
    theme.mono_font_size = gpui::px(size::META);
}
