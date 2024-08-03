// https://leetcode.com/problems/make-two-arrays-equal-by-reversing-subarrays/description/?envType=daily-question&envId=2024-08-03
pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for t in target {
        *map.entry(t).or_insert(0) += 1;
    }
    for a in arr {
        *map.entry(a).or_insert(1) -= 1;
        if map[&a] == 0 {
            map.remove(&a);
        }
    }
    map.len() == 0
}

// https://leetcode.com/problems/longest-repeating-substring/description/?envType=weekly-question&envId=2024-08-01
pub fn longest_repeating_substring_i(s: String) -> i32 {
    let length = s.len();
    let mut suffixes = vec![vec![]; length];

    for i in 0..length {
        suffixes[i] = s[i..].to_string().chars().collect::<Vec<_>>();
    }

    suffixes.sort();

    let mut max_length = 0;
    for i in 1..length {
        let mut j = 0;
        while j < suffixes[i].len().min(suffixes[i - 1].len())
            && suffixes[i][j] == suffixes[i - 1][j]
        {
            j += 1;
        }
        max_length = max_length.max(j);
    }
    max_length as i32
}

pub fn longest_repeating_substring_ii(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let length = s.len();
    let mut dp = vec![vec![0; length + 1]; length + 1];
    let mut max_len = 0;

    for i in 1..=length {
        for j in i + 1..=length {
            if s[i - 1] == s[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                max_len = max_len.max(dp[i][j] as usize);
            }
        }
    }

    max_len as i32
}
