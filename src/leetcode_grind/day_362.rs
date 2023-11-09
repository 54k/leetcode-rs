// https://leetcode.com/problems/count-number-of-homogenous-substrings/description
pub fn count_homogenous(s: String) -> i32 {
    let s = s.as_bytes();
    const MOD: i64 = 1_000_000_007;
    let mut count = 0;
    let mut ans = 0;
    for i in 0..s.len() {
        if i == 0 || s[i - 1] == s[i] {
            count += 1;
        } else {
            count = 1
        }
        ans = (ans + count) % MOD
    }
    ans as i32
}

// https://leetcode.com/problems/consecutive-characters/description/
pub fn max_power_1(s: String) -> i32 {
    let s = s.as_bytes();
    let mut max = 0;
    let mut streak = 0;
    for i in 0..s.len() {
        if (i == 0 || s[i - 1] == s[i]) {
            streak += 1;
        } else {
            streak = 1;
        }
        max = max.max(streak);
    }
    max
}

pub fn max_power_2(s: String) -> i32 {
    let mut ans = 0;
    let mut count = 0;
    let mut prev = ' ';
    for ch in s.chars() {
        if ch == prev {
            count += 1;
        } else {
            count = 1;
            prev = ch;
        }
        ans = ans.max(count);
    }
    ans
}

// https://leetcode.com/problems/number-of-substrings-with-only-1s/description/
pub fn num_sub(s: String) -> i32 {
    const MOD: i64 = 1_000_000_007;

    let mut streak = 0;
    let s = s.as_bytes();
    let mut ans = 0;

    for i in 0..s.len() {
        if (s[i] == b'1') {
            streak += 1;
        } else {
            streak = 0;
        }
        ans = (ans + streak) % MOD;
    }
    ans as i32
}

// https://leetcode.com/problems/pascals-triangle/description/
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut rows = vec![vec![1]];
    for i in 1..num_rows as usize {
        let mut new_row = vec![1];
        for j in 1..i {
            new_row.push(rows[i - 1][j - 1] + rows[i - 1][j]);
        }
        new_row.push(1);
        rows.push(new_row);
    }
    rows
}
