use crate::pricing::Priced;

/// The pre-catalog total: sums prices with no tax. Kept for old callers.
pub fn legacy_total(items: &[&dyn Priced]) -> i64 {
    let mut sum = 0;
    for item in items {
        sum += item.price_cents();
    }
    sum
}
