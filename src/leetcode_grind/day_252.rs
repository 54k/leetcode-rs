// https://leetcode.com/problems/knight-probability-in-chessboard/description/
pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
    const DIRS: [(i32, i32); 8] = [
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
    ];
    let mut dp = vec![vec![vec![0.0; n as usize]; n as usize]; k as usize + 1];
    dp[k as usize][row as usize][column as usize] = 1.0;

    for moves in (1..=k as usize).rev() {
        for i in 0..n {
            for j in 0..n {
                for (x, y) in &DIRS {
                    let (nx, ny) = (i + x, j + y);

                    if 0 <= nx && nx < n && 0 <= ny && ny < n {
                        dp[moves - 1][nx as usize][ny as usize] +=
                            dp[moves][i as usize][j as usize] / 8.0
                    }
                }
            }
        }
    }

    let mut probability = 0.0;
    for i in 0..n as usize {
        for j in 0..n as usize {
            probability += dp[0][i][j];
        }
    }
    probability
}
