#[allow(dead_code)]
pub fn fibonacci_number(n : u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_number(n - 1) + fibonacci_number(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci_number(2), 1);
        assert_eq!(fibonacci_number(3), 2);
        assert_eq!(fibonacci_number(4), 3);
        assert_eq!(fibonacci_number(5), 5);
        assert_eq!(fibonacci_number(6), 8);
        assert_eq!(fibonacci_number(7), 13);
        assert_eq!(fibonacci_number(8), 21);
    }
}