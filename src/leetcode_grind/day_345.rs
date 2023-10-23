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
