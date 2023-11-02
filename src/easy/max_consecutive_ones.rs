use std::cmp::max;

fn max_consecutive_ones(mut nums: Vec<i32>) -> i32 {
    let mut result = match nums[0] {
        1 => 1,
        _ => 0
    };

   for i in 1..nums.len() {
       if nums[i] == 1 {
           nums[i] = nums[i-1] + 1;
           result = max(nums[i], result);
       }
   }

   result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(max_consecutive_ones(vec![1,1,1,0,0,0,1,1,1,1,0]), 4);
        assert_eq!(max_consecutive_ones(vec![0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1]), 4);
        assert_eq!(max_consecutive_ones(vec![1]), 1);
        assert_eq!(max_consecutive_ones(vec![0]), 0);
        assert_eq!(max_consecutive_ones(vec![1,0,0,1]), 1);
        assert_eq!(max_consecutive_ones(vec![1,1,1,1]), 4);
        assert_eq!(max_consecutive_ones(vec![0,0,0,0]), 0);
    }
}