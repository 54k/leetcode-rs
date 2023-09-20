// https://leetcode.com/problems/maximum-size-subarray-sum-equals-k/description/
pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;

    let mut prefix = HashMap::new();
    let mut sum = 0;
    let mut max_len = 0;

    for right in 0..nums.len() {
        sum += nums[right];

        if sum == k {
            max_len = right as i32 + 1;
        }

        if prefix.contains_key(&(sum - k)) {
            max_len = max_len.max(right as i32 - prefix[&(sum - k)]);
        }

        if !prefix.contains_key(&sum) {
            prefix.insert(sum, right as i32);
        }
    }
    max_len
}

#[test]
fn test_max_sub_array_len() {
    assert_eq!(max_sub_array_len(vec![1, -1, 5, -2, 3], 3), 4);
    assert_eq!(max_sub_array_len(vec![-2, -1, 2, 1], 1), 2);
}

// https://leetcode.com/problems/minimum-operations-to-reduce-x-to-zero/description
pub fn min_operations_i(nums: Vec<i32>, x: i32) -> i32 {
    let mut total = 0;
    for &n in &nums {
        total += n;
    }

    let n = nums.len();
    let mut maxi = -1;
    let mut left = 0;
    let mut current = 0;

    for right in 0..n {
        current += nums[right];

        while current > total - x && left <= right {
            current -= nums[left];
            left += 1;
        }

        if current == total - x {
            maxi = maxi.max(right as i32 - left as i32 + 1);
        }
    }

    if maxi != -1 {
        n as i32 - maxi
    } else {
        -1
    }
}

pub fn min_operations_ii(nums: Vec<i32>, x: i32) -> i32 {
    let mut current = 0;
    for &n in &nums {
        current += n;
    }

    let n = nums.len();
    let mut mini = i32::MAX;
    let mut left = 0;

    for right in 0..n {
        current -= nums[right];

        while current < x && left <= right {
            current += nums[left];
            left += 1;
        }

        if current == x {
            mini = mini.min((n as i32 - 1 - right as i32) + left as i32);
        }
    }

    if mini != i32::MAX {
        mini
    } else {
        -1
    }
}
