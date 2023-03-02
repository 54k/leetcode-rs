// https://leetcode.com/problems/string-compression/description/
// https://leetcode.com/problems/string-compression/editorial/
pub fn compress(chars: &mut Vec<char>) -> i32 {
    let mut res = 0;
    let mut i = 0;
    while i < chars.len() {
        let mut group_length = 1;
        while i + group_length < chars.len() && chars[i + group_length] == chars[i] {
            group_length += 1;
        }
        chars[res] = chars[i];
        res += 1;
        if group_length > 1 {
            for ch in group_length.to_string().chars() {
                chars[res] = ch;
                res += 1;
            }
        }
        i += group_length;
    }
    res as i32
}

// https://leetcode.com/problems/number-of-longest-increasing-subsequence/
pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
    let mut dp = vec![(1, 1); nums.len()]; // max_seq_length, num_of_occurrences
    let mut max = 1;
    let mut ans = 0;

    for right in 0..nums.len() {
        for left in 0..right {
            if nums[right] > nums[left] {
                if dp[right].0 < dp[left].0 + 1 {
                    dp[right].0 = dp[left].0 + 1;
                    dp[right].1 = dp[left].1;
                    max = max.max(dp[right].0);
                } else if dp[right].0 == dp[left].0 + 1 {
                    dp[right].1 += dp[left].1;
                }
            }
        }
    }
    for i in 0..dp.len() {
        if dp[i].0 == max {
            ans += dp[i].1;
        }
    }
    ans
}

// https://leetcode.com/problems/coin-change/description/
// https://leetcode.com/problems/coin-change/editorial/
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    fn brute_force(coins: Vec<i32>, amount: i32) -> i32 {
        fn rec(coins: &Vec<i32>, amount: i32, coin_idx: usize) -> i32 {
            if amount == 0 {
                return 0;
            }
            if coin_idx < coins.len() && amount > 0 {
                let max_val = amount / coins[coin_idx];
                let mut min_cost = i32::MAX;
                for coins_count in 0..=max_val {
                    if amount >= coins_count * coins[coin_idx] {
                        let res = rec(coins, amount - coins_count * coins[coin_idx], coin_idx + 1);
                        if res > -1 {
                            min_cost = min_cost.min(res + coins_count);
                        }
                    }
                }
                return if min_cost == i32::MAX { -1 } else { min_cost };
            }
            -1
        }
        rec(&coins, amount, 0)
    }
    fn dp_top_down(coins: Vec<i32>, amount: i32) -> i32 {
        fn rec(coins: &Vec<i32>, rem: i32, coins_count: &mut Vec<i32>) -> i32 {
            if rem < 0 {
                return -1;
            }
            if rem == 0 {
                return 0;
            }
            if coins_count[rem as usize - 1] != 0 {
                return coins_count[rem as usize - 1];
            }
            let mut min = i32::MAX;
            for i in 0..coins.len() {
                let res = rec(coins, rem - coins[i], coins_count);
                if res >= 0 && res < min {
                    min = 1 + res;
                }
            }
            coins_count[rem as usize - 1] = if min == i32::MAX { -1 } else { min };
            coins_count[rem as usize - 1]
        }
        if amount < 1 {
            return 0;
        }
        let mut cache = vec![0; amount as usize];
        rec(&coins, amount, &mut cache)
    }
    fn dp_bottom_up(coins: Vec<i32>, amount: i32) -> i32 {
        let mut max = amount + 1;
        let mut dp = vec![max; amount as usize + 1];
        dp[0] = 0;

        for i in 1..=amount as usize {
            for j in 0..coins.len() {
                if coins[j] <= i as i32 {
                    dp[i] = dp[i].min(dp[i - coins[j] as usize] + 1);
                }
            }
        }
        if dp[amount as usize] > amount {
            -1
        } else {
            dp[amount as usize]
        }
    }
    dp_bottom_up(coins, amount)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test313() {
        let mut chars = vec!['a', 'a', 'a', 'b', 'b', 'a', 'a'];
        println!("{}", compress(&mut chars));
        println!("{:?}", chars);

        let mut chars = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        println!("{}", compress(&mut chars));
        println!("{:?}", chars);

        let mut chars = vec!['a'];
        println!("{}", compress(&mut chars));
        println!("{:?}", chars);

        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        println!("{}", compress(&mut chars));
        println!("{:?}", chars);
    }

    #[test]
    fn test314() {
        println!("{}", find_number_of_lis(vec![1, 3, 2])); // 2
        println!("{}", find_number_of_lis(vec![1, 2, 4, 3, 5, 4, 7, 2])); // 3
        println!("{}", find_number_of_lis(vec![1, 3, 5, 4, 7])); // 2
        println!("{}", find_number_of_lis(vec![2, 2, 2, 2, 2])); // 5
        println!("{}", find_number_of_lis(vec![1, 1, 1, 2, 2, 2, 3, 3, 3])); // 27
    }

    #[test]
    fn test315() {}

    #[test]
    fn test316() {}

    #[test]
    fn test317() {}
}
