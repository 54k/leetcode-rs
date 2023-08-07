// https://leetcode.com/problems/search-a-2d-matrix/
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (m, n) = (matrix.len() as i32, matrix[0].len() as i32);
    let (mut left, mut right) = (0i32, (m * n - 1) as i32);

    while left <= right {
        let mid = left + (right - left) / 2;
        let r = mid / n;
        let c = mid % n;
        if matrix[r as usize][c as usize] == target {
            return true;
        } else if matrix[r as usize][c as usize] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    false
}
