// https://leetcode.com/problems/find-kth-bit-in-nth-binary-string/
pub fn find_kth_bit(n: i32, k: i32) -> char {
    let mut pos_in_section = k & -k;
    let mut is_in_inverted_part = (((k / pos_in_section) >> 1) & 1) == 1;
    let mut original_bit_is_one = (k & 1) == 0;
    if is_in_inverted_part {
        if original_bit_is_one {
            '0'
        } else {
            '1'
        }
    } else {
        if original_bit_is_one {
            '1'
        } else {
            '0'
        }
    }
}
