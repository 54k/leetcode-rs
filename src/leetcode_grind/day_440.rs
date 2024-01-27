// https://leetcode.com/problems/rectangle-overlap/
pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
    fn min(a: i32, b: i32) -> i64 {
        return a.min(b) as i64;
    }
    fn max(a: i32, b: i32) -> i64 {
        return a.max(b) as i64;
    }
    return (min(rec1[2], rec2[2]) - max(rec1[0], rec2[0])).max(0)
        * (min(rec1[3], rec2[3]) - max(rec1[1], rec2[1])).max(0)
        > 0;
}
