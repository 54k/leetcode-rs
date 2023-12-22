// https://leetcode.com/problems/maximum-score-after-splitting-a-string/
pub fn max_score_1(s: String) -> i32 {
    let (mut z, mut o) = (0, 0);
    for ch in s.chars() {
        if ch == '1' {
            o += 1;
        }
    }
    let s = s.chars().collect::<Vec<_>>();
    let mut ans = 0;
    for i in 0..s.len() - 1 {
        let ch = s[i];
        if ch == '1' {
            o -= 1;
        } else if ch == '0' {
            z += 1;
        }
        ans = ans.max(o + z);
    }
    ans
}

pub fn max_score_2(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let mut ones = 0;
    let mut zeros = 0;
    let mut best = i32::MIN;

    for i in 0..s.len() - 1 {
        if s[i] == '1' {
            ones += 1;
        } else {
            zeros += 1;
        }

        best = best.max(zeros - ones);
    }

    if s[s.len() - 1] == '1' {
        ones += 1;
    }

    best + ones
}
