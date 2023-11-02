fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    for i in (0.. digits.len()).rev() {
        if digits[i] < 9 {
            digits[i] = digits[i] + 1;
            break;
        }else{
            digits[i] = 0;
        }
    }

    if digits[0] == 0 {
        let mut new_digits : Vec<i32> = vec![1];
        new_digits.extend_from_slice(&digits);
        digits = new_digits
    }

    digits
}

fn plus_one_1(mut digits: Vec<i32>) -> Vec<i32> {
    for digit in digits.iter_mut().rev() {
        if *digit < 9 {
            *digit += 1;
            break;
        }else{
            *digit = 0;
        }
    }

    if digits[0] == 0 {
        let mut new_digits : Vec<i32> = vec![1];
        new_digits.extend_from_slice(&digits);
        digits = new_digits
    }

    digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        assert_eq!(plus_one(vec![0]), vec![1]);
        assert_eq!(plus_one(vec![1,2,3]), vec![1,2,4]);
        assert_eq!(plus_one(vec![4,3,2,1]), vec![4,3,2,2]);
        assert_eq!(plus_one(vec![4,3,2,9]), vec![4,3,3,0]);
        assert_eq!(plus_one(vec![4,3,9,9]), vec![4,4,0,0]);
        assert_eq!(plus_one(vec![4,9,9,9]), vec![5,0,0,0]);
        assert_eq!(plus_one(vec![9,9,9,9]), vec![1,0,0,0,0]);
    }

    #[test]
    fn test_plus_one_1() {
        assert_eq!(plus_one_1(vec![0]), vec![1]);
        assert_eq!(plus_one_1(vec![1,2,3]), vec![1,2,4]);
        assert_eq!(plus_one_1(vec![4,3,2,1]), vec![4,3,2,2]);
        assert_eq!(plus_one_1(vec![4,3,2,9]), vec![4,3,3,0]);
        assert_eq!(plus_one_1(vec![4,3,9,9]), vec![4,4,0,0]);
        assert_eq!(plus_one_1(vec![4,9,9,9]), vec![5,0,0,0]);
        assert_eq!(plus_one_1(vec![9,9,9,9]), vec![1,0,0,0,0]);
    }
}