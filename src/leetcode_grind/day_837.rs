// https://leetcode.com/problems/count-total-number-of-colored-cells/description/
pub fn colored_cells(n: i32) -> i64 {
    1 + (n as i64) * (n - 1) as i64 * 2
}