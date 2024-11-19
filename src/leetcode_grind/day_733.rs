// https://leetcode.com/problems/maximum-sum-of-distinct-subarrays-with-length-k/description/?envType=daily-question&envId=2024-11-19
pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let mut ans = 0i64;
    let mut current_sum = 0i64;
    let mut begin = 0;
    let mut end = 0;

    use std::collections::HashMap;
    let mut num_to_index = HashMap::new();

    while end < nums.len() {
        let curr_num = nums[end];

        let last_occurrence = num_to_index.get(&curr_num);

        while last_occurrence.is_some() && begin <= *last_occurrence.unwrap()
            || end - begin + 1 > k as usize
        {
            current_sum -= nums[begin] as i64;
            begin += 1;
        }

        num_to_index.insert(curr_num, end);
        current_sum += nums[end] as i64;

        if end - begin + 1 == k as usize {
            ans = ans.max(current_sum);
        }

        end += 1;
    }

    ans
}
