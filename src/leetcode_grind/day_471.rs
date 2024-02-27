// https://leetcode.com/problems/escape-the-ghosts/description/
pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
    fn taxi(a: &[i32], b: &[i32]) -> i32 {
        (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
    }

    for g in ghosts {
        if taxi(&g, &target) <= taxi(&[0, 0], &target) {
            return false;
        }
    }

    true
}
