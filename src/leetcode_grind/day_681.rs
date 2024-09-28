// https://leetcode.com/problems/check-whether-two-strings-are-almost-equivalent/description/
pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
    let mut freq = vec![0; 26];
    let w1 = word1.chars().collect::<Vec<_>>();
    let w2 = word2.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut j = 0;
    while i < w1.len() || j < w2.len() {
        if i < w1.len() {
            freq[w1[i] as usize - 'a' as usize] += 1;
            i += 1;
        }
        if j < w2.len() {
            freq[w2[j] as usize - 'a' as usize] -= 1;
            j += 1;
        }
    }
    for x in freq {
        if x > 3 || x < -3 {
            return false;
        }
    }
    true
}

// https://leetcode.com/problems/find-the-occurrence-of-first-almost-equal-substring/
pub fn min_starting_index(s: String, pattern: String) -> i32 {
    fn z_func(s: Vec<char>) -> Vec<usize> {
        let mut z = vec![0; s.len()];
        let mut l = 0;
        let mut r = 1;
        for i in 1..z.len() {
            if i < r {
                z[i] = z[i - l].min(r - i);
            }
            while i + z[i] < z.len() && s[i + z[i]] == s[z[i]] {
                z[i] += 1;
            }
            if i + z[i] >= r {
                l = i;
                r = i + z[i];
            }
        }
        z
    }

    let n = s.len();
    let m = pattern.len();
    let z1 = z_func((pattern.clone() + &s).chars().collect::<Vec<_>>());
    let z2 = z_func((s.clone() + &pattern).chars().rev().collect::<Vec<_>>());

    for i in 0..=(n - m) {
        if z1[m + i] + 1 + z2[n - i] >= m {
            return i as i32;
        }
    }
    -1
}
