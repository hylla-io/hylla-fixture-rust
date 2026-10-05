pub fn round_half_up(x: f64) -> i64 {
    std::cmp::max(0, (x + 0.5).floor() as i64)
}

pub fn format_money(cents: i64) -> String {
    let sign = if cents < 0 { "-" } else { "" };
    let abs = cents.abs();
    let mut dollars = itoa::Buffer::new();
    format!("{sign}${}.{:02}", dollars.format(abs / 100), abs % 100)
}
