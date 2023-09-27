// https://leetcode.com/problems/decoded-string-at-index
pub fn decode_at_index(s: String, k: i32) -> String {
    let mut k = k as i64;
    let mut size = 0;
    for ch in s.chars() {
        if ch.is_digit(10) {
            size *= ch.to_digit(10).unwrap() as i64;
        } else {
            size += 1;
        }
    }

    for ch in s.chars().rev() {
        k %= size;
        if k == 0 && ch.is_alphabetic() {
            return format!("{}", ch);
        }
        if ch.is_alphabetic() {
            size -= 1;
        } else {
            size /= ch.to_digit(10).unwrap() as i64;
        }
    }
    "".to_string()
}
