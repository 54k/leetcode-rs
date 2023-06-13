// https://leetcode.com/problems/summary-ranges/description/
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut ranges = vec![];
    let mut i = 0;
    while i < nums.len() {
        let mut start = nums[i];

        while i + 1 < nums.len() && nums[i] + 1 == nums[i + 1] {
            i += 1;
        }

        if start != nums[i] {
            ranges.push(format!("{}->{}", start, nums[i]));
        } else {
            ranges.push(format!("{}", start));
        }
        i += 1;
    }
    ranges
}
