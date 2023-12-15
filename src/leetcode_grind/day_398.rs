// https://leetcode.com/problems/count-unique-characters-of-all-substrings-of-a-given-string/description/
pub fn unique_letter_string(s: String) -> i32 {
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len() as i64;
    const MOD: i64 = 1_000_000_007;
    let mut index = vec![vec![-1; 2]; 26];
    let mut res = 0;

    for i in 0..s.len() {
        let c = s[i] as usize - 'A' as usize;
        res = (res + (i as i64 - index[c][1]) * (index[c][1] - index[c][0]) % MOD) % MOD;
        index[c] = vec![index[c][1], i as i64];
    }

    for c in 0..26 {
        res = (res + (n - index[c][1]) * (index[c][1] - index[c][0]) % MOD) % MOD;
    }

    res as i32
}
