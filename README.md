# hylla-fixture-rust

A tiny catalog used as a Hylla ingest fixture. Every file is small and its contents are known
exactly.

## Money

`round_half_up` rounds half up. `format_cents` prints cents as dollars, writing the dollars with
`itoa::Buffer`. Both live in [src/money.rs](src/money.rs).

## Tax

`apply_tax` in [src/tax.rs](src/tax.rs) adds tax and rounds half up. Refunds pass through:
`apply_tax(-1000, 10)` is `-1100`.

## Totals

`Catalog::total` taxes the subtotal. `render_report` prints a report; rounding is described under
[Tax](#tax).
