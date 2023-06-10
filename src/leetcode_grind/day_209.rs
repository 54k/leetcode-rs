// https://leetcode.com/problems/find-smallest-letter-greater-than-target/description/
pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let mut lo = 0;
    let mut hi = letters.len() as i32 - 1;
    while lo <= hi {
        let mid = (lo + hi) / 2;
        if letters[mid as usize] <= target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    if lo > 0 && lo <= letters.len() as i32 - 1 {
        letters[lo as usize]
    } else {
        letters[0]
    }
}

// https://leetcode.com/problems/interleaving-string/description/
pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    let (s1, s2, s3) = (
        s1.chars().collect::<Vec<_>>(),
        s2.chars().collect::<Vec<_>>(),
        s3.chars().collect::<Vec<_>>(),
    );

    if s3.len() != s1.len() + s2.len() {
        return false;
    }

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
    return dp[s1.len()][s2.len()];
}
