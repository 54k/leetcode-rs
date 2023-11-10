// https://leetcode.com/problems/number-of-ways-to-select-buildings/description
pub fn number_of_ways(s: String) -> i64 {
    let s = s.as_bytes();
    let mut ans = 0i64;

    let mut total_zeros = 0i64;
    for i in 0..s.len() {
        total_zeros += if s[i] == b'0' { 1 } else { 0 };
    }
    let total_ones = s.len() as i64 - total_zeros;

    let mut curr_zeros = if s[0] == b'0' { 1 } else { 0 };
    let mut curr_ones = if s[0] == b'1' { 1 } else { 0 };

    for i in 1..s.len() {
        if s[i] == b'1' {
            ans += (curr_zeros * (total_zeros - curr_zeros));
            curr_ones += 1;
        }
        if s[i] == b'0' {
            ans += (curr_ones * (total_ones - curr_ones));
            curr_zeros += 1;
        }
    }

    ans
}
