// https://leetcode.com/problems/optimal-division/description/
pub fn optimal_division(nums: Vec<i32>) -> String {
    if nums.len() == 1 {
        return format!("{}", nums[0]);
    }
    if nums.len() == 2 {
        return format!("{}/{}", nums[0], nums[1]);
    }
    let mut res = format!("{}/({}", nums[0], nums[1]);
    for i in 2..nums.len() {
        res.push_str(&format!("/{}", nums[i]));
    }
    res.push_str(")");
    res
}
