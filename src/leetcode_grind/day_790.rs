// https://leetcode.com/problems/neighboring-bitwise-xor/description/?envType=daily-question&envId=2025-01-17
pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
    let mut xor = 0;
    for el in derived {
        xor = xor ^ el;
    }
    xor == 0
}
