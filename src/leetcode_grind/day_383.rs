// https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero/
pub fn minimum_one_bit_operations(n: i32) -> i32 {
    let mut ans = n;
    ans ^= ans >> 16;
    ans ^= ans >> 8;
    ans ^= ans >> 4;
    ans ^= ans >> 2;
    ans ^= ans >> 1;
    ans
}
