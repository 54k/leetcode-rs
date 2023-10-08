// https://leetcode.com/problems/max-dot-product-of-two-subsequences/description
pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut first_max = i32::MIN;
    let mut second_max = i32::MIN;
    let mut first_min = i32::MAX;
    let mut second_min = i32::MAX;

    for i in 0..nums1.len() {
        first_max = first_max.max(nums1[i]);
        first_min = first_min.min(nums1[i]);
    }

    for i in 0..nums2.len() {
        second_max = second_max.max(nums2[i]);
        second_min = second_min.min(nums2[i]);
    }

    if first_max < 0 && second_min > 0 {
        return first_max * second_min;
    }

    if first_min > 0 && second_max < 0 {
        return first_min * second_max;
    }

    let mut dp = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
    for i in (0..nums1.len()).rev() {
        for j in (0..nums2.len()).rev() {
            let skip1 = dp[i + 1][j];
            let skip2 = dp[i][j + 1];
            let prev = dp[i + 1][j + 1];

            dp[i][j] = skip1.max(skip2).max(nums1[i] * nums2[j] + prev);
        }
    }
    dp[0][0]
}
