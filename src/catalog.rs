use crate::discount::apply_discount;
use crate::pricing::Priced;
use crate::tax::apply_tax;

#[derive(Default)]
pub struct Catalog {
    items: Vec<Box<dyn Priced>>,
}

impl Catalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, item: Box<dyn Priced>) {
        self.items.push(item);
    }

    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn subtotal(&self) -> i64 {
        self.items.iter().map(|item| item.price_cents()).sum()
    }

    pub fn total(&self, rate_percent: i64) -> i64 {
        let bulk = if self.count() >= 3 { 10 } else { 0 };
        apply_tax(apply_discount(self.subtotal(), bulk), rate_percent)
    }
}
