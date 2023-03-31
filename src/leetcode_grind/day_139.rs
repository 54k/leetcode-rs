// https://leetcode.com/problems/number-of-ways-of-cutting-a-pizza/description/
// https://leetcode.com/problems/number-of-ways-of-cutting-a-pizza/editorial/
pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
    let rows = pizza.len();
    let cols = pizza[0].len();
    let pizza = pizza
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut apples = vec![vec![0; cols + 1]; rows + 1];
    let mut dp = vec![vec![vec![0; cols + 1]; rows + 1]; k as usize];

    for row in (0..rows).rev() {
        for col in (0..cols).rev() {
            apples[row][col] = if pizza[row][col] == 'A' { 1 } else { 0 }
                + apples[row + 1][col]
                + apples[row][col + 1]
                - apples[row + 1][col + 1];
            dp[0][row][col] = if apples[row][col] > 0 { 1 } else { 0 };
        }
    }

    const MOD: i32 = 1000000007;

    for remain in 1..k as usize {
        for row in 0..rows {
            for col in 0..cols {
                for next_row in row + 1..rows {
                    if apples[row][col] - apples[next_row][col] > 0 {
                        dp[remain][row][col] += dp[remain - 1][next_row][col];
                        dp[remain][row][col] %= MOD;
                    }
                }

                for next_col in col + 1..cols {
                    if apples[row][col] - apples[row][next_col] > 0 {
                        dp[remain][row][col] += dp[remain - 1][row][next_col];
                        dp[remain][row][col] %= MOD;
                    }
                }
            }
        }
    }

    dp[k as usize - 1][0][0]
}

// https://leetcode.com/problems/range-sum-query-2d-immutable/
// https://leetcode.com/problems/range-sum-query-2d-immutable/editorial/
pub struct NumMatrix {
    dp: Vec<Vec<i32>>,
}
impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut dp = vec![vec![0; matrix[0].len() + 1]; matrix.len()];
        for r in 0..=matrix.len() {
            for c in 0..=matrix[0].len() {
                dp[r + 1][c + 1] = dp[r + 1][c] + dp[r][c + 1] + matrix[r][c] - dp[r][c];
            }
        }
        Self { dp }
    }
    pub fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.dp[row2 as usize + 1][col2 as usize + 1]
            - self.dp[row1 as usize][col2 as usize + 1]
            - self.dp[row2 as usize + 1][col1 as usize]
            + self.dp[row1 as usize][col1 as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test388() {
        println!(
            "{}",
            ways(
                vec!["A..".to_string(), "AAA".to_string(), "...".to_string()],
                3
            )
        ); // 3
    }
}
