// https://leetcode.com/problems/minimum-path-sum/
pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let mut dp = vec![vec![0; grid[0].len() + 1]; grid.len() + 1];
    for i in 0..grid.len() + 1 {
        dp[i][0] = i32::MAX;
    }
    for i in 0..grid[0].len() + 1 {
        dp[0][i] = i32::MAX;
    }
    dp[0][1] = 0;
    for i in 1..=grid.len() {
        for j in 1..=grid[0].len() {
            dp[i][j] = grid[i - 1][j - 1] + dp[i - 1][j].min(dp[i][j - 1]);
        }
    }
    dp[grid.len()][grid[0].len()]
}

// https://leetcode.com/problems/decode-ways-ii/description/
// https://leetcode.com/problems/decode-ways-ii/editorial/
pub fn num_decodings(s: String) -> i32 {
    const MOD: i64 = 1000000007;
    fn using_top_down_approach(s: String) -> i32 {
        fn ways(s: &Vec<char>, i: i32, memo: &mut [i64]) -> i64 {
            if i < 0 {
                return 1;
            }
            let i = i as usize;
            if memo[i] > -1 {
                return memo[i];
            }

            if s[i] == '*' {
                let mut res = 9 * ways(s, i as i32 - 1, memo) % MOD;
                if i > 0 {
                    if s[i - 1] == '*' {
                        res = (res + 15 * ways(s, i as i32 - 2, memo)) % MOD;
                    } else if s[i - 1] == '1' {
                        res = (res + 9 * ways(s, i as i32 - 2, memo)) % MOD;
                    } else if s[i - 1] == '2' {
                        res = (res + 6 * ways(s, i as i32 - 2, memo)) % MOD;
                    }
                }
                memo[i] = res;
                return memo[i];
            }

            let mut res = if s[i] != '0' {
                ways(s, i as i32 - 1, memo) % MOD
            } else {
                0
            };

            if i > 0 {
                if s[i - 1] == '1' || (s[i - 1] == '2' && s[i] <= '6') {
                    res = (res + ways(s, i as i32 - 2, memo)) % MOD;
                } else if s[i - 1] == '*' {
                    let mul = if s[i] <= '6' { 2 } else { 1 };
                    res = (res + mul * ways(s, i as i32 - 2, memo)) % MOD;
                }
            }

            memo[i] = res;
            memo[i]
        }
        let n = s.len();
        let mut memo = vec![-1; n];
        ways(&s.chars().collect(), n as i32 - 1, &mut memo) as i32
    }
    fn using_dynamic_programming_approach(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut dp = vec![0i64; s.len() + 1];
        dp[0] = 1;
        dp[1] = if s[0] == '*' {
            9
        } else if s[0] == '0' {
            0
        } else {
            1
        };
        for i in 1..s.len() {
            if s[i] == '*' {
                dp[i + 1] = 9 * dp[i] % MOD;
                if s[i - 1] == '*' {
                    dp[i + 1] = (dp[i + 1] + 15 * dp[i - 1]) % MOD;
                } else if s[i - 1] == '1' {
                    dp[i + 1] = (dp[i + 1] + 9 * dp[i - 1]) % MOD;
                } else if s[i - 1] == '2' {
                    dp[i + 1] = (dp[i + 1] + 6 * dp[i - 1]) % MOD;
                }
            } else {
                dp[i + 1] = if s[i] == '0' { 0 } else { dp[i] };
                if s[i - 1] == '*' {
                    let mul = if s[i] <= '6' { 2 } else { 1 };
                    dp[i + 1] = (dp[i + 1] + mul * dp[i - 1]) % MOD;
                } else if s[i - 1] == '1' || (s[i - 1] == '2' && s[i] <= '6') {
                    dp[i + 1] = (dp[i + 1] + dp[i - 1]) % MOD;
                }
            }
        }
        dp[s.len()] as i32
    }
    fn using_dynamic_programming_constant_space_approach(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut first = 1;
        let mut second = if s[0] == '*' {
            9
        } else if s[0] == '0' {
            0
        } else {
            1
        };
        for i in 1..s.len() {
            let temp = second;
            if s[i] == '*' {
                second = 9 * second % MOD;
                if s[i - 1] == '1' {
                    second = (second + 9 * first) % MOD;
                } else if s[i - 1] == '2' {
                    second = (second + 6 * first) % MOD;
                } else if s[i - 1] == '*' {
                    second = (second + 15 * first) % MOD;
                }
            } else {
                second = if s[i] == '0' { 0 } else { second };
                if s[i - 1] == '1' || (s[i - 1] == '2' && s[i] <= '6') {
                    second = (second + first) % MOD;
                } else if s[i - 1] == '*' {
                    second = (second + if s[i] <= '6' { 2 } else { 1 } * first) % MOD;
                }
            }
            first = temp;
        }
        second as i32
    }
    using_dynamic_programming_constant_space_approach(s)
}

// https://leetcode.com/problems/take-k-of-each-character-from-left-and-right/
// https://leetcode.com/problems/take-k-of-each-character-from-left-and-right/solutions/2951643/sliding-exclusion-window/
pub fn take_characters(s: String, k: i32) -> i32 {
    let mut freq = [0; 3];
    for ch in s.chars() {
        freq[ch as usize - 'a' as usize] += 1;
    }
    if freq.iter().copied().min().unwrap() < k {
        return -1;
    }
    let mut ans = -1;
    let s = s.chars().collect::<Vec<_>>();
    let mut j = 0;
    for i in 0..s.len() {
        freq[s[i] as usize - 'a' as usize] -= 1;
        if freq[s[i] as usize - 'a' as usize] < k {
            while freq[s[i] as usize - 'a' as usize] < k {
                freq[s[j] as usize - 'a' as usize] += 1;
                j += 1;
            }
        }
        ans = ans.max(i as i32 - j as i32 + 1);
    }
    s.len() as i32 - ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test378() {
        println!(
            "{}",
            min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]])
        ); // 7
        println!("{}", min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]])); // 12
    }

    #[test]
    fn test379() {
        println!("{}", num_decodings("1*".to_string())); // 18
        println!("{}", num_decodings("2*".to_string())); // 15
        println!(
            "{}",
            num_decodings("7*9*3*6*3*0*5*4*9*7*3*7*1*8*3*2*0*0*6*".to_string())
        ); // 196465252
    }

    #[test]
    fn test380() {
        println!("{}", take_characters("abc".to_string(), 1)); // 3
        println!("{}", take_characters("aabaaaacaabc".to_string(), 2)); // 8
        println!("{}", take_characters("a".to_string(), 1)); // -1
    }
}
