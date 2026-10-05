use crate::money::round_half_up;

/// Adds tax at `rate_percent` to an amount in cents, rounding half up.
/// A negative amount (a refund) stays negative.
pub fn apply_tax(cents: i64, rate_percent: i64) -> i64 {
    round_half_up(cents as f64 * (100 + rate_percent) as f64 / 100.0)
}
