use std::collections::HashMap;

fn two_sum(nums : &[i32], target: i32) -> (i32, i32) {
    let mut map = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let want = target - num;
        let value = map.get(&want);

        match value {
            Some(found_number) => return (*found_number as i32, i as i32),
            None => { map.insert(num, i); }
        }
    }

    (0, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_two_sum () {
        assert_eq!(two_sum(&[2, 7, 11, 15], 9), (0, 1));
        assert_eq!(two_sum(&[3, 2, 4], 6), (1, 2));
        assert_eq!(two_sum(&[3, 3], 6), (0, 1));
        assert_eq!(two_sum(&[3, 6, -1], 2), (0, 2));
        assert_eq!(two_sum(&[1, 5, 0, 0], 0), (2, 3));
    }
}