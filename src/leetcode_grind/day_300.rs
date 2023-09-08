// https://leetcode.com/problems/pascals-triangle/description/
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut ans = vec![vec![1]];
    for row in 1..num_rows as usize {
        let mut next = vec![1];
        for col in 1..ans[row - 1].len() {
            next.push(ans[row - 1][col - 1] + ans[row - 1][col]);
        }
        next.push(1);
        ans.push(next);
    }
    ans
}
