// https://leetcode.com/problems/maximum-sum-with-exactly-k-elements/
pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
    nums.into_iter().max().unwrap() * k + (k - 1) * k / 2
}

// https://leetcode.com/problems/sum-of-absolute-differences-in-a-sorted-array/description/
pub fn get_sum_absolute_differences_1(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans = vec![];
    let mut prefix = vec![0; n];
    prefix[0] = nums[0];
    for i in 1..n {
        prefix[i] = prefix[i - 1] + nums[i];
    }
    for i in 0..n {
        let left = prefix[i] - nums[i];
        let right = prefix[n - 1] - prefix[i];
        let left_count = i as i32;
        let right_count = (n - i - 1) as i32;
        let left_total = left_count * nums[i] - left;
        let right_total = right - right_count * nums[i];
        ans.push(left_total + right_total);
    }
    ans
}

pub fn get_sum_absolute_differences_2(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut total_sum = 0;
    for &num in &nums {
        total_sum += num;
    }

    let mut left_sum = 0;
    let mut ans = vec![0; n];

    for i in 0..n {
        let right_sum = total_sum - left_sum - nums[i];

        let left_count = i as i32;
        let right_count = (n - 1 - i) as i32;

        let left_total = left_count * nums[i] - left_sum;
        let right_total = right_sum - right_count * nums[i];

        ans[i] = left_total + right_total;
        left_sum += nums[i];
    }

    ans
}
