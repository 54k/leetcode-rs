// https://leetcode.com/problems/minimize-the-maximum-difference-of-pairs/description/
pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
    fn is_valid(nums: &Vec<i32>, p: i32, threshold: i32) -> bool {
        let mut index = 0;
        let mut count = 0;
        while index < nums.len() - 1 {
            if nums[index + 1] - nums[index] <= threshold {
                count += 1;
                index += 1;
            }
            index += 1;
        }
        count >= p
    }
    nums.sort();
    let (mut left, mut right) = (0, nums[nums.len() - 1] - nums[0]);
    while left < right {
        let mid = left + (right - left) / 2;
        if is_valid(&nums, p, mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

// https://leetcode.com/problems/minimum-absolute-difference/description/
pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut min = i32::MAX;
    let mut arr = arr;
    arr.sort();
    for i in 0..arr.len() - 1 {
        if arr[i + 1] - arr[i] < min {
            ans.clear();
            ans.push(vec![arr[i], arr[i + 1]]);
            min = arr[i + 1] - arr[i];
        } else if arr[i + 1] - arr[i] == min {
            ans.push(vec![arr[i], arr[i + 1]]);
        }
    }
    ans
}

// https://leetcode.com/problems/minimum-difference-between-largest-and-smallest-value-in-three-moves/
pub fn min_difference(mut nums: Vec<i32>) -> i32 {
    if nums.len() < 5 {
        return 0;
    }
    nums.sort();
    let n = nums.len();
    let mut res = i32::MAX;
    for i in 0..4 {
        res = res.min(nums[n - 4 + i] - nums[i]);
    }
    res
}

// https://leetcode.com/problems/minimum-score-by-changing-two-elements/description/
pub fn minimize_sum(mut nums: Vec<i32>) -> i32 {
    if nums.len() < 4 {
        return 0;
    }
    nums.sort();
    let n = nums.len();
    let mut ans = i32::MAX;

    for i in 0..3 {
        ans = ans.min(nums[n - 3 + i] - nums[i]);
    }

    ans
}

// https://leetcode.com/problems/robot-return-to-origin/description/
pub fn judge_circle(moves: String) -> bool {
    use std::collections::HashMap;
    let m = HashMap::from([('U', (0, 1)), ('D', (0, -1)), ('R', (1, 0)), ('L', (-1, 0))]);
    let (mut x, mut y) = (0, 0);
    for ch in moves.chars() {
        x += m[&ch].0;
        y += m[&ch].1;
    }
    return x == 0 && y == 0;
}
