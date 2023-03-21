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

// https://leetcode.com/problems/split-array-into-consecutive-subsequences/description/
// https://leetcode.com/problems/split-array-into-consecutive-subsequences/solutions/106495/java-o-n-time-o-1-space-solution-greedily-extending-shorter-subsequence/
pub fn is_possible(nums: Vec<i32>) -> bool {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test360() {
        println!("{}", zero_filled_subarray(vec![1, 3, 0, 0, 2, 0, 0, 4])); // 6
        println!("{}", zero_filled_subarray(vec![0, 0, 0, 2, 0, 0])); // 9
    }

    #[test]
    fn test361() {
        println!("{}", is_possible(vec![1, 2, 3, 3, 4, 5])); // true
        println!("{}", is_possible(vec![1, 2, 3, 3, 4, 4, 5, 5])); // true
        println!("{}", is_possible(vec![1, 2, 3, 4, 4, 5])); // false
    }
}
