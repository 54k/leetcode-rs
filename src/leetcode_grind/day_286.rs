// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/description/
pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    for i in 0..nums.len() {
        let mut smaller = 0;
        for j in 0..nums.len() {
            if i == j {
                continue;
            }
            if nums[j] < nums[i] {
                smaller += 1;
            }
        }
        ans.push(smaller);
    }
    ans
}

// https://leetcode.com/problems/interleaving-string/description/
pub fn is_interleave_i(s1: String, s2: String, s3: String) -> bool {
    fn rec(
        s1: &Vec<char>,
        i: usize,
        s2: &Vec<char>,
        j: usize,
        s3: &Vec<char>,
        res: &mut Vec<char>,
    ) -> bool {
        if res == s3 && i == s1.len() && j == s2.len() {
            return true;
        }

        let mut result = false;

        if i < s1.len() {
            res.push(s1[i]);
            result |= rec(s1, i + 1, s2, j, s3, res);
            res.pop();
        }

        if j < s2.len() {
            res.push(s2[j]);
            result |= rec(s1, i, s2, j + 1, s3, res);
            res.pop();
        }

        result
    }

    let s1 = s1.chars().collect::<Vec<_>>();
    let s2 = s2.chars().collect::<Vec<_>>();
    let s3 = s3.chars().collect::<Vec<_>>();

    let mut res = vec![];

    rec(&s1, 0, &s2, 0, &s3, &mut res)
}

pub fn is_interleave_ii(s1: String, s2: String, s3: String) -> bool {
    use std::collections::HashMap;

    fn rec(
        s1: &Vec<char>,
        i: usize,
        s2: &Vec<char>,
        j: usize,
        s3: &Vec<char>,
        k: usize,
        memo: &mut HashMap<(usize, usize), bool>,
    ) -> bool {
        if i == s1.len() {
            return s3[k..] == s2[j..];
        }
        if j == s2.len() {
            return s3[k..] == s1[i..];
        }

        if memo.contains_key(&(i, j)) {
            return memo[&(i, j)];
        }
        let mut ans = false;
        if (s1[i] == s3[k] && rec(s1, i + 1, s2, j, s3, k + 1, memo))
            || (s2[j] == s3[k] && rec(s1, i, s2, j + 1, s3, k + 1, memo))
        {
            ans = true;
        }
        memo.insert((i, j), ans);
        ans
    }

    if s1.len() + s2.len() > s3.len() {
        return false;
    }

    let mut memo = HashMap::new();
    let s1 = s1.chars().collect::<Vec<_>>();
    let s2 = s2.chars().collect::<Vec<_>>();
    let s3 = s3.chars().collect::<Vec<_>>();

    rec(&s1, 0, &s2, 0, &s3, 0, &mut memo)
}

pub fn is_interleave_iii(s1: String, s2: String, s3: String) -> bool {
    if s3.len() != s1.len() + s2.len() {
        return false;
    }

    let s1 = s1.chars().collect::<Vec<_>>();
    let s2 = s2.chars().collect::<Vec<_>>();
    let s3 = s3.chars().collect::<Vec<_>>();

    let mut dp = vec![vec![false; s2.len() + 1]; s1.len() + 1];

    for i in 0..=s1.len() {
        for j in 0..=s2.len() {
            if i == 0 && j == 0 {
                dp[i][j] = true;
            } else if i == 0 {
                dp[i][j] = dp[i][j - 1] && s2[j - 1] == s3[i + j - 1];
            } else if j == 0 {
                dp[i][j] = dp[i - 1][j] && s1[i - 1] == s3[i + j - 1];
            } else {
                dp[i][j] = (dp[i - 1][j] && s1[i - 1] == s3[i + j - 1])
                    || (dp[i][j - 1] && s2[j - 1] == s3[i + j - 1]);
            }
        }
    }

    dp[s1.len()][s2.len()]
}

// https://leetcode.com/problems/longest-common-subsequence/description/
pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    use std::collections::HashMap;

    fn rec(
        memo: &mut HashMap<(usize, usize), i32>,
        text1: &Vec<char>,
        text2: &Vec<char>,
        i: usize,
        j: usize,
    ) -> i32 {
        if i == text1.len() || j == text2.len() {
            return 0;
        }
        if memo.contains_key(&(i, j)) {
            return memo[&(i, j)];
        }

        let ans = if text1[i] == text2[j] {
            1 + rec(memo, text1, text2, i + 1, j + 1)
        } else {
            rec(memo, text1, text2, i + 1, j).max(rec(memo, text1, text2, i, j + 1))
        };

        memo.insert((i, j), ans);
        ans
    }

    let text1 = text1.chars().collect::<Vec<_>>();
    let text2 = text2.chars().collect::<Vec<_>>();
    let mut memo = HashMap::new();
    rec(&mut memo, &text1, &text2, 0, 0)
}
