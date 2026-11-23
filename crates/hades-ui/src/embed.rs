//! Contrato para o painel embutir a view de uma ferramenta sem o trilho dela.
//!
//! Cada ferramenta desenha o próprio `Rail` e abre a própria janela. Para entrar no trilho do painel, a view da
//! ferramenta implementa [`FerramentaEmbutida`]: as [`TelaInfo`] viram itens do trilho do painel, os [`Comando`]
//! entram na paleta (Ctrl K) e `render_tela` desenha só o miolo de uma tela. O contrato mora aqui, e não no
//! painel, para as ferramentas não dependerem dele.

use gpui::{AnyElement, Context, Window};

use crate::icons::IconName;

/// Opções de quando a view roda dentro do painel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Embutida {
    /// As Configurações vêm do painel: a ferramenta não mostra a tela de Configurações própria.
    pub integrado: bool,
}

/// Uma tela da ferramenta, como item do trilho do painel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TelaInfo {
    /// Estável entre versões: o painel guarda a tela aberta por ele.
    pub id: &'static str,
    pub rotulo: &'static str,
    pub icone: IconName,
}

/// Uma ação da ferramenta, como item da paleta de comandos.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Comando {
    /// Estável entre versões: o painel o devolve em [`FerramentaEmbutida::executar`].
    pub id: &'static str,
    /// Cabeçalho do grupo na paleta (ex.: "Fila", "Ir para").
    pub grupo: &'static str,
    pub rotulo: &'static str,
    /// Linha curta que também entra na busca.
    pub descricao: Option<&'static str>,
    pub icone: IconName,
    /// Atalho mostrado à direita, como o painel o escreve (ex.: "Ctrl R").
    pub atalho: Option<&'static str>,
}

/// O que a ferramenta expõe ao painel: nome e ícone no trilho, telas e comandos.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FerramentaInfo {
    /// Estável entre versões (ex.: "moirai").
    pub id: &'static str,
    pub titulo: &'static str,
    pub icone: IconName,
    pub telas: &'static [TelaInfo],
    pub comandos: &'static [Comando],
}

impl FerramentaInfo {
    /// Tela de id `id`, se a ferramenta a tiver.
    pub fn tela(&self, id: &str) -> Option<&'static TelaInfo> {
        self.telas.iter().find(|t| t.id == id)
    }

    /// Comando de id `id`, se a ferramenta o tiver.
    pub fn comando(&self, id: &str) -> Option<&'static Comando> {
        self.comandos.iter().find(|c| c.id == id)
    }

    /// Ids repetidos entre as telas ou entre os comandos; vazio quando o contrato está coerente.
    pub fn ids_repetidos(&self) -> Vec<&'static str> {
        let mut out = Vec::new();
        let telas: Vec<&'static str> = self.telas.iter().map(|t| t.id).collect();
        let comandos: Vec<&'static str> = self.comandos.iter().map(|c| c.id).collect();
        for ids in [telas, comandos] {
            for (i, id) in ids.iter().enumerate() {
                if ids[..i].contains(id) && !out.contains(id) {
                    out.push(*id);
                }
            }
        }
        out
    }
}

/// View de ferramenta que o painel embute. A ferramenta segue com a janela própria; dentro do painel, o painel
/// desenha o trilho e chama `render_tela` com a tela escolhida nele.
pub trait FerramentaEmbutida: Sized + 'static {
    /// O que a ferramenta precisa para abrir (ambiente, pasta do projeto), no tipo dela.
    type Ambiente;

    /// Nome, ícone, telas e comandos. Constante: o painel monta o trilho e a paleta antes de abrir a view.
    fn info() -> FerramentaInfo;

    /// Abre a view para rodar dentro do painel, já na tela `foco` quando ela existir.
    fn embutida(
        ambiente: Self::Ambiente,
        foco: Option<&str>,
        opts: Embutida,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self;

    /// Miolo da tela `tela`, sem trilho, barra de topo nem barra de status. Id desconhecido desenha a primeira tela.
    fn render_tela(
        &mut self,
        tela: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement;

    /// Roda o comando `comando` escolhido na paleta. Id desconhecido não faz nada.
    fn executar(&mut self, comando: &str, window: &mut Window, cx: &mut Context<Self>);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TELAS: &[TelaInfo] = &[
        TelaInfo {
            id: "fila",
            rotulo: "Fila",
            icone: IconName::ListOrdered,
        },
        TelaInfo {
            id: "historico",
            rotulo: "Histórico",
            icone: IconName::History,
        },
    ];

    const COMANDOS: &[Comando] = &[Comando {
        id: "recarregar",
        grupo: "Fila",
        rotulo: "Recarregar a fila",
        descricao: None,
        icone: IconName::RefreshCw,
        atalho: Some("Ctrl R"),
    }];

    const DEMO: FerramentaInfo = FerramentaInfo {
        id: "demo",
        titulo: "Demo",
        icone: IconName::Package,
        telas: TELAS,
        comandos: COMANDOS,
    };

    #[test]
    fn acha_tela_e_comando_pelo_id() {
        assert_eq!(DEMO.tela("historico").map(|t| t.rotulo), Some("Histórico"));
        assert_eq!(
            DEMO.comando("recarregar").and_then(|c| c.atalho),
            Some("Ctrl R")
        );
        assert!(DEMO.tela("nada").is_none());
        assert!(DEMO.comando("fila").is_none());
    }

    #[test]
    fn contrato_coerente_nao_tem_id_repetido() {
        assert!(DEMO.ids_repetidos().is_empty());
    }

    #[test]
    fn id_repetido_aparece_uma_vez() {
        const REPETIDAS: &[TelaInfo] = &[TELAS[0], TELAS[0], TELAS[0]];
        let info = FerramentaInfo {
            telas: REPETIDAS,
            ..DEMO
        };
        assert_eq!(info.ids_repetidos(), vec!["fila"]);
    }

    #[test]
    fn integrado_comeca_desligado() {
        assert!(!Embutida::default().integrado);
    }
}
