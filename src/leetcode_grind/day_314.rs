// https://leetcode.com/problems/is-subsequence/editorial
pub fn is_subsequence_two_pointers(s: String, t: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();
    let mut left = 0;
    let mut right = 0;
    while left < s.len() && right < t.len() {
        if s[left] == t[right] {
            left += 1;
        }
        right += 1;
    }
    left == s.len()
}

pub fn is_subsequence_dp_lcs_i(s: String, t: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();

    let (m, n) = (s.len(), t.len());
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for row in 1..=m {
        for col in 1..=n {
            if s[row - 1] == t[col - 1] {
                dp[row][col] = dp[row - 1][col - 1] + 1;
            } else {
                dp[row][col] = dp[row][col - 1].max(dp[row - 1][col]);
            }
        }
    }

    for &val in &dp[m] {
        if val == s.len() {
            return true;
        }
    }

    false
}

pub fn is_subsequence_dp_lcs_ii(s: String, t: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let t = t.chars().collect::<Vec<_>>();

    let (m, n) = (s.len(), t.len());

    if m == 0 {
        return true;
    }

    let mut dp = vec![vec![0; n + 1]; m + 1];
    for col in 1..=n {
        for row in 1..=m {
            if s[row - 1] == t[col - 1] {
                dp[row][col] = dp[row - 1][col - 1] + 1;
            } else {
                dp[row][col] = dp[row - 1][col].max(dp[row][col - 1]);
            }
        }
        if dp[m][col] == m {
            return true;
        }
    }
    false
}

// https://leetcode.com/problems/shortest-way-to-form-string/description/
pub fn shortest_way(source: String, target: String) -> i32 {
    fn is_subsequece(s: &Vec<char>, t: &Vec<char>) -> bool {
        let mut i = 0;
        for &ch in t {
            if ch == s[i] {
                i += 1;
            }
            if i == s.len() {
                return true;
            }
        }
        return false;
    }

    let (s, t) = (
        source.chars().collect::<Vec<_>>(),
        target.chars().collect::<Vec<_>>(),
    );

    let mut left = 0;
    let mut right = target.len() as i32 + 1;

    while left < right {
        let mid = (left + right) / 2;
        if is_subsequece(&t, &s.repeat(mid as usize)) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    if left > t.len() as i32 {
        return -1;
    }
    left
}
