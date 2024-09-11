// https://leetcode.com/problems/minimum-bit-flips-to-convert-number/description/?envType=daily-question&envId=2024-09-11
pub fn min_bit_flips_i(start: i32, goal: i32) -> i32 {
    let mut ans = 0;
    for i in 0..31 {
        if ((start >> i) & 1) != ((goal >> i) & 1) {
            ans += 1;
        }
    }
    ans
}

pub fn min_bit_flips_ii(start: i32, goal: i32) -> i32 {
    let mut xor_result = start ^ goal;
    let mut count = 0;
    while xor_result != 0 {
        count += xor_result & 1;
        xor_result >>= 1;
    }
    count
}

pub fn min_bit_flips_iii(start: i32, goal: i32) -> i32 {
    let mut xor_result = start ^ goal;
    let mut count = 0;
    while xor_result != 0 {
        xor_result = xor_result & (xor_result - 1);
        count += 1;
    }
    count
}
