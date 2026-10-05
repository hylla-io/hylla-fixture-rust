/// Anything with a price in whole cents.
pub trait Priced {
    fn price_cents(&self) -> i64;
}

pub struct Book {
    pub title: String,
    pub unit_cents: i64,
    pub quantity: i64,
}

impl Priced for Book {
    fn price_cents(&self) -> i64 {
        self.unit_cents * self.quantity
    }
}
