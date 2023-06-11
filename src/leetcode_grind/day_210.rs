// https://leetcode.com/problems/maximum-value-at-a-given-index-in-a-bounded-array/description/
pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
    fn get_sum(index: i64, value: i64, n: i64) -> i32 {
        let mut count = 0;

        if value > index {
            count += (value + value - index) * (index + 1) / 2;
        } else {
            count += (value + 1) * value / 2 + index - value + 1;
        }

        if value >= n - index {
            count += (value + value - n + 1 + index) * (n - index) / 2;
        } else {
            count += (value + 1) * value / 2 + n - index - value;
        }

        (count - value) as i32
    }

    let mut left = 1;
    let mut right = max_sum;
    while left < right {
        let mid = (left + right + 1) / 2;
        if get_sum(index as i64, mid as i64, n as i64) <= max_sum {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left
}
