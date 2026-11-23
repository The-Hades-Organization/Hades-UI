//! Liga a paleta do Hades ao tema do `gpui-component`: só sobrescreve o que o tema dele já tem lugar para guardar.
use gpui::{App, Global, Hsla, SharedString};
use gpui_component::{Theme, ThemeMode};

use crate::tokens::{FONT_MONO, FONT_SANS, Palette, RADIUS, size};

/// Paleta ativa; os componentes do `hades-ui` leem daqui.
#[derive(Clone, Copy)]
struct ActivePalette(Palette, bool);

impl Global for ActivePalette {}

/// Paleta do modo ativo. Antes de [`init`] devolve a clara.
pub fn palette(cx: &App) -> Palette {
    cx.try_global::<ActivePalette>()
        .map(|p| p.0)
        .unwrap_or_else(Palette::light)
}

/// Registra as fontes embutidas, inicializa o `gpui-component` e instala o tema do Hades no modo pedido.
pub fn init(cx: &mut App, mode: ThemeMode) {
    crate::fonts::register(cx);
    gpui_component::init(cx);
    set_mode(cx, mode);
    use_brand_fonts(cx);
}

/// Troca de modo e reaplica a paleta (o `Theme::change` recarrega o tema registrado por cima).
pub fn set_mode(cx: &mut App, mode: ThemeMode) {
    Theme::change(mode, None, cx);
    let palette = if mode.is_dark() {
        Palette::dark()
    } else {
        Palette::light()
    };
    cx.set_global(ActivePalette(palette, mode.is_dark()));
    Theme::update(cx, |theme| apply(theme, &palette));
}

/// Alterna entre claro e escuro e devolve o modo novo; é o que o [`ThemeToggle`](crate::components::ThemeToggle) chama.
pub fn toggle_mode(cx: &mut App) -> ThemeMode {
    let next = if is_dark(cx) {
        ThemeMode::Light
    } else {
        ThemeMode::Dark
    };
    set_mode(cx, next);
    next
}

/// Se o tema ativo é o escuro.
pub fn is_dark(cx: &App) -> bool {
    cx.try_global::<ActivePalette>().is_some_and(|p| p.1)
}

/// Nomeia Figtree e Geist Mono no tema. `init` já chama; use só depois de trocar o tema por fora.
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
    c.input = p.line;
    c.ring = p.primary;
    c.caret = p.primary;
    c.selection = p.vivo.opacity(0.28);
    c.muted = p.muted;
    c.muted_foreground = p.muted_fg;
    c.accent = p.accent;
    c.accent_foreground = p.fg;
    c.popover = p.popover;
    c.popover_foreground = p.fg;
    c.group_box = p.card;
    c.group_box_foreground = p.fg;
    c.primary = p.primary;
    c.primary_hover = p.primary_hover;
    c.primary_active = p.primary_hover;
    c.primary_foreground = p.primary_fg;
    c.secondary = p.muted;
    c.secondary_hover = p.secondary_hover;
    c.secondary_active = p.secondary_hover;
    c.secondary_foreground = p.fg;
    c.danger_hover = p.danger_soft_hover;
    c.danger_active = p.danger_soft_hover;
    c.link = p.fg;
    c.link_hover = p.primary;
    c.link_active = p.primary;
    c.success = p.ok;
    c.warning = p.warn;
    c.danger = p.bad;
    c.info = p.info;
    c.sidebar = p.rail;
    c.sidebar_foreground = p.rail_fg;
    c.sidebar_border = p.rail_line;
    c.sidebar_accent = p.rail_accent;
    c.sidebar_accent_foreground = p.rail_fg;
    c.sidebar_primary = p.metal;
    c.sidebar_primary_foreground = p.rail;
    c.title_bar = p.bg;
    c.title_bar_border = p.border;
    c.status_bar = p.bg;
    c.status_bar_border = p.border;
    c.table = p.card;
    c.table_even = p.muted.opacity(0.55);
    c.table_head = p.card;
    c.table_head_foreground = p.muted_fg;
    c.table_hover = p.hover;
    c.table_active = p.accent;
    c.table_row_border = p.border;
    c.list = p.popover;
    c.list_hover = p.hover;
    c.list_active = p.accent;
    c.list_active_border = p.primary;
    c.progress_bar = p.fg;
    c.skeleton = p.muted;
    c.switch = p.control_off;
    c.tab_active = p.muted;
    c.overlay = Hsla::black().opacity(0.5);

    theme.radius = gpui::px(RADIUS);
    theme.radius_lg = gpui::px(RADIUS);
    theme.shadow = false;
    theme.font_family = SharedString::from(FONT_SANS);
    theme.mono_font_family = SharedString::from(FONT_MONO);
    theme.font_size = gpui::px(size::BODY);
    theme.mono_font_size = gpui::px(size::META);
}
