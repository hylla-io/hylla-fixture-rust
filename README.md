# hylla-fixture-rust

A tiny catalog used as a Hylla ingest fixture. Every file is small and its contents are known
exactly.

## Money

`format_money` prints cents as dollars, writing the dollars with `itoa::Buffer`. It lives in
[src/money.rs](src/money.rs).

## Tax

`apply_tax` in [src/tax.rs](src/tax.rs) adds tax and rounds with `round_half_up`, which lives
beside it. Refunds clamp to zero: `apply_tax(-1000, 10)` is `0`.

## Totals

`Catalog::total` takes a bulk discount with `apply_discount` at three or more items, then taxes.
`render_report` prints a report; rounding is described under [Tax](#tax).
