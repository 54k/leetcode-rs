// https://leetcode.com/problems/sum-of-total-strength-of-wizards/description/
pub fn total_strength(strength: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/smallest-range-i/description/
pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
    todo!();
}

// https://leetcode.com/problems/edit-distance/
pub fn min_distance(word1: String, word2: String) -> i32 {
    let n = word1.len();
    let m = word2.len();
    let word1 = word1.chars().collect::<Vec<_>>();
    let word2 = word2.chars().collect::<Vec<_>>();
    let mut dp = vec![vec![10000007; m + 1]; n + 1];
    for i in 1..=n {
        dp[i][0] = i as i32;
    }
    for j in 1..=m {
        dp[0][j] = j as i32;
    }
    dp[0][0] = 0;

    for i in 1..=n {
        for j in 1..=m {
            if word1[i - 1] == word2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
            }
        }
    }
    dp[n][m]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test300() {
        println!("{}", total_strength(vec![1, 3, 1, 2])); // 44
        println!("{}", total_strength(vec![5, 4, 6])); // 213
    }

    #[test]
    fn test301() {
        println!("{}", smallest_range_i(vec![1], 0)); // 0
        println!("{}", smallest_range_i(vec![0, 10], 2)); // 6
        println!("{}", smallest_range_i(vec![1, 3, 6], 3)); // 0
    }

    #[test]
    fn test302() {
        println!("{}", min_distance("horse".to_string(), "ros".to_string())); // 3
    }
}
