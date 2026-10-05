use crate::tax::apply_tax;

// Best practice: ASCII identifiers (`compute_vat`, `ore`), as the `non_ascii_idents` lint asks.
pub fn räkna_moms(öre: i64) -> i64 { apply_tax(öre, 25) }
