pub fn format_money(cents: i64) -> String {
    let sign = if cents < 0 { "-" } else { "" };
    let abs = cents.abs();
    let mut dollars = itoa::Buffer::new();
    format!("{sign}${}.{:02}", dollars.format(abs / 100), abs % 100)
}
