// https://leetcode.com/problems/find-score-of-an-array-after-marking-all-elements/description/
pub fn find_score(nums: Vec<i32>) -> i64 {
    let mut nums: Vec<_> = nums.into_iter().enumerate().map(|x| (x.1, x.0)).collect();
    nums.sort();
    let mut marked = vec![false; nums.len()];
    let mut ans = 0;
    for i in 0..nums.len() {
        if marked[nums[i].1] {
            continue;
        }
        ans += nums[i].0 as i64;

        marked[nums[i].1] = true;
        if nums[i].1 >= 1 {
            marked[nums[i].1 - 1] = true;
        }
        if nums[i].1 < marked.len() - 1 {
            marked[nums[i].1 + 1] = true;
        }
    }
    ans
}
