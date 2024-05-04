// https://leetcode.com/problems/maximum-number-of-operations-with-the-same-score-i/description/
pub fn max_operations(mut nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut prev = -1;
    for i in (0..=nums.len() - 2).step_by(2) {
        println!("{i}");
        let s = nums[i] + nums[i + 1];
        if prev == -1 {
            prev = s;
        } else if prev != s {
            break;
        }
        ans += 1;
    }
    ans
}

// https://leetcode.com/problems/wiggle-subsequence/description
pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums.len() as i32;
    }
    let mut prev_diff = nums[1] - nums[0];
    let mut count = if prev_diff != 0 { 2 } else { 1 };
    for i in 2..nums.len() {
        let mut diff = nums[i] - nums[i - 1];
        if ((diff > 0 && prev_diff <= 0) || (diff < 0 && prev_diff >= 0)) {
            count += 1;
            prev_diff = diff;
        }
    }
    count
}

// https://leetcode.com/problems/longest-valid-parentheses/description
pub fn longest_valid_parentheses(s: String) -> i32 {
    let s = s.as_bytes();
    let mut left = 0;
    let mut right = 0;
    let mut max_len = 0;

    for i in 0..s.len() {
        if s[i] == b'(' {
            left += 1;
        } else {
            right += 1;
        }
        if left == right {
            max_len = max_len.max(2 * right);
        } else if right > left {
            left = 0;
            right = 0;
        }
    }

    left = 0;
    right = 0;

    for i in (0..s.len()).rev() {
        if s[i] == b'(' {
            left += 1;
        } else {
            right += 1;
        }
        if left == right {
            max_len = max_len.max(2 * left);
        } else if left > right {
            left = 0;
            right = 0;
        }
    }

    max_len
}
