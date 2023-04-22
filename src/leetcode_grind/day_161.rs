// https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/description/
pub fn min_insertions(s: String) -> i32 {
    fn lcs(s1: &Vec<char>, s2: &Vec<char>) -> i32 {
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
        for i in 1..=s1.len() {
            for j in 1..=s2.len() {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
                if s1[i - 1] == s2[j - 1] {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j - 1] + 1);
                }
            }
        }
        dp[s1.len()][s2.len()]
    }
    let s1 = s.chars().collect::<Vec<_>>();
    let mut s2 = s1.clone();
    s2.reverse();
    s1.len() as i32 - lcs(&s1, &s2)
}

// https://leetcode.com/problems/maximum-length-of-pair-chain/description/
pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
    fn dp(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_by_key(|x| x[0]);
        let mut dp = vec![1; pairs.len()];
        let mut ans = 0;
        for i in 0..pairs.len() {
            for j in 0..i {
                if pairs[j][1] < pairs[i][0] {
                    dp[i] = dp[i].max(dp[j] + 1);
                    ans = ans.max(dp[j] + 1);
                }
            }
        }
        ans
    }
    fn greedy(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_by_key(|x| x[1]);
        let mut ans = 0;
        let mut cur = i32::MIN;
        for i in 0..pairs.len() {
            if cur < pairs[i][0] {
                cur = pairs[i][1];
                ans += 1;
            }
        }
        ans
    }
    greedy(pairs)
}

// https://leetcode.com/problems/solving-questions-with-brainpower/description/
pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test454() {
        println!(
            "{}",
            find_longest_chain(vec![vec![1, 2], vec![7, 8], vec![4, 5]])
        ); // 3
    }

    #[test]
    fn test_455() {
        println!(
            "{}",
            most_points(vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]])
        ); // 5
    }
}
