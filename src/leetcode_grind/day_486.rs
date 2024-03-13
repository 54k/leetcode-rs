// https://leetcode.com/problems/find-the-pivot-integer/description/
pub fn pivot_integer(n: i32) -> i32 {
    let mut left_val = 1;
    let mut right_val = n;
    let mut sum_left = left_val;
    let mut sum_right = right_val;

    if n == 1 {
        return n;
    }

    while left_val < right_val {
        if sum_left < sum_right {
            left_val += 1;
            sum_left += left_val;
        } else {
            right_val -= 1;
            sum_right += right_val;
        }

        if sum_left == sum_right && left_val + 1 == right_val - 1 {
            return left_val + 1;
        }
    }
    return -1;
}
