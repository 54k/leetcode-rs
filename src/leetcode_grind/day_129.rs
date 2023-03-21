// https://leetcode.com/problems/number-of-zero-filled-subarrays/description/
// https://leetcode.com/problems/number-of-zero-filled-subarrays/editorial/
pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
    let mut ans = 0;
    let mut current = 0;
    for n in nums {
        if n == 0 {
            current += 1;
        } else {
            current = 0;
        }
        ans += current
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test360() {
        println!("{}", zero_filled_subarray(vec![1, 3, 0, 0, 2, 0, 0, 4])); // 6
        println!("{}", zero_filled_subarray(vec![0, 0, 0, 2, 0, 0])); // 9
    }
}
