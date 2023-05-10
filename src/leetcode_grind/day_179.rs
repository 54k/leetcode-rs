// https://leetcode.com/problems/spiral-matrix-iii/description/
pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
    let dr = vec![0, 1, 0, -1];
    let dc = vec![1, 0, -1, 0];

    let mut ans = vec![vec![0; 2]; (rows * cols) as usize];
    let mut t = 1;
    ans[0] = vec![r_start, c_start];
    if rows * cols == 1 {
        return ans;
    }

    let (mut r0, mut c0) = (r_start, c_start);

    for k in (1..2 * (rows + cols)).step_by(2) {
        for i in 0..4 {
            // i: direction index
            let dk = k + (i / 2); // number of steps in this direction
            for j in 0..dk {
                // step in the i-th direction
                r0 += dr[i as usize];
                c0 += dc[i as usize];

                if 0 <= r0 && r0 < rows && 0 <= c0 && c0 < cols {
                    ans[t] = vec![r0, c0];
                    t += 1;
                    if t as i32 == rows * cols {
                        return ans;
                    }
                }
            }
        }
    }
    unreachable!()
}

// https://leetcode.com/problems/spiral-matrix-iv/
pub fn spiral_matrix(m: i32, n: i32, mut head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
    let mut dir = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut d = 0;
    let (mut row, mut col) = (0, 0);
    let mut cnt = 1;
    let mut ans = vec![vec![-1; n as usize]; m as usize];

    while head.is_some() && cnt <= m * n {
        let mut node = head.take().unwrap();
        head = node.next.take();
        cnt += 1;
        ans[row as usize][col as usize] = node.val;

        let r = (row + dir[d].0 + m) % m;
        let c = (col + dir[d].1 + n) % n;

        if ans[r as usize][c as usize] != -1 {
            d = (d + 1) % 4;
        }

        row += dir[d as usize].0;
        col += dir[d as usize].1;
    }
    ans
}
