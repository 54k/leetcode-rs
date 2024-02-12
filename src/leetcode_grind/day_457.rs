// https://leetcode.com/problems/majority-element/
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut maj_el = 0;
    let len = nums.len() as i32;
    for i in 0..32 {
        let mask = 1 << i;
        let mut bc = 0;
        for &n in &nums {
            if (n & mask) != 0 {
                bc += 1;
            }
        }

        if bc > len / 2 {
            maj_el |= mask;
        }
    }
    maj_el
}
