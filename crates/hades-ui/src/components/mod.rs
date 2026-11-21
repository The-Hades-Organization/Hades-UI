//! Componentes base, só os que aparecem em ao menos duas das sete páginas de design. Sem lógica de ferramenta.
mod card;
mod data_table;
mod empty_state;
mod kpi_strip;
mod meter;
mod page_header;
mod rail;
mod status_badge;
mod status_bar;
mod term_block;
mod top_bar;

pub use card::Card;
pub use data_table::{Align, DataTable};
pub use empty_state::EmptyState;
pub use kpi_strip::{Kpi, KpiStrip};
pub use meter::Meter;
pub use page_header::PageHeader;
pub use rail::{Rail, RailGroup, RailItem};
pub use status_badge::StatusBadge;
pub use status_bar::StatusBar;
pub use term_block::TermBlock;
pub use top_bar::TopBar;
