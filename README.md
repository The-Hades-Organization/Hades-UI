# Hades-UI

Tema e componentes gpui compartilhados pelos apps do Hades.

## Crates

- `crates/hades-ui`: lib gpui com o tema do Hades e os componentes base; exemplo `gallery`.

## Estado

F0, em construção. Tema e componentes base prontos; nada específico de ferramenta entra aqui.

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
| `TopBar` | barra de topo escura, título centralizado |
| `Rail`, `RailGroup`, `RailItem` | navegação lateral agrupada, com item ativo e contador; `on_select` (clique e Enter/Espaço, foco visível) com o estado no chamador |
| `PageHeader` | título da página e ações à direita |
| `Card` | cartão com fio e título em caixa alta |
| `StatusBadge` | selo de estado (`Tone`: neutro, vivo, ok, warn, bad, info) |
| `DataTable` | tabela de texto com linha selecionável, ordenação e coluna numérica |
| `KpiStrip`, `Kpi` | faixa de indicadores |
| `Meter` | medidor de 6 px |
| `TermBlock` | bloco de terminal |
| `EmptyState` | estado vazio |
| `StatusBar` | barra de estado no pé da janela |
| `Button` | botão quadrado, primário ou secundário; `disabled(motivo)` mostra o motivo no tooltip |
| `Segmented` | escolha de uma opção entre poucas; estado no chamador (`selected`, `on_select`) |
| `FilterBar` | chips de filtro que ligam e desligam; estado no chamador |
| `PathField` | caminho somente leitura com "Escolher" no diálogo de arquivo do sistema (`on_pick`) |
| `MarkdownView` | documento markdown somente leitura: cabeçalhos `#` a `###`, listas e código inline; o resto aparece como texto |
| `Timeline` | linha do tempo: hora em mono e evento; `sourced` acrescenta selo da fonte (a cor só vai no selo) |
| `TextField` | campo de texto (`InputState` do gpui-component) com rótulo e erro inline; a validação é do chamador |
| `CopyButton` | copia o texto para a área de transferência do gpui e mostra "Copiado" por 1,5 s; sem IO na thread da UI |
| `Confirm` | diálogo de confirmação sobre a janela, com alvo em mono; renderize por último, só quando aberto |

`DataTable` aceita ainda `Align::Numeric` (direita, mono), `sorted` + `on_sort` (seta e clique no cabeçalho) e
`on_select` (clique na linha). A ordenação e a seleção são do chamador.

Tokens em `hades_ui::tokens`: `Palette` (claro e escuro), `Tone`, escalas `size` e `space`, fontes e medidas fixas.

## Fontes e licença

Figtree (400, 600), Marcellus (400) e Geist Mono (400, 600) vão embutidas no binário; `init` as registra e o tema já
usa Figtree e Geist Mono. Nenhuma instalação de fonte no sistema. Origem, commit e como as instâncias foram geradas:
`crates/hades-ui/assets/fonts/README.md`. As três famílias são SIL OFL 1.1 (texto em `OFL-<Família>.txt` na mesma
pasta), compatível com a licença MIT do crate; ao redistribuir o binário, mantenha esses avisos.

## Relação com o Hades-Skill

Faz parte da reescrita em Rust das ferramentas do [Hades-Skill](https://github.com/The-Hades-Organization/Hades-Skill). O Hades-Skill continua sendo a fonte até o corte, quando passa a chamar o binário.

## Licença

MIT. Veja `LICENSE`.
