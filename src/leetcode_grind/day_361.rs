// https://leetcode.com/problems/determine-if-a-cell-is-reachable-at-a-given-time/
pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
    let x = (sx - fx).abs();
    let y = (sy - fy).abs();
    if x == 0 && y == 0 && t == 1 {
        return false;
    }
    x.max(y) <= t
}
