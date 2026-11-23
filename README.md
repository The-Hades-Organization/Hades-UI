# Hades-UI

Tema e componentes gpui compartilhados pelos apps do Hades.

## Crates

- `crates/hades-ui`: lib gpui com o tema do Hades e os componentes base; exemplo `gallery`.

## Estado

Tema e componentes com a identidade visual do painel web do Hades (inventário e divergências em
[`docs/identidade.md`](docs/identidade.md), capturas em `docs/identidade/`). Nada específico de ferramenta entra aqui.

## Uso

```rust
gpui_platform::application().run(|cx| {
    hades_ui::init(cx, hades_ui::ThemeMode::Light); // gpui-component + paleta do Hades
    // hades_ui::set_mode(cx, ThemeMode::Dark);     // troca de modo
});
```

`init` registra as fontes embutidas, chama `gpui_component::init` e sobrescreve as cores, o raio (0) e as fontes do tema dele com os tokens do
`DESIGN.md` (`~/.claude/skills/hades/DESIGN.md`). Botão, input, seletor e o resto vêm do `gpui-component` já com a
paleta. Os componentes próprios leem `hades_ui::palette(cx)`; para cor fora do que eles cobrem, use essa paleta e nunca
um hex solto.

Exemplo: `cargo run --example gallery` (`-- --dark` abre no escuro; o botão do cabeçalho alterna).

## Componentes

Só o que é genérico de UI e aparece em páginas de design diferentes.

| Componente | Papel |
|---|---|
| `TopBar` | barra de topo de 48 px, fundo da página e linha de base primary |
| `Rail`, `RailGroup`, `RailItem` | sidebar de 248 px: rio, luz por distância, item atual com gradiente metal e óbolo, contador e `alive`; `on_select` (clique e Enter/Espaço, foco visível) com o estado no chamador |
| `PageHeader` | título Marcellus 40 e ações à direita |
| `Card` | cartão sem borda, com colchetes de canto, título Marcellus, `description` e variante `object` |
| `StatusBadge` | selo cheio na cor do estado (`Tone`: neutro, vivo, ok, warn, bad, info, busy com giro) |
| `DataTable` | tabela de texto com linha selecionável, ordenação e coluna numérica |
| `KpiStrip`, `Kpi` | faixa de indicadores |
| `Meter` | barra de progresso de 4 px sobre trilho `--border` |
| `TermBlock` | bloco de terminal; `dim`, `warn` e `bad` por linha |
| `EmptyState` | estado vazio |
| `StatusBar` | barra de estado no pé da janela |
| `Button` | botão quadrado: `ButtonKind` (secondary, primary, ghost, destructive, ghost-destructive, link), `ButtonSize` (md, xs, lg, icon), `loading` com giro, `disabled(motivo)` com hachura e o motivo no tooltip |
| `Segmented` | escolha de uma opção entre poucas; estado no chamador (`selected`, `on_select`) |
| `FilterBar` | chips de filtro que ligam e desligam; estado no chamador |
| `PathField` | caminho somente leitura com "Escolher" no diálogo de arquivo do sistema (`on_pick`) |
| `MarkdownView` | documento markdown somente leitura: cabeçalhos `#` a `###`, listas e código inline; o resto aparece como texto |
| `Timeline` | linha do tempo: hora em mono e evento; `sourced` acrescenta selo da fonte (a cor só vai no selo) |
| `TextField` | campo de texto (`InputState` do gpui-component) com rótulo e erro inline; a validação é do chamador |
| `CopyButton` | copia o texto para a área de transferência do gpui e mostra "Copiado" por 1,5 s; sem IO na thread da UI |
| `Confirm` | diálogo de confirmação sobre a janela, com alvo em mono; renderize por último, só quando aberto |
| `Switch` | chave liga/desliga de 44×22 com deslize de 200 ms; estado no chamador |
| `Menu`, `MenuEntry`, `ContextMenu` | menu com itens, separador, item perigoso e desabilitado; `ContextMenu` abre no clique direito |
| `Tip` | tooltip do painel: `.tooltip(Tip::text("..."))` |
| `Toast` | aviso com barra de 5 px na cor do `Tone`, título, descrição e fechar; entra emergindo |
| `Spinner`, `Skeleton`, `Dot` | giro, placeholder com shimmer e ponto de estado (com pulso) |
| `Kbd` | tecla de atalho |
| `Fio`, `FioState` | fio parado, medido, fiando (animado) ou cortado |

`DataTable` aceita ainda `Align::Numeric` (direita, mono), `sorted` + `on_sort` (seta e clique no cabeçalho) e
`on_select` (clique na linha). A ordenação e a seleção são do chamador.

Tokens em `hades_ui::tokens`: `Palette` (claro e escuro), `Tone`, escalas `size`, `space` e `control`, fontes,
medidas fixas e `motion` (durações e curvas do painel).

## Migração de 0.1 para 0.2

- `Palette` ganhou campos (`line`, `popover`, `hover`, `accent`, `primary_hover`, `secondary_hover`, `danger_soft`,
  `danger_soft_hover`, `control_off`, `busy`, `metal`, `metal_ink`, `rail_accent`, `term_bad`, `term_warn`). Quem monta
  `Palette` por literal precisa preencher os novos; quem só lê `palette(cx)` não muda nada.
- `Tone` ganhou `Busy`. Um `match` exaustivo sobre `Tone` precisa do braço novo.
- `Button`: `.primary()` continua; as variantes novas entram por `.kind(ButtonKind::...)` e `.size(ButtonSize::...)`.
- Mudou o desenho, não a API: `StatusBadge` é cheio; `Card`, `TextField`, `PathField`, `EmptyState` e `KpiStrip` perderam
  a borda (colchetes de canto ou linha de base no lugar); `Rail` tem 248 px; `Meter` tem 4 px; `PageHeader` usa
  Marcellus 40; `TopBar` deixou de ser escura. Reveja as telas que dependiam dessas medidas.
- Componentes novos: `Switch`, `Menu`, `ContextMenu`, `Tip`, `Toast`, `Spinner`, `Skeleton`, `Dot`, `Kbd`, `Fio`.

## Fontes e licença

Figtree (400, 600), Marcellus (400) e Geist Mono (400, 600) vão embutidas no binário; `init` as registra e o tema já
usa Figtree e Geist Mono. Nenhuma instalação de fonte no sistema. Origem, commit e como as instâncias foram geradas:
`crates/hades-ui/assets/fonts/README.md`. As três famílias são SIL OFL 1.1 (texto em `OFL-<Família>.txt` na mesma
pasta), compatível com a licença MIT do crate; ao redistribuir o binário, mantenha esses avisos.

## Relação com o Hades-Skill

Faz parte da reescrita em Rust das ferramentas do [Hades-Skill](https://github.com/The-Hades-Organization/Hades-Skill). O Hades-Skill continua sendo a fonte até o corte, quando passa a chamar o binário.

## Licença

MIT. Veja `LICENSE`.
