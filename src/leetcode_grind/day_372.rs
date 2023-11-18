// https://leetcode.com/problems/reduction-operations-to-make-the-array-elements-equal/description
pub fn reduction_operations1(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut ops = 0;
    let mut gs = 1;
    for i in (0..nums.len() - 1).rev() {
        if nums[i] != nums[i + 1] {
            ops += gs;
        }
        gs += 1;
    }
    ops
}

pub fn reduction_operations2(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut ops = 0;
    let mut up = 0;
    for i in 1..nums.len() {
        if nums[i] != nums[i - 1] {
            up += 1;
        }
        ops += up;
    }
    ops
}
