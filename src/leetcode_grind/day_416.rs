// https://leetcode.com/problems/convert-an-array-into-a-2d-array-with-conditions/description
pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut freq = vec![0; nums.len() + 1];
    let mut ans = vec![];
    for i in 0..nums.len() {
        if freq[nums[i] as usize] >= ans.len() {
            ans.push(vec![]);
        }
        ans[freq[nums[i] as usize]].push(nums[i]);
        freq[nums[i] as usize] += 1;
    }
    ans
}
