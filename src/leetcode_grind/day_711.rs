// https://leetcode.com/problems/longest-square-streak-in-an-array/description/?envType=daily-question&envId=2024-10-28
pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut longest_streak = 0;
    let mut unique_numbers = nums.iter().copied().collect::<HashSet<_>>();
    for start_number in nums {
        let mut current_streak = 0;
        let mut current = start_number as i64;

        while unique_numbers.contains(&(current as i32)) {
            current_streak += 1;
            if current * current > 1e5 as i64 {
                break;
            }
            current *= current;
        }

        longest_streak = longest_streak.max(current_streak);
    }
    if longest_streak < 2 {
        -1
    } else {
        longest_streak
    }
}
