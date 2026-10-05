use crate::odd::is_odd;

pub fn is_even(n: usize) -> bool {
    if n == 0 { true } else { is_odd(n - 1) }
}
