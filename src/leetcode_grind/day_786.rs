// https://leetcode.com/problems/minimum-length-of-string-after-operations/description/?envType=daily-question&envId=2025-01-13
pub fn minimum_length(s: String) -> i32 {
    let mut freq = vec![0; 26];
    for ch in s.chars() {
        freq[ch as usize - 'a' as usize] += 1;
    }

    let mut total = 0;
    for f in freq {
        if f == 0 {
            continue;
        }

        if f % 2 == 0 {
            total += 2;
        } else {
            total += 1
        }
    }
    total
}
