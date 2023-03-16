#[allow(dead_code)]
pub fn minimum_average_difference(nums: Vec<i32>) -> i32 {
    let total: i64 = nums.iter().map(|i| *i as i64).sum();
    let mut ans = -1;
    let mut avg = i64::MAX;
    let mut left_prefix = 0;
    let len = nums.len();
    for i in 0..len {
        left_prefix += nums[i] as i64;
        let right_prefix = total - left_prefix;

        let left_avg = left_prefix / (i as i64 + 1);
        let mut right_avg = right_prefix;
        if i as i64 != len as i64 - 1 {
            right_avg /= len as i64 - i as i64 - 1;
        }

        let diff: i64 = (left_avg - right_avg).abs();
        if diff < avg {
            avg = diff;
            ans = i as i32;
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test95() {
        println!("{}", minimum_average_difference(vec![2, 5, 3, 9, 5, 3])); // 3
    }
}
