// https://leetcode.com/problems/minimize-maximum-of-array/
// https://leetcode.com/problems/minimize-maximum-of-array/editorial/
pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut prefix_sum = 0i64;
    for i in 0..nums.len() {
        prefix_sum += nums[i] as i64;
        ans = ans.max(((prefix_sum + i as i64) / (i as i64 + 1)) as i32);
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test400() {
        println!("{}", minimize_array_value(vec![3, 7, 1, 6])); // 5
    }
}
