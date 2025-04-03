// https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-ii/description/?envType=daily-question&envId=2025-04-03
pub fn maximum_triplet_value_i(nums: Vec<i32>) -> i64 {
    let n = nums.len();        
    let mut left_max = vec![0; n];
    let mut right_max = vec![0; n];

    for i in 1..n {
        left_max[i] = left_max[i-1].max(nums[i-1]);
        right_max[n - i - 1] = right_max[n - i].max(nums[n - i]);
    }

    let mut res = 0i64;
    for j in 1..(n-1) {
        res = res.max((left_max[j] as i64 - nums[j] as i64) * right_max[j] as i64);
    }
    res
}

pub fn maximum_triplet_value_ii(nums: Vec<i32>) -> i64 {
    let mut res = 0;       
    let mut imax = 0;
    let mut dmax = 0;

    for &num in &nums {
        res = res.max(dmax * num as i64);
        dmax = dmax.max(imax - num as i64);
        imax = imax.max(num as i64);
    }

    res
}