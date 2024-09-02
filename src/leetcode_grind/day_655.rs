// https://leetcode.com/problems/find-the-student-that-will-replace-the-chalk/description/?envType=daily-question&envId=2024-09-02
pub fn chalk_replacer(chalk: Vec<i32>, mut k: i32) -> i32 {
    let n = chalk.len();
    let mut sum = 0;
    for i in 0..n {
        sum += chalk[i] as i64;
        if sum > k as i64 {
            break;
        }
    }
    k = k % (sum as i32);
    for i in 0..n {
        if k < chalk[i] {
            return i as i32;
        }
        k -= chalk[i];
    }
    0
}

// https://leetcode.com/problems/longest-happy-prefix/description/
pub fn longest_prefix(s: String) -> String {
    let mut maxi = 0;
    let s = s.chars().collect::<Vec<_>>();
    let mut p = vec![0; s.len()];
    for i in 1..s.len() {
        let mut j = p[i - 1];
        while j > 0 && s[i] != s[j] {
            j = p[j - 1];
        }
        if s[i] == s[j] {
            j += 1;
        }
        p[i] = j;
        if p[maxi] < p[i] {
            maxi = i;
        }
    }

    s[..p[s.len() - 1]].iter().collect()
}
