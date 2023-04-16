// https://leetcode.com/problems/number-of-ways-to-form-a-target-string-given-a-dictionary/description/
// https://leetcode.com/problems/number-of-ways-to-form-a-target-string-given-a-dictionary/editorial/
pub fn num_ways(words: Vec<String>, target: String) -> i32 {
    fn bottom_up(words: Vec<String>, target: String) -> i32 {
        let words = words
            .into_iter()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let target = target.chars().collect::<Vec<_>>();
        const MOD: i64 = 1000000007;
        let m = target.len();
        let k = words[0].len();
        let mut cnt = vec![vec![0i64; k]; 26];
        for j in 0..k {
            for word in &words {
                cnt[word[j] as usize - 'a' as usize][j] += 1;
            }
        }
        let mut dp = vec![vec![0; k + 1]; m + 1];
        dp[0][0] = 1;
        for i in 0..=m {
            for j in 0..k {
                if i < m {
                    dp[i + 1][j + 1] += cnt[target[i] as usize - 'a' as usize][j] * dp[i][j];
                    dp[i + 1][j + 1] %= MOD;
                }
                dp[i][j + 1] += dp[i][j];
                dp[i][j + 1] %= MOD;
            }
        }
        dp[m][k] as i32
    }
    bottom_up(words, target)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test441() {
        println!(
            "{}",
            num_ways(
                vec!["acca".to_string(), "bbbb".to_string(), "caca".to_string()],
                "aba".to_string()
            )
        ); // 6
    }
}
