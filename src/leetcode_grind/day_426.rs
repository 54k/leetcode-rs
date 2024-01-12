// https://leetcode.com/problems/degree-of-an-array/description/
pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut left = HashMap::new();
    let mut right = HashMap::new();
    let mut count = HashMap::new();

    for i in 0..nums.len() {
        let x = nums[i];

        if !left.contains_key(&x) {
            left.insert(x, i);
        }

        right.insert(x, i);
        *count.entry(x).or_insert(0) += 1;
    }

    let mut ans = nums.len();
    let degree = *count.values().max().unwrap_or(&0);

    for x in count.keys() {
        if count[&x] == degree {
            ans = ans.min(right[&x] - left[&x] + 1);
        }
    }

    ans as i32
}
