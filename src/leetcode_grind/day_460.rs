// https://leetcode.com/problems/1-bit-and-2-bit-characters/description/
pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
    let mut i = 0;
    while i < bits.len() - 1 {
        i += bits[i] as usize + 1;
    }
    i == bits.len() - 1
}
