// https://leetcode.com/problems/shortest-completing-word/
pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
    let mut words = words;
    words.sort_by_key(|w| w.len());
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    for ch in license_plate.chars() {
        if char::is_alphabetic(ch) {
            let lower = char::to_lowercase(ch).next().unwrap();
            *freq.entry(lower).or_insert(0) += 1;
        }
    }

    for word in words {
        let mut f = freq.clone();
        for ch in word.chars() {
            let ch = char::to_lowercase(ch).next().unwrap();
            if f.contains_key(&ch) {
                *f.get_mut(&ch).unwrap() -= 1;
                if f[&ch] <= 0 {
                    f.remove(&ch);
                }
            }
        }
        if f.is_empty() {
            return word;
        }
    }
    panic!("oops");
}

// https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones/
pub fn check_ones_segment(s: String) -> bool {
    let mut seg = 0;
    let mut mx = 0;
    for ch in s.chars() {
        if ch == '1' {
            seg += 1;
            if (seg < mx) {
                return false;
            }
            mx = mx.max(seg);
        } else {
            seg = -1;
        }
    }
    true
}

// https://leetcode.com/problems/longer-contiguous-segments-of-ones-than-zeros/
pub fn check_zero_ones(s: String) -> bool {
    let mut c0 = 0;
    let mut c1 = 0;
    let mut m0 = 0;
    let mut m1 = 0;
    for ch in s.chars() {
        if ch == '1' {
            c1 += 1;
            c0 = 0;
        } else {
            c1 = 0;
            c0 += 1;
        }
        m0 = m0.max(c0);
        m1 = m1.max(c1);
    }
    m1 > m0
}

// https://leetcode.com/problems/minimum-common-value/description/
pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let (mut i, mut j) = (0, 0);
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] > nums2[j] {
            j += 1
        } else if nums1[i] == nums2[j] {
            return nums1[i];
        } else {
            i += 1;
        }
    }
    -1
}
