# Fontes embutidas

Registradas por `hades_ui::fonts::register` (chamada por `hades_ui::init`) com `include_bytes!`.

Origem: <https://github.com/google/fonts>, commit `06bfe0d62a98375cb09c942fa18e68bd32cff057`
(`ofl/figtree`, `ofl/marcellus`, `ofl/geistmono`). Licença SIL OFL 1.1 de cada família em `OFL-<Família>.txt`.

| Arquivo | Família | Peso | Origem |
|---|---|---|---|
| `Figtree-Regular.ttf` | Figtree | 400 | instância estática de `Figtree[wght].ttf` |
| `Figtree-SemiBold.ttf` | Figtree | 600 | instância estática de `Figtree[wght].ttf` |
| `Marcellus-Regular.ttf` | Marcellus | 400 | `Marcellus-Regular.ttf` |
| `GeistMono-Regular.ttf` | Geist Mono | 400 | instância estática de `GeistMono[wght].ttf` |
| `GeistMono-SemiBold.ttf` | Geist Mono | 600 | instância estática de `GeistMono[wght].ttf` |

Os pesos são os que os componentes usam (normal e `SEMIBOLD`). Figtree e Geist Mono só existem como fonte variável no
`google/fonts`; as instâncias foram geradas com `fontTools.varLib.instancer` (`wght=400` e `wght=600`, nomes atualizados)
e, na de 600, o nome de família (ID 1) foi ajustado para a própria família, para o text system escolher o peso pelo
`OS/2.usWeightClass`. Para trocar o peso ou a versão, repita o processo e atualize esta tabela.
