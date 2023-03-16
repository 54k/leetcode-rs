// https://leetcode.com/problems/min-cost-climbing-stairs/description/
// https://leetcode.com/problems/min-cost-climbing-stairs/solutions/110074/short-o-1-space-ruby-python-java-c/
pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    fn with_dp_array(cost: Vec<i32>) -> i32 {
        let mut dp = vec![0; cost.len() + 1];
        for i in 2..dp.len() {
            dp[i] = (dp[i - 2] + cost[i - 2]).min(dp[i - 1] + cost[i - 1]);
        }
        dp[cost.len()]
    }
    fn short(cost: Vec<i32>) -> i32 {
        let mut a = 0;
        let mut b = 0;
        let mut min = 0;
        for c in cost {
            b = a;
            a = c + min;
            min = a.min(b);
        }
        min
    }
    assert_eq!(with_dp_array(cost.clone()), short(cost.clone()));
    short(cost)
}

// https://leetcode.com/problems/triangle/description/
pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    fn top_down(mut triangle: Vec<Vec<i32>>) -> i32 {
        let mut ans = i32::MAX;
        for i in 1..triangle.len() {
            for j in 0..triangle[i].len() {
                let left = if j > 0 { j - 1 } else { j };
                let right = if j < triangle[i - 1].len() { j } else { j - 1 };
                triangle[i][j] += triangle[i - 1][right].min(triangle[i - 1][left])
            }
        }
        for i in 0..triangle[triangle.len() - 1].len() {
            ans = ans.min(triangle[triangle.len() - 1][i]);
        }
        ans
    }
    fn bottom_up(mut triangle: Vec<Vec<i32>>) -> i32 {
        // Traverse the triangle from bottom to top
        // The minimum path sum is stored in the first element of the dp array
        // For each element in the current row, update the dp array
        let h = triangle.len();
        for i in (0..h - 1).rev() {
            for j in 0..triangle[i].len() {
                triangle[i][j] += triangle[i + 1][j + 1].min(triangle[i + 1][j]);
            }
        }
        triangle[0][0]
    }
    bottom_up(triangle)
}

// https://leetcode.com/problems/maximal-square/description/
// https://leetcode.com/problems/maximal-square/editorial/
pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let mut max_square = 0;
    let mut dp = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
    for i in 1..=matrix.len() {
        for j in 1..=matrix[0].len() {
            if matrix[i - 1][j - 1] == '1' {
                dp[i][j] = dp[i - 1][j].min(dp[i - 1][j - 1]).min(dp[i][j - 1]) + 1;
            }
            max_square = max_square.max(dp[i][j]);
        }
    }
    max_square * max_square
}

#[cfg(test)]
mod test {
    // todo https://leetcode.com/list/55ac4kuc/
    use super::*;

    #[test]
    fn test_min_cost_climbing_stairs() {
        println!("{}", min_cost_climbing_stairs(vec![10, 15, 20])); // 15
        println!(
            "{}",
            min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
        ); // 6
    }

    #[test]
    fn test_minimum_total() {
        println!(
            "{}",
            minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]])
        ); // 11

        println!("{}", minimum_total(vec![vec![-10]])); // -10
    }

    #[test]
    fn test_maximal_square() {
        println!(
            "{}",
            maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1']
            ])
        ); // 4
    }
}
