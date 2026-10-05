use crate::catalog::Catalog;
use crate::even::is_even;
use crate::money::format_cents;

pub fn render_report(catalog: &Catalog, rate_percent: i64) -> String {
    let parity = if is_even(catalog.count()) {
        "even"
    } else {
        "odd"
    };
    [
        format!("items: {} ({parity})", catalog.count()),
        format!("subtotal: {}", format_cents(catalog.subtotal())),
        format!("total: {}", format_cents(catalog.total(rate_percent))),
    ]
    .join("\n")
}
