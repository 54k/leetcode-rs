// https://leetcode.com/problems/longest-alternating-subarray/
pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[j] == nums[i] + (j as i32 - i as i32) % 2 {
                ans = ans.max(j - i + 1);
            } else {
                break;
            }
        }
    }
    if ans < 1 {
        return -1;
    }
    ans as i32
}

// https://leetcode.com/problems/longest-turbulent-subarray/
pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
    let n = arr.len();
    let mut ans = 1;
    let mut anchor = 0;

    fn tmp(arr: &Vec<i32>, i: usize, j: usize) -> i32 {
        if arr[i] - arr[j] < 0 {
            -1
        } else if arr[i] == arr[j] {
            0
        } else {
            1
        }
    }

    for i in 1..n {
        let c = tmp(&arr, i - 1, i);
        if c == 0 {
            anchor = i;
        } else if i == n - 1 || tmp(&arr, i, i + 1) * c != -1 {
            ans = ans.max(i - anchor + 1);
            anchor = i;
        }
    }

    ans as i32
}
