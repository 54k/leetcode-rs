// https://leetcode.com/problems/check-if-number-is-a-sum-of-powers-of-three/description/
pub fn check_powers_of_three(mut n: i32) -> bool {
    while n > 0 {
        if n % 3 == 2 { 
            return false;
        }
        n /= 3;
    }
    true
}