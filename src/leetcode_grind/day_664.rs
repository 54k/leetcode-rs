// https://leetcode.com/problems/minimum-bit-flips-to-convert-number/description/?envType=daily-question&envId=2024-09-11
pub fn min_bit_flips_i(start: i32, goal: i32) -> i32 {
    let mut ans = 0;
    for i in 0..31 {
        if ((start >> i) & 1) != ((goal >> i) & 1) {
            ans += 1;
        }
    }
    ans
}

pub fn min_bit_flips_ii(start: i32, goal: i32) -> i32 {
    let mut xor_result = start ^ goal;
    let mut count = 0;
    while xor_result != 0 {
        count += xor_result & 1;
        xor_result >>= 1;
    }
    count
}

pub fn min_bit_flips_iii(start: i32, goal: i32) -> i32 {
    let mut xor_result = start ^ goal;
    let mut count = 0;
    while xor_result != 0 {
        xor_result = xor_result & (xor_result - 1);
        count += 1;
    }
    count
}

// https://leetcode.com/problems/longest-happy-prefix/description/
pub fn longest_prefix(s: String) -> String {
    let s = s.chars().collect::<Vec<_>>();
    let mut p = vec![0; s.len()];
    for i in 1..s.len() {
        let mut j = p[i - 1];
        while j > 0 && s[j] != s[i] {
            j = p[j - 1];
        }
        if s[j] == s[i] {
            j += 1;
        }
        p[i] = j;
    }
    s[..p[s.len() - 1]].iter().collect()
}

// https://leetcode.com/problems/shortest-common-supersequence/description/
pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
    let str1 = str1.chars().collect::<Vec<_>>();
    let str2 = str2.chars().collect::<Vec<_>>();

    let mut n = str1.len();
    let mut m = str2.len();

    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            if str1[i - 1] == str2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    let mut ans = String::new();
    while n > 0 && m > 0 {
        if str1[n - 1] == str2[m - 1] {
            ans.push(str1[n - 1]);
            n -= 1;
            m -= 1;
        } else if dp[n - 1][m] >= dp[n][m - 1] {
            ans.push(str1[n - 1]);
            n -= 1;
        } else {
            ans.push(str2[m - 1]);
            m -= 1;
        }
    }
    while n > 0 {
        ans.push(str1[n - 1]);
        n -= 1;
    }
    while m > 0 {
        ans.push(str2[m - 1]);
        m -= 1;
    }
    ans.chars().rev().collect()
}
