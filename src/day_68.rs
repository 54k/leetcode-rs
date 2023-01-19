// https://leetcode.com/problems/subarray-sums-divisible-by-k/solutions/2913063/subarray-sums-divisible-by-k/?orderBy=most_relevant
pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::*;
    let mut ans = 0;
    let mut prefix = 0;
    let mut map = HashMap::new();
    map.entry(0).or_insert(1);
    for num in nums {
        prefix = (prefix + num % k + k) % k;
        let prefix_mod = map.entry(prefix).or_insert(0);
        ans += *prefix_mod;
        *prefix_mod += 1;
    }
    ans
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test161() {
        println!("{}", subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5));
    }
}
