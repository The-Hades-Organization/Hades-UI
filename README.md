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
    // hades_ui::use_brand_fonts(cx);               // só com Figtree e Geist Mono instaladas
});
```

`init` chama `gpui_component::init` e sobrescreve as cores, o raio (0) e as fontes do tema dele com os tokens do
`DESIGN.md` (`~/.claude/skills/hades/DESIGN.md`). Botão, input, seletor e o resto vêm do `gpui-component` já com a
paleta. Os componentes próprios leem `hades_ui::palette(cx)`; para cor fora do que eles cobrem, use essa paleta e nunca
um hex solto.

Exemplo: `cargo run --example gallery` (`-- --dark` abre no escuro; o botão do cabeçalho alterna).

## Componentes

Só o que aparece em ao menos duas das sete páginas de design.

| Componente | Papel |
|---|---|
| `TopBar` | barra de topo escura, título centralizado |
| `Rail`, `RailGroup`, `RailItem` | navegação lateral agrupada, com item ativo e contador |
| `PageHeader` | título da página e ações à direita |
| `Card` | cartão com fio e título em caixa alta |
| `StatusBadge` | selo de estado (`Tone`: neutro, vivo, ok, warn, bad, info) |
| `DataTable` | tabela de texto com linha selecionável |
| `KpiStrip`, `Kpi` | faixa de indicadores |
| `Meter` | medidor de 6 px |
| `TermBlock` | bloco de terminal |
| `EmptyState` | estado vazio |
| `StatusBar` | barra de estado no pé da janela |

Tokens em `hades_ui::tokens`: `Palette` (claro e escuro), `Tone`, escalas `size` e `space`, fontes e medidas fixas.

## Relação com o Hades-Skill

Faz parte da reescrita em Rust das ferramentas do [Hades-Skill](https://github.com/The-Hades-Organization/Hades-Skill). O Hades-Skill continua sendo a fonte até o corte, quando passa a chamar o binário.

## Licença

MIT. Veja `LICENSE`.
