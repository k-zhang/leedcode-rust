use std::cmp::min;

fn min_cost_climbing_stairs(cost: &[i32]) -> i32 {
    if cost.len() == 0 || cost.len() == 1 {
        return 0;
    }

    let mut dp = vec![0; cost.len() + 1];
    dp[0] = 0;
    dp[1] = cost[0];

    for i in 2..=cost.len() {
        dp[i] = min(dp[i-1] + cost[i-1], dp[i-2] + cost[i-1])
    }

    min(dp[cost.len()], dp[cost.len() - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(min_cost_climbing_stairs(&[10, 15, 20]), 15);
        assert_eq!(min_cost_climbing_stairs(&[1, 100, 1, 1, 1, 100, 1, 1, 100, 1]), 6);
        assert_eq!(min_cost_climbing_stairs(&[1, 2]), 1);
        assert_eq!(min_cost_climbing_stairs(&[0, 0, 0, 0]), 0);
        assert_eq!(min_cost_climbing_stairs(&[0, 0, 0, 1]), 0);
        assert_eq!(min_cost_climbing_stairs(&[1]), 0);
        assert_eq!(min_cost_climbing_stairs(&[]), 0);
    }
}