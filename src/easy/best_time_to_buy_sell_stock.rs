use std::cmp::{max, min};

fn max_profit(prices : &[i32]) -> i32 {
    let mut max_profit = 0;
    let mut min_price = prices[0];
    for price in prices {
        max_profit = max(max_profit, price - min_price);
        min_price = min(*price, min_price);
    }

    max_profit
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(max_profit(&[7,1,5,3,6,4]), 5);
        assert_eq!(max_profit(&[7,6,4,3,1]), 0);
        assert_eq!(max_profit(&[1]), 0);
        assert_eq!(max_profit(&[1,1]), 0);
        assert_eq!(max_profit(&[1,0,1]), 1);
    }
}