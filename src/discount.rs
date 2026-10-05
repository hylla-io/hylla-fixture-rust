use crate::money::round_half_up;

/// Takes `percent` off an amount in cents, rounding the discount half up.
pub fn apply_discount(cents: i64, percent: i64) -> i64 {
    cents - round_half_up(cents as f64 * percent as f64 / 100.0)
}
