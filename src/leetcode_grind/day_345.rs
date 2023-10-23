// https://leetcode.com/problems/maximum-subarray/description/
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    fn find_best_sub_array(nums: &Vec<i32>, left: i32, right: i32) -> i32 {
        if left > right {
            return i32::MIN;
        }

        let mid = (left + right) / 2;
        let mut curr = 0;
        let mut best_left_sum = 0;
        let mut best_right_sum = 0;

        for i in (0..mid as usize).rev() {
            curr += nums[i];
            best_left_sum = best_left_sum.max(curr);
        }

        curr = 0;
        for i in mid as usize + 1..nums.len() {
            curr += nums[i];
            best_right_sum = best_right_sum.max(curr);
        }

        let best_combined_sum = nums[mid as usize] + best_left_sum + best_right_sum;

        let left_half = find_best_sub_array(nums, left, mid - 1);
        let right_half = find_best_sub_array(nums, mid + 1, right);

        best_combined_sum.max(left_half.max(right_half))
    }

    find_best_sub_array(&nums, 0, nums.len() as i32 - 1)
}

// https://leetcode.com/problems/meeting-rooms/description/
pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
    if intervals.is_empty() {
        return true;
    }
    let mut intervals = intervals;
    intervals.sort();
    for i in 0..intervals.len() - 1 {
        if intervals[i][1] > intervals[i + 1][0] {
            return false;
        }
    }
    true
}

// https://leetcode.com/problems/points-that-intersect-with-cars/description/
pub fn number_of_points_brute(nums: Vec<Vec<i32>>) -> i32 {
    let max = nums.iter().map(|x| x[1]).max().unwrap();
    let mut points = vec![false; max as usize + 1];
    for num in nums {
        let start = num[0];
        let end = num[1];
        for i in start..=end {
            points[i as usize] = true;
        }
    }
    let mut ans = 0;
    for i in 1..points.len() {
        if points[i] {
            ans += 1;
        }
    }
    ans
}

pub fn number_of_points_better_brute(nums: Vec<Vec<i32>>) -> i32 {
    let mut covered = vec![0; 102];
    for num in nums {
        covered[num[0] as usize] += 1;
        covered[num[1] as usize + 1] -= 1;
    }
    for i in 1..covered.len() {
        covered[i] += covered[i - 1];
    }
    let mut ans = covered.len() as i32;
    for c in covered {
        if c == 0 {
            ans -= 1;
        }
    }
    ans
}

pub fn number_of_points_sort(nums: Vec<Vec<i32>>) -> i32 {
    let mut nums = nums;
    nums.sort_by_key(|x| x[0]);

    let mut ans = 0;
    let mut curr = nums[0].clone();

    for num in nums {
        if num[0] > curr[1] {
            ans += curr[1] - curr[0] + 1;
            curr = num;
        } else {
            // remove overlap/merge
            curr[1] = curr[1].max(num[1]);
        }
    }
    ans += curr[1] - curr[0] + 1;
    ans
}
