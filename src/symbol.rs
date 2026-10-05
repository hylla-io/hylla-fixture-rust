// Best practice: one function that branches on `cfg!(feature = "cents")`, so every build has the same item.
#[cfg(feature = "cents")]
pub fn currency_symbol() -> &'static str {
    "¢"
}

#[cfg(not(feature = "cents"))]
pub fn currency_symbol() -> &'static str {
    "$"
}
