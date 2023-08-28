fn is_armstrong(n : u32) -> bool {
    let str = n.to_string();
    let power = u32::try_from(str.len()).unwrap();
    let mut sum = 0;

    for c in str.chars() {
        sum += u32::pow(c.to_digit(10).unwrap(), power);
    }

    sum == n
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_armstrong() {
        assert_eq!(is_armstrong(153), true);
        assert_eq!(is_armstrong(123), false);
        assert_eq!(is_armstrong(9234567), false);
    }
}