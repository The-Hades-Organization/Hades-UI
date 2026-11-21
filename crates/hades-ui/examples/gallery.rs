//! Mostra tema e componentes base. O botão do cabeçalho alterna claro e escuro; `--dark` abre no escuro.
use gpui::prelude::FluentBuilder as _;
use gpui::{
    App, AppContext, Context, IntoElement, ParentElement, Render, SharedString, Styled, Window,
    WindowOptions, div, px,
};
use gpui_component::{ActiveTheme as _, Sizable as _, button::Button};
use hades_ui::ThemeMode;
use hades_ui::Tone;
use hades_ui::components::{
    Align, Button as Btn, Card, Confirm, DataTable, EmptyState, FilterBar, Kpi, KpiStrip, Meter,
    PageHeader, PathField, Rail, RailGroup, RailItem, Segmented, SortDirection, StatusBadge,
    StatusBar, TermBlock, TopBar,
};

#[derive(Default)]
struct Gallery {
    escopo: usize,
    filtros: [bool; 3],
    caminho: Option<String>,
    ordem: (usize, bool),
    linha: usize,
    pagina: Option<SharedString>,
    confirmando: bool,
}

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
        let this = cx.entity();
        let controles = Card::new()
            .title("Controles")
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(10.0))
                    .child(
                        Segmented::new("escopo", ["projeto", "skill"])
                            .selected(self.escopo)
                            .on_select({
                                let this = this.clone();
                                move |i, _, cx| {
                                    this.update(cx, |g, cx| {
                                        g.escopo = i;
                                        cx.notify();
                                    })
                                }
                            }),
                    )
                    .child(Btn::new("profunda", "Profunda").disabled("Sem provedor configurado"))
                    .child(Btn::new("gravar", "Gravar no plano").primary().on_click({
                        let this = this.clone();
                        move |_, _, cx| {
                            this.update(cx, |g, cx| {
                                g.confirmando = true;
                                cx.notify();
                            })
                        }
                    })),
            )
            .child(
                PathField::new("plano")
                    .placeholder("Escolha o plan.md")
                    .prompt("Abrir")
                    .on_pick({
                        let this = this.clone();
                        move |path, cx| {
                            this.update(cx, |g, cx| {
                                g.caminho = Some(path.display().to_string());
                                cx.notify();
                            })
                        }
                    })
                    .when_some(self.caminho.clone(), |f, v| f.value(v)),
            )
            .child(
                FilterBar::new()
                    .label("Papel")
                    .chip("impl", self.filtros[0])
                    .chip("review", self.filtros[1])
                    .chip("consolidate", self.filtros[2])
                    .on_toggle({
                        let this = this.clone();
                        move |i, _, cx| {
                            this.update(cx, |g, cx| {
                                g.filtros[i] = !g.filtros[i];
                                cx.notify();
                            })
                        }
                    }),
            )
            .child({
                let (col, desc) = self.ordem;
                let mut linhas = vec![
                    ["pedidos: disparo", "12,4"],
                    ["cameras: grade", "8,1"],
                    ["emergencies", "31,7"],
                ];
                linhas.sort_by(|a, b| {
                    let o = if col == 0 {
                        a[0].cmp(b[0])
                    } else {
                        a[1].replace(',', ".")
                            .parse::<f64>()
                            .unwrap_or(0.0)
                            .total_cmp(&b[1].replace(',', ".").parse::<f64>().unwrap_or(0.0))
                    };
                    if desc { o.reverse() } else { o }
                });
                let mut t = DataTable::new([
                    ("Unidade".into(), Align::Start),
                    ("Custo (US$)".into(), Align::Numeric),
                ])
                .sorted(
                    col,
                    if desc {
                        SortDirection::Descending
                    } else {
                        SortDirection::Ascending
                    },
                )
                .selected(self.linha)
                .on_sort({
                    let this = this.clone();
                    move |c, _, cx| {
                        this.update(cx, |g, cx| {
                            g.ordem = (c, g.ordem.0 == c && !g.ordem.1);
                            cx.notify();
                        })
                    }
                })
                .on_select({
                    let this = this.clone();
                    move |i, _, cx| {
                        this.update(cx, |g, cx| {
                            g.linha = i;
                            cx.notify();
                        })
                    }
                });
                for l in linhas {
                    t = t.row([l[0].into(), l[1].into()]);
                }
                t
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
        .selected(self.pagina.clone().unwrap_or_else(|| "Visão geral".into()))
        .on_select(cx.listener(|this, label: &SharedString, _, cx| {
            this.pagina = Some(label.clone());
            cx.notify();
        }));
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
            .child(controles)
            .child(EmptyState::new("Nenhum agente ativo").hint("Recrute um pelo Argos."));
        let dialogo = self.confirmando.then(|| {
            let fechar = {
                let this = this.clone();
                move |_: &gpui::ClickEvent, _: &mut Window, cx: &mut App| {
                    this.update(cx, |g, cx| {
                        g.confirmando = false;
                        cx.notify();
                    })
                }
            };
            Confirm::new("Gravar a estimativa no plano?")
                .detail("A seção de estimativa será reescrita; o resto do arquivo não muda.")
                .target("plans/hades-rust.md")
                .confirm_label("Gravar")
                .on_confirm(fechar.clone())
                .on_cancel(fechar)
        });
        div()
            .relative()
            .flex()
            .flex_col()
            .size_full()
            .child(TopBar::new("Hades UI · gallery"))
            .child(div().flex().flex_1().min_h_0().child(rail).child(main))
            .child(StatusBar::new("gallery", "hades-ui 0.1.0"))
            .children(dialogo)
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
            cx.new(|_| Gallery::default())
        });
        if aberta.is_err() {
            cx.quit();
        }
    });
}
