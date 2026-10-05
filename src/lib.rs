//! A tiny catalog used as a Hylla ingest fixture.

mod catalog;
mod even;
mod legacy;
mod money;
mod odd;
mod pricing;
mod report;
mod tax;

pub use catalog::Catalog;
pub use legacy::legacy_total;
pub use money::format_cents;
pub use pricing::{Book, Priced};
pub use report::render_report;
pub use tax::apply_tax;
