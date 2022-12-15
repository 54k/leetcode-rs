#[allow(dead_code)]
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    fn top_down(text1: String, text2: String) -> i32 {
        fn go(
            text1: &mut Vec<char>,
            text2: &mut Vec<char>,
            i: usize,
            j: usize,
            dp: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if i == text1.len() || j == text2.len() {
                return 0;
            }
            if dp[i][j] != -1 {
                return dp[i][j];
            }

            if text1[i] == text2[j] {
                dp[i][j] = 1 + go(text1, text2, i + 1, j + 1, dp);
            } else {
                dp[i][j] = go(text1, text2, i + 1, j, dp).max(go(text1, text2, i, j + 1, dp));
            }

            dp[i][j]
        }

        go(
            &mut text1.chars().collect(),
            &mut text2.chars().collect(),
            0,
            0,
            &mut vec![vec![-1; text2.len()]; text1.len()],
        )
    }

    fn bottom_up(text1: String, text2: String) -> i32 {
        let mut ans = 0;
        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        for (i, ic) in text1.chars().enumerate() {
            for (j, jc) in text2.chars().enumerate() {
                let i = i + 1;
                let j = j + 1;
                if ic == jc {
                    dp[i][j] = 1 + dp[i - 1][j - 1];
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                }
                ans = ans.max(dp[i][j]);
            }
        }
        ans
    }

    bottom_up(text1, text2)
}

#[allow(dead_code)]
pub fn longest_palindrome_subseq(s: String) -> i32 {
    fn top_down(s: String) -> i32 {
        fn go(s: &Vec<char>, i: usize, j: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
            if i > j {
                return 0;
            }
            if i == j {
                return 1;
            }
            if dp[i][j] != -1 {
                return dp[i][j];
            }
            if s[i] == s[j] {
                dp[i][j] = 2 + go(s, i + 1, j - 1, dp);
            } else {
                dp[i][j] = go(s, i + 1, j, dp).max(go(s, i, j - 1, dp));
            }
            dp[i][j]
        }
        go(
            &s.chars().collect(),
            0,
            s.len() - 1,
            &mut vec![vec![-1; s.len()]; s.len()],
        )
    }

    fn bottom_up(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut dp = vec![vec![0; s.len()]; s.len()];
        for i in 0..dp.len() {
            dp[i][i] = 1;
        }
        for i in (0..s.len()).rev() {
            for j in i + 1..s.len() {
                if s[i] == s[j] {
                    dp[i][j] = 2 + dp[i + 1][j - 1];
                } else {
                    dp[i][j] = dp[i + 1][j].max(dp[i][j - 1]);
                }
            }
        }
        dp[0][s.len() - 1]
    }

    bottom_up(s)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test116() {
        println!(
            "{}",
            longest_common_subsequence("abcde".to_string(), "ace".to_string()) // 3
        );
    }

    #[test]
    fn test117() {
        println!("{}", longest_palindrome_subseq("bbbab".to_string())); // 4
    }
}
