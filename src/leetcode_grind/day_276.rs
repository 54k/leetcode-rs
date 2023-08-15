// https://leetcode.com/problems/longest-palindromic-substring/description/
pub fn longest_palindrome(s: String) -> String {
    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();
    let mut dp = vec![vec![false; n]; n];
    let mut ans = vec![0, 0];

    for i in 0..n {
        dp[i][i] = true;
    }

    for i in 0..n - 1 {
        if s[i] == s[i + 1] {
            dp[i][i + 1] = true;
            ans[0] = i;
            ans[1] = i + 1;
        }
    }

    for diff in 2..n {
        for i in 0..n - diff {
            let j = i + diff;
            if s[i] == s[j] && dp[i + 1][j - 1] {
                dp[i][j] = true;
                ans[0] = i;
                ans[1] = j;
            }
        }
    }

    let i = ans[0];
    let j = ans[1];
    s[i..=j].into_iter().collect()
}

// https://leetcode.com/problems/find-all-anagrams-in-a-string/
fn find_anagrams(s: String, p: String) -> Vec<i32> {
    use std::collections::HashMap;

    let mut ans = vec![];

    let s = s.chars().collect::<Vec<_>>();
    let p = p.chars().collect::<Vec<_>>();

    let mut pat = HashMap::new();

    for &ch in &p {
        *pat.entry(ch).or_insert(0) += 1;
    }

    let desired = pat.keys().len() as i32;
    for &ch in &s {
        pat.entry(ch).or_insert(0);
    }

    let mut curr_desired = 0;
    let mut curr_pat = HashMap::new();
    for &ch in &s {
        curr_pat.entry(ch).or_insert(0);
    }

    let mut left = 0;
    for right in 0..s.len() {
        let curr_ch = s[right];

        let was_eq = pat[&curr_ch] == curr_pat[&curr_ch];
        *curr_pat.entry(curr_ch).or_insert(0) += 1;
        let now_eq = pat[&curr_ch] == curr_pat[&curr_ch];

        if was_eq && !now_eq {
            curr_desired -= 1;
        } else if !was_eq && now_eq {
            curr_desired += 1;
        }

        while right - left + 1 > p.len() {
            let curr_ch = s[left];

            let was_eq = pat[&curr_ch] == curr_pat[&curr_ch];
            *curr_pat.entry(curr_ch).or_insert(0) -= 1;
            let now_eq = pat[&curr_ch] == curr_pat[&curr_ch];

            if was_eq && !now_eq {
                curr_desired -= 1;
            } else if !was_eq && now_eq {
                curr_desired += 1;
            }

            left += 1;
        }

        if desired == curr_desired {
            ans.push(left as i32);
        }
    }

    ans
}

#[test]
fn test_find_substr() {
    println!(
        "{:?}",
        find_anagrams("reebok".to_string(), "bee".to_string())
    );
    println!(
        "{:?}",
        find_anagrams("reebok".to_string(), "foo".to_string())
    );
    println!(
        "{:?}",
        find_anagrams("reebok".to_string(), "ebo".to_string())
    );
}

// https://leetcode.com/problems/palindromic-substrings/description
pub fn count_substrings(s: String) -> i32 {
    let n = s.len();
    let s = s.chars().collect::<Vec<_>>();
    let mut dp = vec![vec![false; n]; n];

    let mut ans = 0;
    for i in 0..n {
        dp[i][i] = true;
        ans += 1;
    }

    for i in 0..n - 1 {
        if s[i] == s[i + 1] {
            dp[i][i + 1] = true;
            ans += 1;
        }
    }

    for diff in 2..n {
        for i in 0..n - diff {
            let j = i + diff;
            if s[i] == s[j] && dp[i + 1][j - 1] {
                dp[i][j] = true;
                ans += 1;
            }
        }
    }
    ans
}
