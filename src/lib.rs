//! A tiny catalog used as a Hylla ingest fixture.

mod catalog;
mod discount;
mod even;
mod money;
mod odd;
mod pricing;
mod report;
mod tax;

pub use catalog::Catalog;
pub use discount::apply_discount;
pub use money::format_money;
pub use pricing::{Book, Priced};
pub use report::render_report;
pub use tax::apply_tax;
