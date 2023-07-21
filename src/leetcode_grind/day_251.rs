// https://leetcode.com/problems/largest-divisible-subset/description/
pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut nums = nums;
    nums.sort();
    let mut eds = vec![vec![]; n];
    for i in 0..nums.len() {
        let mut max_subset = vec![];

        for j in 0..i {
            if nums[i] % nums[j] == 0 && max_subset.len() < eds[j].len() {
                max_subset = eds[j].clone();
            }
        }

        eds[i].extend(max_subset);
        eds[i].push(nums[i]);
    }

    let mut ans = vec![];
    for i in 0..n {
        if ans.len() < eds[i].len() {
            ans = eds[i].clone();
        }
    }
    ans
}
