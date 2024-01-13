// https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/description
pub fn min_steps_i(s: String, t: String) -> i32 {
    use std::collections::HashMap;
    let mut f1 = HashMap::new();
    let mut f2 = HashMap::new();

    for ch in s.chars() {
        *f1.entry(ch).or_insert(0) += 1;
    }

    for ch in t.chars() {
        *f2.entry(ch).or_insert(0) += 1;
    }

    for (k, v) in f2 {
        if f1.contains_key(&k) {
            *f1.get_mut(&k).unwrap() -= v;
            if f1[&k] <= 0 {
                f1.remove(&k);
            }
        }
    }

    let mut ans = 0;
    for v in f1.values() {
        ans += v;
    }
    ans
}

pub fn min_steps_ii(s: String, t: String) -> i32 {
    let (s, t) = (s.chars().collect::<Vec<_>>(), t.chars().collect::<Vec<_>>());
    let mut count = vec![0; 26];
    for i in 0..s.len() {
        count[t[i] as usize - 'a' as usize] += 1;
        count[s[i] as usize - 'a' as usize] -= 1;
    }
    count.into_iter().filter(|&x| x > 0).sum()
}
