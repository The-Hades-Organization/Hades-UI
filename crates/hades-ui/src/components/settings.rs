use std::path::PathBuf;
use std::rc::Rc;

use gpui::{
    AnyElement, App, ClickEvent, ElementId, Entity, FontWeight, InteractiveElement, IntoElement,
    ParentElement, RenderOnce, SharedString, Styled, Window, div, px,
};
use gpui_component::input::InputState;

use super::{Button, ButtonKind, ButtonSize, Card, Dot, PathField, StatusBadge, Switch, TextField};
use crate::icons::IconName;
use crate::theme::palette;
use crate::tokens::{FONT_MONO, Palette, Tone, size};

type BoolHandler = Rc<dyn Fn(bool, &mut Window, &mut App)>;
type PickHandler = Rc<dyn Fn(PathBuf, &mut App)>;
type ClickHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>;

/// De onde veio o valor mostrado numa [`SettingRow`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SettingOrigin {
    /// Nada definido: vale o padrão da chave.
    #[default]
    Default,
    /// Valor gravado no arquivo de configuração.
    File,
    /// Variável de ambiente; vence o arquivo e deixa o campo só leitura.
    Env,
}

impl SettingOrigin {
    fn label(self) -> &'static str {
        match self {
            Self::Default => "padrão",
            Self::File => "arquivo",
            Self::Env => "ambiente",
        }
    }

    fn tone(self) -> Tone {
        match self {
            Self::Default => Tone::Neutral,
            Self::File => Tone::Info,
            Self::Env => Tone::Warn,
        }
    }
}

/// Valida um inteiro dentro de `min..=max` (tipo `int:min:max` do catálogo). Devolve a mensagem de erro, ou
/// `None` se o texto é válido; o chamador passa o resultado a [`SettingRow::error_opt`].
pub fn validate_int(text: &str, min: i64, max: i64) -> Option<String> {
    match text.trim().parse::<i64>() {
        Err(_) => Some("Informe um número inteiro".to_string()),
        Ok(n) if n < min || n > max => Some(format!("Use um valor entre {min} e {max}")),
        Ok(_) => None,
    }
}

enum Control {
    Switch(bool),
    Number {
        state: Entity<InputState>,
        min: i64,
        max: i64,
    },
    Text(Entity<InputState>),
    Path {
        value: Option<SharedString>,
        directories: bool,
    },
}

/// Linha de uma configuração, no molde da página Configurações do painel: rótulo, descrição e chave em mono à
/// esquerda; controle do tipo à direita, com selo de origem, erro em linha e "Restaurar padrão".
/// Sem estado e sem leitura de arquivo: o chamador guarda o valor, a origem e o erro e os passa a cada render.
/// Com origem [`SettingOrigin::Env`] o controle fica só leitura, com a dica "definido no ambiente".
#[derive(IntoElement)]
pub struct SettingRow {
    id: ElementId,
    label: SharedString,
    key: Option<SharedString>,
    description: Option<SharedString>,
    default: Option<SharedString>,
    control: Control,
    origin: SettingOrigin,
    modified: bool,
    error: Option<SharedString>,
    disabled: bool,
    on_change: Option<BoolHandler>,
    on_pick: Option<PickHandler>,
    on_reset: Option<ClickHandler>,
}

impl SettingRow {
    fn base(id: impl Into<ElementId>, label: impl Into<SharedString>, control: Control) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            key: None,
            description: None,
            default: None,
            control,
            origin: SettingOrigin::Default,
            modified: false,
            error: None,
            disabled: false,
            on_change: None,
            on_pick: None,
            on_reset: None,
        }
    }

    /// Interruptor (tipo `bool`).
    pub fn switch(id: impl Into<ElementId>, label: impl Into<SharedString>, checked: bool) -> Self {
        Self::base(id, label, Control::Switch(checked))
    }

    /// Número com faixa (tipo `int:min:max`). A faixa aparece como dica; validar é do chamador
    /// (veja [`validate_int`]).
    pub fn number(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        state: &Entity<InputState>,
        min: i64,
        max: i64,
    ) -> Self {
        let control = Control::Number {
            state: state.clone(),
            min,
            max,
        };
        Self::base(id, label, control)
    }

    /// Texto livre.
    pub fn text(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        state: &Entity<InputState>,
    ) -> Self {
        Self::base(id, label, Control::Text(state.clone()))
    }

    /// Caminho (tipo `path`), escolhido no diálogo do sistema; o valor volta por [`SettingRow::on_pick`].
    pub fn path(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        value: Option<SharedString>,
    ) -> Self {
        let control = Control::Path {
            value,
            directories: false,
        };
        Self::base(id, label, control)
    }

    /// Escolhe pasta em vez de arquivo (só para `path`).
    pub fn directories(mut self) -> Self {
        if let Control::Path { directories, .. } = &mut self.control {
            *directories = true;
        }
        self
    }

    /// Nome da chave (`HADES_..._X`), em mono discreto abaixo da descrição.
    pub fn key(mut self, key: impl Into<SharedString>) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Valor padrão, mostrado como "padrão <valor>" junto da chave.
    pub fn default_value(mut self, default: impl Into<SharedString>) -> Self {
        self.default = Some(default.into());
        self
    }

    pub fn origin(mut self, origin: SettingOrigin) -> Self {
        self.origin = origin;
        self
    }

    /// O valor atual difere do padrão: mostra "Restaurar padrão" (menos com origem ambiente ou desabilitada).
    pub fn modified(mut self, modified: bool) -> Self {
        self.modified = modified;
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

    /// Novo valor do interruptor.
    pub fn on_change(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(f));
        self
    }

    /// Caminho escolhido (só para `path`).
    pub fn on_pick(mut self, f: impl Fn(PathBuf, &mut App) + 'static) -> Self {
        self.on_pick = Some(Rc::new(f));
        self
    }

    /// Clique em "Restaurar padrão".
    pub fn on_reset(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_reset = Some(Rc::new(f));
        self
    }
}

impl RenderOnce for SettingRow {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let env = self.origin == SettingOrigin::Env;
        let locked = env || self.disabled;

        let mut title = div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(px(8.0))
            .child(
                div()
                    .text_size(px(size::LABEL))
                    .font_weight(FontWeight::MEDIUM)
                    .child(self.label.clone()),
            )
            .child(StatusBadge::new(self.origin.label(), self.origin.tone()));
        if self.modified && !env {
            title = title.child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .text_color(p.muted_fg)
                    .text_size(px(size::META))
                    .child(Dot::new((self.id.clone(), "alterado"), Tone::Info))
                    .child("fora do padrão"),
            );
        }
        let mut info = div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .gap(px(4.0))
            .child(title);
        if let Some(description) = self.description {
            info = info.child(
                div()
                    .text_color(p.muted_fg)
                    .text_size(px(size::BODY))
                    .child(description),
            );
        }
        let mut meta = div()
            .flex()
            .flex_wrap()
            .items_baseline()
            .gap_x(px(16.0))
            .text_size(px(size::META))
            .text_color(p.muted_fg);
        if let Some(key) = self.key {
            meta = meta.child(div().font_family(FONT_MONO).text_color(p.fg).child(key));
        }
        if let Some(default) = self.default.clone() {
            meta = meta.child(
                div()
                    .flex()
                    .gap(px(4.0))
                    .child("padrão")
                    .child(div().font_family(FONT_MONO).child(default)),
            );
        }
        if let Control::Number { min, max, .. } = &self.control {
            meta = meta.child(format!("de {min} a {max}"));
        }
        info = info.child(meta);

        let has_field = matches!(self.control, Control::Number { .. } | Control::Text(_));
        let control: AnyElement = match self.control {
            Control::Switch(checked) => {
                let mut sw = Switch::new((self.id.clone(), "sw"), checked).disabled(locked);
                if let Some(f) = self.on_change {
                    sw = sw.on_change(move |v, w, cx| f(v, w, cx));
                }
                sw.into_any_element()
            }
            Control::Number { state, .. } | Control::Text(state) => TextField::new(&state)
                .disabled(locked)
                .error_opt(self.error.clone())
                .into_any_element(),
            Control::Path { value, directories } => {
                if locked {
                    readonly_path(&p, value)
                } else {
                    let mut field = PathField::new((self.id.clone(), "path"));
                    if let Some(v) = value {
                        field = field.value(v);
                    }
                    if directories {
                        field = field.directories();
                    }
                    if let Some(f) = self.on_pick {
                        field = field.on_pick(move |path, cx| f(path, cx));
                    }
                    field.into_any_element()
                }
            }
        };

        let mut side = div()
            .flex()
            .flex_col()
            .flex_none()
            .w(px(300.0))
            .gap(px(6.0));
        side = side.child(control);
        if !has_field && let Some(error) = self.error {
            side = side.child(
                div()
                    .text_color(p.bad)
                    .text_size(px(size::META))
                    .child(error),
            );
        }
        if env {
            side = side.child(
                div()
                    .text_color(p.muted_fg)
                    .text_size(px(size::META))
                    .child("definido no ambiente"),
            );
        } else if self.modified
            && !self.disabled
            && let Some(f) = self.on_reset
        {
            side = side.child(
                div().flex().child(
                    Button::new((self.id.clone(), "reset"), "Restaurar padrão")
                        .kind(ButtonKind::Ghost)
                        .size(ButtonSize::Xs)
                        .icon(IconName::RotateCcw)
                        .on_click(move |e, w, cx| f(e, w, cx)),
                ),
            );
        }

        div()
            .id(self.id)
            .flex()
            .flex_wrap()
            .items_start()
            .justify_between()
            .gap_x(px(32.0))
            .gap_y(px(12.0))
            .py(px(14.0))
            .child(info)
            .child(side)
    }
}

fn readonly_path(p: &Palette, value: Option<SharedString>) -> AnyElement {
    div()
        .px(px(10.0))
        .py(px(7.0))
        .min_w_0()
        .overflow_hidden()
        .bg(p.popover)
        .shadow(vec![Palette::baseline(p.line, 2.0)])
        .opacity(0.55)
        .font_family(FONT_MONO)
        .text_size(px(size::META))
        .child(value.unwrap_or_else(|| "vazio".into()))
        .into_any_element()
}

/// Grupo de configurações: cartão com título e a lista de linhas separadas por filete.
#[derive(IntoElement)]
pub struct SettingsGroup {
    title: SharedString,
    meta: Option<SharedString>,
    description: Option<SharedString>,
    rows: Vec<AnyElement>,
}

impl SettingsGroup {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            meta: None,
            description: None,
            rows: Vec::new(),
        }
    }

    /// Resumo ao lado do título (ex.: "3 configurações · 1 fora do padrão").
    pub fn meta(mut self, meta: impl Into<SharedString>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn row(mut self, row: SettingRow) -> Self {
        self.rows.push(row.into_any_element());
        self
    }
}

impl RenderOnce for SettingsGroup {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let mut card = Card::new().title(self.title);
        if let Some(meta) = self.meta {
            card = card.meta(meta);
        }
        if let Some(description) = self.description {
            card = card.description(description);
        }
        let last = self.rows.len().saturating_sub(1);
        let mut list = div().flex().flex_col();
        for (i, row) in self.rows.into_iter().enumerate() {
            let mut cell = div().child(row);
            if i != last {
                cell = cell.border_b_1().border_color(p.border);
            }
            list = list.child(cell);
        }
        card.child(list)
    }
}

/// Barra de ações da página: "Salvar" e "Descartar" só ficam ativos com alteração pendente; o aviso à esquerda
/// diz quantas há, ou "Salvo" logo depois de gravar. O estado é do chamador.
#[derive(IntoElement)]
pub struct SettingsBar {
    pending: usize,
    saving: bool,
    saved: bool,
    on_save: Option<ClickHandler>,
    on_discard: Option<ClickHandler>,
}

impl SettingsBar {
    /// `pending`: quantas alterações ainda não foram salvas.
    pub fn new(pending: usize) -> Self {
        Self {
            pending,
            saving: false,
            saved: false,
            on_save: None,
            on_discard: None,
        }
    }

    /// Gravando: o "Salvar" gira e não aceita clique.
    pub fn saving(mut self, saving: bool) -> Self {
        self.saving = saving;
        self
    }

    /// Mostra o aviso "Salvo" (só aparece sem alteração pendente).
    pub fn saved(mut self, saved: bool) -> Self {
        self.saved = saved;
        self
    }

    pub fn on_save(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_save = Some(Rc::new(f));
        self
    }

    pub fn on_discard(mut self, f: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_discard = Some(Rc::new(f));
        self
    }
}

impl RenderOnce for SettingsBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let p = palette(cx);
        let dirty = self.pending > 0;
        let notice: AnyElement = if dirty {
            let text = if self.pending == 1 {
                "1 alteração não salva".to_string()
            } else {
                format!("{} alterações não salvas", self.pending)
            };
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(Dot::new("settings-bar-dot", Tone::Warn))
                .child(text)
                .into_any_element()
        } else if self.saved {
            StatusBadge::new("Salvo", Tone::Ok)
                .icon(IconName::Check)
                .into_any_element()
        } else {
            div().into_any_element()
        };
        let mut discard = Button::new("settings-discard", "Descartar").kind(ButtonKind::Ghost);
        let mut save = Button::new(
            "settings-save",
            if self.saving { "Salvando…" } else { "Salvar" },
        )
        .primary()
        .icon(IconName::Save)
        .loading(self.saving);
        if dirty {
            if let Some(f) = self.on_discard {
                discard = discard.on_click(move |e, w, cx| f(e, w, cx));
            }
            if let Some(f) = self.on_save {
                save = save.on_click(move |e, w, cx| f(e, w, cx));
            }
        } else {
            discard = discard.disabled("Nada a descartar");
            save = save.disabled("Nada a salvar");
        }
        div()
            .flex()
            .items_center()
            .gap(px(12.0))
            .px(px(16.0))
            .py(px(10.0))
            .bg(p.popover)
            .text_color(p.fg)
            .text_size(px(size::BODY))
            .shadow(vec![Palette::baseline(p.line, 2.0)])
            .child(div().flex_1().min_w_0().child(notice))
            .child(discard)
            .child(save)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inteiro_dentro_da_faixa_passa() {
        assert_eq!(validate_int("50", 10, 100), None);
        assert_eq!(validate_int(" 10 ", 10, 100), None);
        assert_eq!(validate_int("100", 10, 100), None);
    }

    #[test]
    fn inteiro_fora_da_faixa_ou_invalido_recusa() {
        assert_eq!(
            validate_int("9", 10, 100).as_deref(),
            Some("Use um valor entre 10 e 100")
        );
        assert_eq!(
            validate_int("101", 10, 100).as_deref(),
            Some("Use um valor entre 10 e 100")
        );
        assert!(validate_int("abc", 10, 100).is_some());
        assert!(validate_int("", 10, 100).is_some());
    }

    #[test]
    fn origem_ambiente_tem_tom_de_aviso() {
        assert_eq!(SettingOrigin::Env.tone(), Tone::Warn);
        assert_eq!(SettingOrigin::Default.label(), "padrão");
    }
}
