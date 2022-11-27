// https://leetcode.com/problems/arithmetic-slices-ii-subsequence/solution/
// class Solution {
//     public int numberOfArithmeticSlices(int[] A) {
//         int n = A.length;
//         long ans = 0;
//         Map<Integer, Integer>[] cnt = new Map[n];
//         for (int i = 0; i < n; i++) {
//             cnt[i] = new HashMap<>(i);
//             for (int j = 0; j < i; j++) {
//                 long delta = (long)A[i] - (long)A[j];
//                 if (delta < Integer.MIN_VALUE || delta > Integer.MAX_VALUE) {
//                     continue;
//                 }
//                 int diff = (int)delta;
//                 int sum = cnt[j].getOrDefault(diff, 0);
//                 int origin = cnt[i].getOrDefault(diff, 0);
//                 cnt[i].put(diff, origin + sum + 1);
//                 ans += sum;
//             }
//         }
//         return (int)ans;
//     }
// }
#[allow(dead_code)]
pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    fn number_of_arithmetic_slices_dp(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut dp = vec![HashMap::new(); nums.len()];
        let mut ans = 0;
        for i in 0..nums.len() {
            for j in 0..i {
                let delta: i64 = nums[i] as i64 - nums[j] as i64;
                if delta < i32::MIN as i64 || delta > i32::MAX as i64 {
                    continue;
                }
                let diff = delta as i32;
                let sum = *dp[j].get(&diff).unwrap_or(&0);
                let origin = *dp[i].get(&diff).unwrap_or(&0);
                dp[i].insert(diff, origin + sum + 1);
                ans += sum;
            }
        }
        ans
    }

    fn number_of_arithmetic_slices_dfs(nums: Vec<i32>) -> i32 {
        fn is_arithmetic(buf: &Vec<i32>) -> bool {
            if buf.len() < 3 {
                return false;
            }
            let diff = buf[0] - buf[1];
            for i in 2..buf.len() {
                if (buf[i - 1] - buf[i]) != diff {
                    return false;
                }
            }
            true
        }

        fn dfs(nums: &Vec<i32>, buf: &mut Vec<i32>, i: usize) -> i32 {
            if i == nums.len() {
                return is_arithmetic(buf) as i32;
            }

            let mut res = 0;
            res += dfs(nums, buf, i + 1);
            buf.push(nums[i]);
            res += dfs(nums, buf, i + 1);
            buf.pop();
            res
        }

        dfs(&nums, &mut vec![], 0)
    }

    number_of_arithmetic_slices_dp(nums)
}

#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    fn max_profit_brute(prices: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..prices.len() - 1 {
            for j in i + 1..prices.len() {
                ans = ans.max(prices[j] - prices[i]);
            }
        }
        ans
    }

    fn max_profit_dp(prices: Vec<i32>) -> i32 {
        let mut least_so_far = i32::MAX;
        let mut ans = 0;
        for p in prices {
            least_so_far = least_so_far.min(p);
            ans = ans.max(p - least_so_far);
        }
        ans
    }

    max_profit_dp(prices)
}

#[allow(dead_code)]
pub fn max_profit2(prices: Vec<i32>) -> i32 {
    // Buy very first stock
    let mut buy = prices[0];
    let mut profit = 0;
    for p in prices.iter().skip(1) {
        // If you find stock less than what you bought, take that instead
        if *p < buy {
            buy = *p;
            // If you find stock greater than what you bought, sell that
            // Also buy it as you want to sell if you see greater stock in future
        } else {
            profit += *p - buy;
            buy = *p;
        }
    }
    profit
}

#[cfg(test)]
mod test {
    use crate::day_15::*;

    #[test]
    fn test80() {
        println!("{}", number_of_arithmetic_slices(vec![2, 4, 6, 8, 10])); //7
    }

    #[test]
    fn test81() {
        println!("{}", max_profit(vec![7, 1, 5, 3, 6, 4])); // 5
    }

    #[test]
    fn test82() {
        println!("{}", max_profit2(vec![7, 1, 5, 3, 6, 4])); // 7
    }
}