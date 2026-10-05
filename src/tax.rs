pub fn round_half_up(x: f64) -> i64 {
    std::cmp::max(0, (x + 0.5).floor() as i64)
}

/// Adds tax at `rate_percent` to an amount in cents, rounding half up.
/// A negative amount (a refund) becomes zero.
pub fn apply_tax(cents: i64, rate_percent: i64) -> i64 {
    round_half_up(cents as f64 * (100 + rate_percent) as f64 / 100.0)
}
