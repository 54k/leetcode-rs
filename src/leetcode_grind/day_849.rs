// https://leetcode.com/problems/divide-array-into-equal-pairs/description/?envType=daily-question&envId=2025-03-17
pub fn divide_array(nums: Vec<i32>) -> bool {
    let max_num = *nums.iter().max().unwrap_or(&0);
    let mut needs_pair = vec![false; max_num as usize + 1];
    for &num in &nums {
        needs_pair[num as usize] = !needs_pair[num as usize];
    }
    for &num in &nums {
        if needs_pair[num as usize] {
            return false;
        }
    }
    true
}