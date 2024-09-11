// https://leetcode.com/problems/minimum-bit-flips-to-convert-number/description/?envType=daily-question&envId=2024-09-11
pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
    let mut ans = 0;
    for i in 0..31 {
        if ((start >> i) & 1) != ((goal >> i) & 1) {
            ans += 1;
        }
    }
    ans
}
