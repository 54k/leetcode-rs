// https://leetcode.com/problems/spiral-matrix-ii/description/
pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let mut res = vec![vec![0; n as usize]; n as usize];
    let mut cnt = 1;
    let dir = vec![vec![0, 1], vec![1, 0], vec![0, -1], vec![-1, 0]];
    let mut d = 0;
    let mut row = 0;
    let mut col = 0;

    while cnt <= n * n {
        res[row as usize][col as usize] = cnt;
        cnt += 1;

        let r = ((row + dir[d][0]) + n) % n;
        let c = ((col + dir[d][1]) + n) % n;

        if res[r as usize][c as usize] != 0 {
            d = (d + 1) % 4;
        }

        row += dir[d as usize][0];
        col += dir[d as usize][1];
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test495() {
        let r = generate_matrix(3);
        println!("{:?}", r);
    }
}
