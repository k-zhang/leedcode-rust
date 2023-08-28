fn climbing_stairs(n : i32) -> i32 {
    if n < 3 { return n; }

    let mut dp_0 = 1;
    let mut dp_1 = 2;
    let mut dp_2 = 0;

    for _i in 2..n {
        dp_2 = dp_1 + dp_0;
        dp_0 = dp_1;
        dp_1 = dp_2;
    }

    dp_2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_climbing_stairs() {
        assert_eq!(climbing_stairs(1), 1);
        assert_eq!(climbing_stairs(2), 2);
        assert_eq!(climbing_stairs(3), 3);
        assert_eq!(climbing_stairs(4), 5);
    }
}