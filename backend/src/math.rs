pub fn factorial(n: f64) -> f64 {
    if n == 0. {
        1.
    } else {
        n * factorial(n - 1.)
    }
}

pub fn is_positive_integer(num: f64) -> bool {
    num > 0.0 && num.fract() == 0.0
}