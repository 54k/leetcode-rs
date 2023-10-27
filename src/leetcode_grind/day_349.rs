// https://leetcode.com/problems/longest-palindromic-substring
pub fn longest_palindrome_dp(s: String) -> String {
    let s = s.chars().collect::<Vec<_>>();

    let mut dp = vec![vec![false; s.len()]; s.len()];

    let (mut lo, mut hi) = (0, 0);
    for i in 0..s.len() {
        dp[i][i] = true;
        if i < s.len() - 1 && s[i] == s[i + 1] {
            dp[i][i + 1] = true;
            lo = i;
            hi = i + 1;
        }
    }

    for diff in 2..s.len() {
        for start in 0..s.len() - diff {
            let end = start + diff;
            if s[start] == s[end] && dp[start + 1][end - 1] {
                dp[start][end] = true;
                if hi - lo < end - start {
                    lo = start;
                    hi = end;
                }
            }
        }
    }

    s[lo..=hi].into_iter().collect()
}

pub fn longest_palindrome_expand_around_center(s: String) -> String {
    fn expand(s: &[char], start: usize, end: usize) -> i32 {
        let mut left = start as i32;
        let mut right = end as i32;

        while left >= 0 && right < s.len() as i32 && s[left as usize] == s[right as usize] {
            left -= 1;
            right += 1;
        }
        right - left - 1
    }

    let s = s.chars().collect::<Vec<_>>();
    let (mut lo, mut hi): (usize, usize) = (0, 0);

    for center in 0..s.len() {
        let odd_len = expand(&s, center, center);
        if odd_len > (hi - lo + 1) as i32 {
            let dist = odd_len as usize / 2;
            lo = center - dist;
            hi = center + dist;
        }

        let even_len = expand(&s, center, center + 1);
        if even_len > (hi - lo + 1) as i32 {
            let dist = (even_len as usize / 2) - 1;
            lo = center - dist;
            hi = center + dist + 1;
        }
    }

    s[lo..=hi].into_iter().collect()
}
