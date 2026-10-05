use crate::even::is_even;

pub fn is_odd(n: usize) -> bool {
    if n == 0 { false } else { is_even(n - 1) }
}
