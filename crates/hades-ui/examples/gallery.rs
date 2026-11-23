//! Mostra tema e componentes base. O botão no pé do trilho alterna claro e escuro; `--dark` abre no escuro,
//! `--recolhida` abre com o trilho recolhido e `--fim` abre rolado até o fim (menu, toast e ícones).
use gpui::prelude::FluentBuilder as _;
use gpui::{
    App, AppContext, Context, InteractiveElement, IntoElement, ParentElement, Render, SharedString,
    StatefulInteractiveElement, Styled, Window, WindowOptions, div, px,
};
use gpui_component::input::{InputEvent, InputState};
use hades_ui::ThemeMode;
use hades_ui::Tone;
use hades_ui::components::{
    Align, Button as Btn, ButtonKind, ButtonSize, Card, Confirm, ContextMenu, CopyButton,
    DataTable, Dot, EmptyState, FilterBar, Fio, FioState, IconButton, Kbd, Kpi, KpiStrip,
    MarkdownView, Menu, MenuEntry, MetaList, Meter, Page, PageHeader, PathField, Rail, RailBrand,
    RailGroup, RailItem, RailStat, Segmented, Skeleton, SortDirection, Spinner, StatusBadge,
    StatusBar, Switch, TermBlock, TextField, Timeline, Tip, Toast,
};
use hades_ui::icons::{Icon, IconName};

#[derive(Default)]
struct Gallery {
    campo: Option<gpui::Entity<InputState>>,
    campo_erro: Option<SharedString>,
    escopo: usize,
    filtros: [bool; 3],
    caminho: Option<String>,
    ordem: (usize, bool),
    linha: usize,
    pagina: Option<SharedString>,
    confirmando: bool,
    chaves: [bool; 2],
    ultimo_menu: Option<SharedString>,
    rolagem: gpui::ScrollHandle,
    recolhida: bool,
    pulada: bool,
    rolar_ao_fim: bool,
}

impl Render for Gallery {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.rolar_ao_fim {
            if self.rolagem.max_offset().y > px(0.0) {
                self.rolagem.scroll_to_bottom();
                self.rolar_ao_fim = false;
            }
            window.request_animation_frame();
        }
        let campo = match self.campo.clone() {
            Some(c) => c,
            None => {
                let c = cx
                    .new(|cx| InputState::new(window, cx).placeholder("2026-11-20T10:00:00-03:00"));
                cx.subscribe(&c, |g: &mut Gallery, c, e: &InputEvent, cx| {
                    if matches!(e, InputEvent::Change) {
                        let v = c.read(cx).value();
                        g.campo_erro = (!v.is_empty() && !v.contains(['+', '-', 'Z']))
                            .then(|| "Intervalo sem fuso: use o offset, ex.: -03:00".into());
                        cx.notify();
                    }
                })
                .detach();
                self.campo = Some(c.clone());
                c
            }
        };
        let this = cx.entity();
        let estados = estados(self, &this);
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
                    RailItem::new("Visão geral").icon(IconName::LayoutDashboard),
                    RailItem::new("Serviços").icon(IconName::Server).count(0, 1),
                    RailItem::new("Orquestração")
                        .icon(IconName::Grafo)
                        .badge("3")
                        .alive(true),
                    RailItem::new("Fila de gates")
                        .icon(IconName::ListOrdered)
                        .badge("1"),
                    RailItem::new("Herdr")
                        .icon(IconName::SquareTerminal)
                        .subtitle("terminais e agentes"),
                ],
            ),
            RailGroup::new(
                "Ambiente",
                [RailItem::new("Configurações").icon(IconName::SlidersHorizontal)],
            ),
        ])
        .brand(RailBrand::new("Moirai").subtitle("planos e estimativas"))
        .stat(
            RailStat::new("Memória", "1,2 GiB · teto 4 GiB")
                .fraction(0.3)
                .tone(Tone::Ok),
        )
        .theme_toggle()
        .collapsed(self.recolhida)
        .on_collapse({
            let this = this.clone();
            move |recolher, _, cx| {
                this.update(cx, |g, cx| {
                    g.recolhida = recolher;
                    cx.notify();
                })
            }
        })
        .selected(self.pagina.clone().unwrap_or_else(|| "Visão geral".into()))
        .on_select(cx.listener(|this, label: &SharedString, _, cx| {
            this.pagina = Some(label.clone());
            cx.notify();
        }));
        let main = Page::new("main")
            .track_scroll(&self.rolagem)
            .header(
                PageHeader::new("Visão geral")
                    .subtitle("Estado do ambiente, agentes e filas num lugar só")
                    .description(
                        "Escolher só marca o recurso: nada roda até você confirmar. Os passos abaixo \
                         mostram o comando de cada um, com o botão de copiar embutido no bloco.",
                    )
                    .action(
                        Btn::new("tour", "Fazer o tour")
                            .kind(ButtonKind::Secondary)
                            .icon(IconName::Compass),
                    )
                    .action(
                        Btn::new("verificar", "Verificar de novo")
                            .kind(ButtonKind::Ghost)
                            .icon(IconName::RefreshCw),
                    )
                    .action(IconButton::new(
                        "congelar",
                        IconName::Snowflake,
                        "Congelar a tela",
                    ))
                    .meta("main @ 1a2b3c4")
                    .meta_icon(IconName::GitBranch),
            )
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
                            Card::new()
                                .title("PRs em voo")
                                .meta("2 abertas · 1 verde")
                                .action(StatusBadge::new("ao vivo", Tone::Ok))
                                .child(
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
                                .child(Meter::new(0.62).label("Janela 5h").value("62%"))
                                .child(
                                    Meter::new(0.71)
                                        .tone(Tone::Warn)
                                        .label("Janela semanal")
                                        .value("71% · renova em 2d"),
                                ),
                        ),
                    ),
            )
            .child(escolhas(self, &this))
            .child(
                Card::new()
                    .title("Blocos de código")
                    .meta("comando com copiar · saída de terminal")
                    .child(
                        TermBlock::new()
                            .line("/tmp/hades-demo/kit/tunnel.sh panel-start --porta 21220 --sem-navegador --perfil demo")
                            .copy("copiar-tunel", "/tmp/hades-demo/kit/tunnel.sh panel-start"),
                    )
                    .child(
                        TermBlock::new()
                            .line("$ hades fila status")
                            .dim("1 em execução · 0 na fila")
                            .warn("aviso: worktree sem hooks")
                            .bad("erro: gate barrado"),
                    )
                    .child(
                        TermBlock::new()
                            .terminal()
                            .line("$ hades fila status")
                            .dim("1 em execução · 0 na fila")
                            .warn("aviso: worktree sem hooks")
                            .bad("erro: gate barrado"),
                    ),
            )
            .child(
                Card::new()
                    .title("Post-mortem: o ready caía quando o consumer reiniciava e o rollout travava no meio")
                    .child(
                        MetaList::new()
                            .row("Tempo até a correção", "47 minutos, do primeiro alerta ao rollout verde no ambiente de demonstração")
                            .mono("Commit", "7e20d2b · fix: consumer reinicia sem derrubar o ready do serviço de demonstração")
                            .row("Fontes", "github, orbit, notas do recruta"),
                    ),
            )
            .child(estados)
            .child(controles)
            .child(
                Card::new()
                    .title("Documento, linha do tempo e campo")
                    .child(MarkdownView::new(
                        "## Resumo\n\nO `/health/ready` caía quando o consumer reiniciava.\n\n- rollout travava\n- `live` não mudou",
                    ))
                    .child(
                        Timeline::new()
                            .entry("13:02", "deploy do sha 7e20d2b")
                            .sourced("13:43", "PR #1234 aberta", "github", Tone::Info)
                            .sourced("13:50", "merge e rollout verde", "orbit", Tone::Ok),
                    )
                    .child(
                        TextField::new(&campo)
                            .label("Intervalo")
                            .error_opt(self.campo_erro.clone()),
                    )
                    .child(CopyButton::new("copiar", "! hades caronte pack plano.md")),
            )
            .child(
                div()
                    .flex()
                    .gap(px(12.0))
                    .h(px(260.0))
                    .child(
                        div().flex().flex_col().flex_1().child(
                            Card::new().title("Tabela que cresce").fill().child(
                                DataTable::new([
                                    ("Pacote".into(), Align::Start),
                                    ("Estado".into(), Align::End),
                                ])
                                .fill("tabela-vazia")
                                .empty("Nenhum pacote ainda"),
                            ),
                        ),
                    )
                    .child(
                        div().flex().flex_col().flex_1().child(
                            Card::new()
                                .title("Novo plano")
                                .fill()
                                .child(TextField::new(&campo).label("Intervalo"))
                                .child(div().flex_1())
                                .child(
                                    div().flex().justify_end().child(
                                        Btn::new("estacionar", "Estacionar")
                                            .primary()
                                            .icon(IconName::Pause)
                                            .disabled("Nenhum pacote selecionado"),
                                    ),
                                ),
                        ),
                    )
                    .child(
                        div().flex().flex_col().flex_1().child(
                            EmptyState::new("Nenhum agente ativo")
                                .hint("Recrute um pelo Argos.")
                                .fill(),
                        ),
                    ),
            );
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
            .child(div().flex().flex_1().min_h_0().child(rail).child(main))
            .child(
                StatusBar::new("gallery", "dados fictícios")
                    .version(concat!("hades-ui ", env!("CARGO_PKG_VERSION"))),
            )
            .children(dialogo)
    }
}

/// Dois cartões no molde do Onboarding do painel: selo de estado e ação no cabeçalho, `Meter` e `MetaList`
/// (com um `Meter` numa linha) dentro do cartão.
fn escolhas(g: &Gallery, this: &gpui::Entity<Gallery>) -> impl IntoElement {
    let alternar = {
        let this = this.clone();
        move |_: &gpui::ClickEvent, _: &mut Window, cx: &mut App| {
            this.update(cx, |g, cx| {
                g.pulada = !g.pulada;
                cx.notify();
            })
        }
    };
    let acesso = Card::new()
        .title("Acesso remoto")
        .meta("escolhida · 5 de 7 feitos")
        .description("Abrir o painel fora da máquina, inclusive no celular.")
        .status(StatusBadge::new("Escolhida", Tone::Ok).icon(IconName::Check))
        .action(
            Btn::new("pular", "Pular")
                .kind(ButtonKind::Ghost)
                .size(ButtonSize::Xs),
        )
        .child(Meter::new(5.0 / 7.0).label("Passos").value("5 de 7"))
        .child(
            MetaList::new()
                .label_width(120.0)
                .mono("Endereço", "https://exemplo.invalid/painel")
                .element("Janela 5h", Meter::new(0.38).label("uso").value("38%"))
                .row("Dono do passo", "você faz"),
        );
    let notificacoes = if g.pulada {
        Card::new()
            .title("Notificações")
            .meta("pulada · 1 de 4 feitos")
            .badge("pulada", Tone::Neutral)
            .action(
                Btn::new("escolher", "Escolher")
                    .icon(IconName::Plus)
                    .size(ButtonSize::Xs)
                    .on_click(alternar),
            )
    } else {
        Card::new()
            .title("Notificações")
            .meta("escolhida · 1 de 4 feitos")
            .status(StatusBadge::new("Escolhida", Tone::Ok).icon(IconName::Check))
            .action(
                Btn::new("escolher", "Pular")
                    .kind(ButtonKind::Ghost)
                    .size(ButtonSize::Xs)
                    .on_click(alternar),
            )
    }
    .description("Avisos de gate, serviço caído e review, no desktop e no celular.")
    .child(
        Meter::new(0.25)
            .tone(Tone::Warn)
            .label("Passos")
            .value("1 de 4"),
    );
    div()
        .flex()
        .gap(px(12.0))
        .child(div().flex_1().min_w_0().child(acesso))
        .child(div().flex_1().min_w_0().child(notificacoes))
}

fn linha(rotulo: &'static str) -> gpui::Div {
    div()
        .flex()
        .flex_wrap()
        .items_center()
        .gap(px(10.0))
        .child(div().w(px(110.0)).text_size(px(12.0)).child(rotulo))
}

fn estados(g: &Gallery, this: &gpui::Entity<Gallery>) -> impl IntoElement {
    let kinds = [
        ("Primary", ButtonKind::Primary),
        ("Secondary", ButtonKind::Secondary),
        ("Ghost", ButtonKind::Ghost),
        ("Destructive", ButtonKind::Destructive),
        ("Ghost destr.", ButtonKind::GhostDestructive),
        ("Link", ButtonKind::Link),
    ];
    let mut botoes = div().flex().flex_col().gap(px(8.0));
    for (i, (nome, kind)) in kinds.into_iter().enumerate() {
        let id = |s: &str| SharedString::from(format!("{s}-{i}"));
        botoes = botoes.child(
            linha(nome)
                .child(Btn::new(id("n"), "Normal").kind(kind).icon(IconName::Play))
                .child(
                    Btn::new(id("d"), "Desabilitado")
                        .kind(kind)
                        .disabled("Sem permissão"),
                )
                .child(Btn::new(id("l"), "Carregando").kind(kind).loading(true)),
        );
    }
    botoes = botoes.child(
        linha("Tamanhos")
            .child(Btn::new("xs", "Xs").size(ButtonSize::Xs))
            .child(Btn::new("md", "Md").size(ButtonSize::Md))
            .child(Btn::new("lg", "Lg").size(ButtonSize::Lg))
            .child(
                Btn::new("ic", "")
                    .size(ButtonSize::Icon)
                    .icon(IconName::Plus),
            ),
    );
    let chave = |i: usize| {
        let this = this.clone();
        move |v: bool, _: &mut Window, cx: &mut App| {
            this.update(cx, |g, cx| {
                g.chaves[i] = v;
                cx.notify();
            })
        }
    };
    let escolheu = |rotulo: &'static str| {
        let this = this.clone();
        move |_: &gpui::ClickEvent, _: &mut Window, cx: &mut App| {
            this.update(cx, |g, cx| {
                g.ultimo_menu = Some(rotulo.into());
                cx.notify();
            })
        }
    };
    let lista = {
        let a = escolheu("Abrir");
        let b = escolheu("Copiar caminho");
        let c = escolheu("Dispensar");
        move || {
            vec![
                MenuEntry::item("Abrir").on_click(a.clone()),
                MenuEntry::item("Copiar caminho").on_click(b.clone()),
                MenuEntry::item("Renomear").disabled(true),
                MenuEntry::separator(),
                MenuEntry::item("Dispensar").danger().on_click(c.clone()),
            ]
        }
    };
    let entradas = {
        let lista = lista.clone();
        move |_: &mut Window, _: &mut App| lista()
    };
    let tons = [
        (Tone::Neutral, "neutro"),
        (Tone::Vivo, "vivo"),
        (Tone::Ok, "ok"),
        (Tone::Warn, "warn"),
        (Tone::Bad, "bad"),
        (Tone::Info, "info"),
        (Tone::Busy, "busy"),
    ];
    let mut pontos = linha("Pontos");
    let mut selos = linha("Selos");
    for (i, (t, nome)) in tons.into_iter().enumerate() {
        pontos = pontos.child(Dot::new(("dot", i), t).pulse(t == Tone::Warn));
        selos = selos.child(StatusBadge::new(nome, t));
    }
    let mut toasts = div().flex().flex_col().gap(px(8.0)).w(px(380.0));
    for (i, (t, titulo)) in [
        (Tone::Ok, "Gate verde"),
        (Tone::Warn, "Recruta parado no teto"),
        (Tone::Bad, "Push recusado"),
        (Tone::Busy, "Compilando gpui"),
    ]
    .into_iter()
    .enumerate()
    {
        toasts = toasts.child(
            Toast::new(("toast", i), t, titulo).description("Detalhe em uma linha, cor muted."),
        );
    }
    Card::new()
        .title("Estados")
        .description(
            "Hover, foco (Tab) e pressionado são ao vivo; o resto está fixo em cada linha.",
        )
        .child(botoes)
        .child(
            linha("Switch")
                .child(
                    Switch::new("sw0", g.chaves[0])
                        .label("Ligado")
                        .on_change(chave(0)),
                )
                .child(
                    Switch::new("sw1", g.chaves[1])
                        .label("Desligado")
                        .on_change(chave(1)),
                )
                .child(
                    Switch::new("sw2", true)
                        .label("Desabilitado")
                        .disabled(true),
                ),
        )
        .child(selos)
        .child(pontos)
        .child(
            linha("Carregando")
                .child(Spinner::new("sp"))
                .child(Spinner::new("sp-lg").size(24.0))
                .child(Skeleton::new("sk1").width(180.0).height(14.0))
                .child(Skeleton::new("sk2").width(90.0).height(32.0)),
        )
        .child(
            linha("Fio")
                .child(div().w(px(120.0)).child(Fio::new("f0", FioState::Parado)))
                .child(div().w(px(120.0)).child(Fio::new("f1", FioState::Medido)))
                .child(div().w(px(120.0)).child(Fio::new("f2", FioState::Fiando)))
                .child(div().w(px(120.0)).child(Fio::new("f3", FioState::Cortado))),
        )
        .child(
            linha("Kbd e tooltip")
                .child(Kbd::new("Ctrl K"))
                .child(Kbd::new("Esc"))
                .child(
                    div()
                        .id("tip")
                        .px(px(8.0))
                        .child("passe o mouse")
                        .tooltip(Tip::text("Tooltip: popover, meta, sombra")),
                ),
        )
        .child(
            linha("Menu")
                .child(
                    Menu::new("menu-fixo", lista())
                        .icon(0, IconName::ExternalLink)
                        .icon(1, IconName::Copy)
                        .icon(2, IconName::Pencil)
                        .icon(4, IconName::Trash2),
                )
                .child(
                    ContextMenu::new(
                        "ctx",
                        div().px(px(12.0)).py(px(20.0)).child(match &g.ultimo_menu {
                            Some(m) => format!("Clique direito aqui · último: {m}"),
                            None => "Clique direito aqui".to_string(),
                        }),
                        entradas,
                    )
                    .icon(0, IconName::ExternalLink)
                    .icon(4, IconName::Trash2),
                ),
        )
        .child(linha("Toast").child(toasts))
        .child(
            linha("Ícones").child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(10.0))
                    .max_w(px(720.0))
                    .children(IconName::ALL.iter().map(|&i| Icon::new(i))),
            ),
        )
}

fn main() {
    let dark = std::env::args().any(|a| a == "--dark");
    let fim = std::env::args().any(|a| a == "--fim");
    let recolhida = std::env::args().any(|a| a == "--recolhida");
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
            cx.new(|_| Gallery {
                chaves: [true, false],
                recolhida,
                pulada: true,
                rolar_ao_fim: fim,
                ..Default::default()
            })
        });
        if aberta.is_err() {
            cx.quit();
        }
    });
}
