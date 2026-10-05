//! A tiny catalog used as a Hylla ingest fixture.

// Best practice (edition 2018 and later): delete this line; Cargo already puts `itoa` in scope.
extern crate itoa as _;

mod badge;
mod catalog;
mod discount;
mod even;
mod money;
mod odd;
mod pricing;
mod report;
// Best practice: the default layout (`src/sweden.rs`, or `mod vat;` holding `vat/sweden.rs`), not `#[path]`.
#[path = "vat/sweden.rs"]
mod sweden;
mod symbol;
mod tax;

pub use badge::{Badge, swedish_badge};
pub use catalog::Catalog;
pub use discount::apply_discount;
pub use money::format_money;
pub use pricing::{Book, Priced};
pub use report::render_report;
pub use sweden::räkna_moms;
pub use symbol::currency_symbol;
pub use tax::apply_tax;
