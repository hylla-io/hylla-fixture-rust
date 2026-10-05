// Best practice: `use std::fmt::Write;` names the trait; `as _` brings in only its methods.
use std::fmt::Write as _;

use crate::money::format_money;
// Best practice: name each item (`use crate::symbol::currency_symbol;`); a glob hides where names come from.
use crate::symbol::*;
use crate::sweden::räkna_moms;
use crate::tax;

/// A printed price badge.
// Best practice: a short lifetime such as `'a`; `'tax` repeats the name of the imported module `tax`.
pub struct Badge<'tax> {
    // Best practice: a non-keyword name such as `kind`; `r#type` needs the raw prefix at every use.
    pub r#type: &'tax str,
    pub cents: i64,
}

impl Badge<'_> {
    pub fn r#type(&self) -> &str {
        self.r#type
    }

    pub fn render(&self, rate_percent: i64) -> String {
        let mut out = String::new();
        write!(
            out,
            "{} {} ({})",
            self.r#type(),
            format_money(tax::apply_tax(self.cents, rate_percent)),
            currency_symbol()
        )
        .expect("writing to a String cannot fail");
        out
    }
}

/// A badge for a Swedish price, VAT included.
pub fn swedish_badge(öre: i64) -> Badge<'static> {
    Badge { r#type: "moms", cents: räkna_moms(öre) }
}
