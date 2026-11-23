# Identidade visual do Hades no Hades-UI

Fonte da verdade: o painel web (`~/.claude/skills/hades/kit/panel/ui/index.html`, `app.tsx`, `DESIGN.md` da pasta).
Onde o `DESIGN.md` e o CSS divergem, vale o CSS. Componente "falta" = ainda não existe no `hades-ui`.

## Tokens

### Cores (claro / escuro)

| Token CSS | Claro | Escuro | Campo `Palette` |
|---|---|---|---|
| `--background` | `#f8ecf1` | `#0e050a` | `bg` |
| `--foreground` | `#301423` | `#f2e6d9` | `fg` |
| `--card` | `#fff8fb` | `#190d14` | `card` |
| `--popover` = `--muted` = `--secondary` | `#f0dfe6` | `#25161f` | `popover`, `muted` |
| `--muted-foreground` | `#684b5b` | `#bca7b4` | `muted_fg` |
| `--line` (= `--input`) | `#926e7e` | `#876678` | `line` |
| `--border` | line 32% | line 32% | `border` |
| `--border-strong` | line 58% | line 58% | `border_strong` |
| `--hover` | fg 9% translúcido | fg 11% | `hover` |
| `--accent` | fg 9% sobre card | fg 11% sobre card | `accent` |
| `--primary` (= `--ring`, `--vivo`) | `#391a2c` | `#ffc6df` | `primary`, `vivo` |
| `--primary-foreground` | `#f5e8dc` | `#391a2c` | `primary_fg` |
| `--primary-hover` | primary 84% sobre bg | idem | `primary_hover` |
| `--secondary-hover` | fg 14% sobre secondary | idem | `secondary_hover` |
| `--danger-soft` / `-hover` | bad 14% / 18% sobre card | idem | `danger_soft`, `danger_soft_hover` |
| `--control-off` | line 18% sobre card | idem | `control_off` |
| `--busy` | = fg | = fg | `busy` |
| `--metal` | `#ffc6df` | `#ffc6df` | `metal` (= `rail_sel`) |
| `--metal-ink` | oklch(0.6 0.1 350) ≈ `#ad6689` | `#ffc6df` | `metal_ink` |
| `--sidebar` | `#391a2c` | `#190d14` | `rail` |
| `--sidebar-foreground` | `#f5e8dc` | `#f2e6d9` | `rail_fg` |
| `--sidebar-accent` | creme 12% sobre `#391a2c` | `#25161f` | `rail_accent` |
| `--sidebar-border` | creme 12% | line 32% | `rail_line` |
| `--rail-muted` | `#d7bfcb` | `#bca7b4` | `rail_mute` |
| `--ok` `--warn` `--bad` `--info` | `#006b40` `#8a4c00` `#b71d00` `#006585` | `#61d19a` `#f6b84d` `#ff6d53` `#5bc1d9` | `ok` `warn` `bad` `info` |
| `--term` / `-foreground` / `-muted` | `#1d0f17` / `#f2e6d9` / `#bca7b4` | `#0b0408` / idem | `term*` |
| `--term-bad` / `--term-warn` | `#ff6d53` / `#f6b84d` | idem | `term_bad`, `term_warn` |

### Forma, sombra, tipografia

- Raio 0 em tudo; só ponto de estado e giro são redondos.
- `--elev`: `0 12px 32px -8px rgb(0 0 0/.7)` (menu, tooltip, toast); diálogo `0 24px 60px -12px rgb(0 0 0/.8)`.
- Colchetes de canto (`cantos`): 10 px, traço 2 px, sup. esq. e inf. dir., cor `--line` (diálogo: primary; erro: bad).
- Linha de base de campo: `inset 0 -2px 0 --line`; foco → primary; inválido → bad com fundo bad 12%.
- Escala (px): micro 11 · meta 12 · label 13 · body 14 · prose 15 · section 18 · section-lg 21 · display 24 · page-title 40.
- Papéis: label 13/20 peso 500; meta-em 12/16 500; micro 11/16 500; títulos Marcellus 400.
- Casca: sidebar 248 (recolhida 64), topo 48, controle 32 (xs 28, lg 40, icon 36).

### Movimento

| Efeito | Duração | Curva |
|---|---|---|
| cor/fundo de controle | 150 ms | ease |
| fade-in (tooltip, popover, fundo de modal) | 250 ms | ease-out |
| emergir (toast, erro) | 450 ms | cubic-bezier(.2,.7,.2,1), sobe 8 px + blur 7 px |
| desfazer (saída) | 600 ms | ease-in |
| zoom-in | 180 ms | cubic-bezier(.16,1,.3,1) |
| slide lateral / toast-in | 300 / 350 ms | cubic-bezier(.16,1,.3,1) |
| shimmer (skeleton) | 1,6 s linear infinito | |
| spin (LoaderCircle) | — | |
| pulso (warn/bloqueado) | 2,4 s ease-in-out, opacidade .35 | |
| fio fiando | 1,8 s cubic-bezier(.45,0,.55,1) | |
| luz da sidebar 350 ms ease; óbolo 420 ms cubic-bezier(.2,.7,.2,1); ponto-vivo 900 ms cubic-bezier(.16,1,.3,1) | | |
| flash (dado que mudou) | 1,2 s ease-out, warn 25% → transparente | |

Movimento reduzido: tudo parado (`cx.reduce_motion()` no gpui).

## Elementos

| Elemento | Seletor de origem | Normal | Hover | Foco | Pressionado | Desabilitado | Carregando | `hades-ui` |
|---|---|---|---|---|---|---|---|---|
| Botão default | `BTN_V.default` | bg primary, texto primary-fg, h 32, px 14, label 13/500 | primary-hover | outline 2 px tracejado ring, offset 3 | translate-y 1 px | opacidade .6 + hachura 135° fg 16% | giro + cor mantida, cursor progress | `Button` |
| Botão secondary/outline | `BTN_V.secondary` | bg secondary, texto fg | secondary-hover | idem | idem | idem | idem | `Button` |
| Botão ghost | `BTN_V.ghost` | transparente | bg hover | idem | idem | idem | idem | `Button` |
| Botão destructive | `BTN_V.destructive` | danger-soft, texto bad | danger-soft-hover | idem | idem | idem | idem | `Button` |
| Botão ghost-destructive | | texto bad | danger-soft | | | | | `Button` |
| Botão link | `BTN_V.link` | sublinhado line | sublinhado primary | | | | | `Button` |
| Selo | `BADGE_V` | cheio na cor do estado, texto bg, meta-em 600, px 8 py 2 | — | — | — | — | busy: giro | `StatusBadge` |
| Ponto | `Dot` | 8 px redondo; idle muted 40% | — | — | — | — | warn: pulso | falta → `Dot` |
| Card | `Card`, `cantos` | card 96%, sem borda, colchetes line, p 20 gap 16 (object p 14 gap 10) | — | — | — | — | — | `Card` |
| Campo | `campo` | popover, linha de base line | fg 9% sobre popover | linha primary | — | opacidade .55 | — | `TextField` |
| Segmentado | `Segmented` | h 32, label 600 muted | bg hover, texto fg | outline ring | — | — | — | `Segmented` |
| Segmentado selecionado | | bg secondary + linha de base primary | | | | | | `Segmented` |
| Switch | `Switch` | 44×22, control-off + linha de base, botão 12 px muted | — | outline ring | — | opacidade .55 | — | falta → `Switch` |
| Tabela | `Th/Td/Tr` | th micro 600 caixa-alta muted h 40; zebra secondary 55% | linha bg hover | — | — | — | — | `DataTable` |
| Menu | `Menu` | popover, p 4, sombra elev + linha de base primary, fade-in | item bg hover | item bg hover | — | opacidade .5 | — | falta → `Menu`, `ContextMenu` |
| Item perigoso | | texto bad | danger-soft | | | | | `Menu` |
| Tooltip | `Tip` | popover, px 10 py 6, meta, sombra 0 10 30 -10, fade-in | — | — | — | — | — | falta → `Tip` |
| Toast | `ToastItem` | popover, barra 5 px na cor do tipo, py 12, label + body muted, emergir | fechar: muted → fg | | | | busy: giro | falta → `Toast` |
| Skeleton | `Skeleton` | muted com faixa clara, shimmer 1,6 s | — | — | — | — | — | falta → `Skeleton` |
| Giro | `I.LoaderCircle animate-spin` | arco de 3/4, traço 2 px ponta seca | — | — | — | — | — | falta → `Spinner` |
| Kbd | `Kbd` | muted, micro mono, linha de base 1 px line | — | — | — | — | — | falta → `Kbd` |
| Progresso | `Progress` | 4 px, trilho border, barra fg ou estado, 700 ms ease-out | — | — | — | — | — | `Meter` |
| Fio | `.fio[data-s]` | parado 1 px border-strong; medido 1,5 px fg; fiando 3 px busy + brilho; cortado bad | — | — | — | — | fiando | falta → `Fio` |
| Sidebar | `Sidebar` | bg sidebar, item h 32 label, ícone rail-muted | sidebar-accent | outline metal −2 | — | — | — | `Rail` |
| Item atual | `.nav-atual` | gradiente metal 24% → transparente + filete 3 px metal, 600 | | | | | | `Rail` |
| Luz / rio / óbolo | `.luz .rio .obolo` | ponto 7 px metal, opacidade max(.16, 1 − d·.17); rio 1 px rail-muted 26%; óbolo 9 px com halo | | | | | | `Rail` |
| Cabeçalho de página | `h1` | Marcellus 40 (`PAGE_TITLE`) | | | | | | `PageHeader` |
| Rótulo de grupo | | micro 600 caixa-alta tracking .14em rail-muted | | | | | | `Rail` |
| Topo | `header` | h 48, bg 90%, linha de base 2 px primary | | | | | | `TopBar` |
| Terminal | `bg-term` | term, code 12 mono; linhas `term-warn` / `term-bad` | | | | | | `TermBlock` |
| Diálogo de confirmação | `Dialog` | card, sombra de diálogo, colchetes primary, título Marcellus 18 | | | | | | `Confirm` |
| Filtros | chips | h 28, label 600 muted; ativo popover + linha de base primary | bg hover, texto fg | | | | | `FilterBar` |
| Estado vazio | | card 96%, colchetes line, título Marcellus 18 | | | | | | `EmptyState` |

## Divergências do gpui

O `gpui-pre =0.3.11` e o `gpui-component =0.9.0` não cobrem tudo o que o CSS do painel faz. Cada linha diz o que o painel faz e a aproximação usada no `hades-ui`.

| Painel (CSS) | `hades-ui` | Onde |
|---|---|---|
| Foco: `outline` 2 px tracejado `--ring`, offset 3 | Duas sombras sólidas: anel de 3 px na cor do fundo e anel de 2 px `--ring` por fora | `Button`, `Switch`, `Segmented` |
| Pressionado: `translate-y` 1 px | `mt 1 / mb -1` enquanto ativo | `Button` |
| `letter-spacing: .14em` no rótulo de grupo | `Tracked`: uma caixa por letra com gap de em×tamanho, pois o gpui 0.3.11 não expõe `letter-spacing` | `Rail`, marca |
| Emergir: sobe 8 px com `blur` 7 px | Sobe 8 px com opacidade, sem blur | `Toast` |
| Figtree 500 (label, meta-em) | Cai em 400 ou 600, os pesos embutidos | todos os rótulos |
| `cursor: progress` no carregando | Cursor padrão; o giro indica a espera | `Button` |
| Gradiente com 3 ou mais paradas | O gpui só pinta `linear_gradient` de 2 paradas; shimmer e fio usam duas meias-faixas | `Skeleton`, `Fio` |
| Ícone do tipo no toast | Ícone do tipo por padrão; `Toast::icon` troca | `Toast` |
| Óbolo desliza até o item atual (420 ms) | Óbolo fica no item atual, sem deslizar: o `RenderOnce` não guarda a posição anterior | `Rail` |
| Correnteza animada no rio da sidebar | Rio estático, 1 px `rail-muted` 26% | `Rail` |
| Navegação por setas no menu | Só ponteiro e clique; `Esc` fecha | `Menu`, `ContextMenu` |
| Progresso anima a largura em 700 ms | Largura muda sem transição: o `RenderOnce` não guarda o valor anterior | `Meter` |
| Topo com `backdrop-filter: blur` sobre fundo 90% | Fundo 90% sem blur | `TopBar` |
| Fundo do modal com `fade-in` | Fundo preto 50% sem fade | `Confirm` |
| Ícones Lucide no rail, nos botões, no menu e no toast | Módulo `icons` (171 SVGs embutidos, Lucide ISC e próprios), cor herdada do texto; `RailItem::icon`, `Button::icon`, `Toast::icon`, `Menu::icon(índice)` e `ContextMenu::icon(índice)` (por índice, porque `MenuEntry::Item` tem campos públicos e um campo novo quebraria a API) | `Rail`, `Button`, `Toast`, `Menu` |
| Marca "HADES" no topo da sidebar com tracking largo | `RailBrand` com `Rail::brand`: glifo, nome em Marcellus espaçado e subtítulo | `Rail` |
| Flash amarelo no dado que mudou (1,2 s) | Não portado; fica para quando um app precisar | — |
