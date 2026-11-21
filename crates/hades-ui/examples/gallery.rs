//! Mostra tema e componentes base. O botão do cabeçalho alterna claro e escuro; `--dark` abre no escuro.
use gpui::{
    App, AppContext, Context, IntoElement, ParentElement, Render, Styled, Window, WindowOptions,
    div, px,
};
use gpui_component::{ActiveTheme as _, Sizable as _, button::Button};
use hades_ui::ThemeMode;
use hades_ui::Tone;
use hades_ui::components::{
    Align, Card, DataTable, EmptyState, Kpi, KpiStrip, Meter, PageHeader, Rail, RailGroup,
    RailItem, StatusBadge, StatusBar, TermBlock, TopBar,
};

struct Gallery;

impl Render for Gallery {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dark = cx.theme().mode.is_dark();
        let toggle = Button::new("modo")
            .small()
            .outline()
            .label(if dark { "Claro" } else { "Escuro" })
            .on_click(|_, _, cx| {
                let next = if cx.theme().mode.is_dark() {
                    ThemeMode::Light
                } else {
                    ThemeMode::Dark
                };
                hades_ui::set_mode(cx, next);
            });
        let rail = Rail::new([
            RailGroup::new(
                "Hades",
                [
                    RailItem::new("Visão geral"),
                    RailItem::new("Orquestração").badge("3"),
                    RailItem::new("Fila de gates").badge("1"),
                ],
            ),
            RailGroup::new("Ambiente", [RailItem::new("Configurações")]),
        ])
        .selected("Visão geral");
        let main = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(px(14.0))
            .p(px(18.0))
            .bg(cx.theme().background)
            .child(PageHeader::new("Visão geral").action(toggle))
            .child(KpiStrip::new([
                Kpi::new("Agentes ativos", "7"),
                Kpi::new("Pedem você", "2").tone(Tone::Vivo),
                Kpi::new("Fila de gates", "1 · 4m12"),
                Kpi::new("Janela 5h", "38%"),
            ]))
            .child(
                div()
                    .flex()
                    .gap(px(12.0))
                    .child(
                        div().flex_1().child(
                            Card::new().title("PRs em voo").child(
                                DataTable::new([
                                    ("PR".into(), Align::Start),
                                    ("Título".into(), Align::Start),
                                    ("Estado".into(), Align::End),
                                ])
                                .row([
                                    "#1234".into(),
                                    "DEMO-12 ajuste de saude".into(),
                                    "verde".into(),
                                ])
                                .row(["#1235".into(), "pedidos: disparo".into(), "review".into()])
                                .selected(0),
                            ),
                        ),
                    )
                    .child(
                        div().flex_1().child(
                            Card::new()
                                .title("Estados")
                                .child(
                                    div()
                                        .flex()
                                        .gap(px(8.0))
                                        .child(StatusBadge::new("aguarda", Tone::Vivo))
                                        .child(StatusBadge::new("verde", Tone::Ok))
                                        .child(StatusBadge::new("review", Tone::Warn))
                                        .child(StatusBadge::new("barrado", Tone::Bad))
                                        .child(StatusBadge::new("issue", Tone::Info))
                                        .child(StatusBadge::new("neutro", Tone::Neutral)),
                                )
                                .child(Meter::new(0.62))
                                .child(Meter::new(0.71).tone(Tone::Warn)),
                        ),
                    ),
            )
            .child(
                TermBlock::new()
                    .line("$ hades gate-lock status")
                    .dim("1 em execução · 0 na fila"),
            )
            .child(EmptyState::new("Nenhum agente ativo").hint("Recrute um pelo Argos."));
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(TopBar::new("Hades UI · gallery"))
            .child(div().flex().flex_1().min_h_0().child(rail).child(main))
            .child(StatusBar::new("gallery", "hades-ui 0.1.0"))
    }
}

fn main() {
    let dark = std::env::args().any(|a| a == "--dark");
    gpui_platform::application().run(move |cx: &mut App| {
        hades_ui::init(
            cx,
            if dark {
                ThemeMode::Dark
            } else {
                ThemeMode::Light
            },
        );
        let aberta = cx.open_window(WindowOptions::default(), |window, cx| {
            window.set_window_title("Hades UI");
            cx.new(|_| Gallery)
        });
        if aberta.is_err() {
            cx.quit();
        }
    });
}
