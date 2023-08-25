// https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/description/
pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![];
    for i in 0..nums.len() {
        let mut smaller = 0;
        for j in 0..nums.len() {
            if i == j {
                continue;
            }
            if nums[j] < nums[i] {
                smaller += 1;
            }
        }
        ans.push(smaller);
    }
    ans
}
