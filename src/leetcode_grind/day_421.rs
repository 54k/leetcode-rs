// https://leetcode.com/problems/maximum-difference-between-increasing-elements/description/
pub fn maximum_difference(nums: Vec<i32>) -> i32 {
    let mut diff = i32::MIN;
    let mut mn = i32::MAX;
    for n in nums {
        if n <= mn {
            mn = n;
        } else {
            diff = diff.max(n - mn);
        }
    }
    if diff == i32::MIN {
        return -1;
    }
    diff
}
