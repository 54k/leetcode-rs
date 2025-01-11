// https://leetcode.com/problems/construct-k-palindrome-strings/description/?envType=daily-question&envId=2025-01-11
pub fn can_construct(s: String, k: i32) -> bool {
    if s.len() < k as usize {
        return false;
    }
    if s.len() == k as usize {
        return true;
    }

    let mut odd_count = 0;
    for chr in s.chars() {
        odd_count ^= (1 << (chr as i32 - 'a' as i32));
    }
    let mut cnt = 0;
    while odd_count > 0 {
        cnt += (odd_count & 1);
        odd_count >>= 1;
    }
    cnt <= k
}
