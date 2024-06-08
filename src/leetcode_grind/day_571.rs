// https://leetcode.com/problems/substrings-that-begin-and-end-with-the-same-letter/description
pub fn number_of_substrings(s: String) -> i64 {
    use std::collections::HashMap;
    let mut cnt = HashMap::new();
    let mut ans = 0;
    for ch in s.chars() {
        ans += 1;
        if cnt.contains_key(&ch) {
            ans += *cnt.get(&ch).unwrap();
        }
        *cnt.entry(ch).or_insert(0) += 1;
    }
    ans
}

// https://leetcode.com/problems/sum-of-beauty-of-all-substrings/description/
pub fn beauty_sum(s: String) -> i32 {
    use std::collections::BTreeMap;
    let s = s.as_bytes();
    let mut ans = 0;
    for i in 0..s.len() {
        let mut freq = BTreeMap::new();
        for j in i..s.len() {
            *freq.entry(s[j] as usize - 'a' as usize).or_insert(0) += 1;
            let mx = freq.values().max().unwrap();
            let mn = freq.values().min().unwrap();
            ans += mx - mn;
        }
    }
    ans
}
 