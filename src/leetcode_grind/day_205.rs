// https://leetcode.com/problems/paint-house/description/
pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
    fn top_down(costs: Vec<Vec<i32>>) -> i32 {
        fn dp(costs: &Vec<Vec<i32>>, i: usize, color: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
            if i == costs.len() {
                return 0;
            }

            if cache[i][color] == -1 {
                let mut cost = i32::MAX;

                for next in 0..3 {
                    if next != color {
                        let c = costs[i][color] + dp(costs, i + 1, next, cache);
                        cost = cost.min(c);
                    }
                }
                cache[i][color] = cost;
            }
            cache[i][color]
        }
        let mut cache = vec![vec![-1; 3]; costs.len()];
        dp(&costs, 0, 0, &mut cache)
            .min(dp(&costs, 0, 1, &mut cache))
            .min(dp(&costs, 0, 2, &mut cache))
    }

    fn bottom_up(costs: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; 3]; costs.len() + 1];

        for i in (0..costs.len()).rev() {
            for j in 0..3 {
                let mut min = i32::MAX;
                for k in 0..3 {
                    if k != j {
                        min = min.min(dp[i + 1][k]);
                    }
                }
                dp[i][j] = costs[i][j] + min;
            }
        }

        let mut ans = i32::MAX;
        for i in 0..3 {
            ans = ans.min(dp[0][i]);
        }
        ans
    }

    bottom_up(costs)
}

pub fn min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
    fn bottom_up(costs: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (costs.len(), costs[0].len());
        let mut dp = vec![vec![1000_000_000; m]; n + 1];
        dp[0] = vec![0; m];
        for i in 1..=n {
            for j in 0..m {
                for k in 0..m {
                    if j == k {
                        continue;
                    }
                    dp[i][j] = dp[i][j].min(costs[i - 1][j] + dp[i - 1][k])
                }
            }
        }
        dp[n].iter().copied().min().unwrap()
    }
    fn bottom_up_space_optimized(costs: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
    bottom_up(costs)
}
// https://leetcode.com/problems/h-index/
pub fn h_index(mut citations: Vec<i32>) -> i32 {
    pub fn h_index_ON(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut papers = vec![0; n + 1];
        for &c in &citations {
            papers[n.min(c as usize)] += 1;
        }
        let mut k = n;
        let mut s = papers[n];
        while k > s {
            k -= 1;
            s += papers[k];
        }
        k as i32
    }
    fn h_index_OLOGN(mut citations: Vec<i32>) -> i32 {
        citations.sort();
        let mut i = 0;
        let h = citations.len();
        while i < h && citations[citations.len() - 1 - i] > i as i32 {
            i += 1;
        }
        i as i32
    }
    h_index_ON(citations)
}

// https://leetcode.com/problems/h-index-ii/description/
pub fn h_index_ii(citations: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/minimum-falling-path-sum-ii/
pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test510() {
        println!(
            "{}",
            min_cost(vec![vec![17, 2, 17], vec![16, 16, 5], vec![14, 3, 19]])
        );
    }
}

// https://leetcode.com/problems/stone-game-iv/description/
pub fn winner_square_game(n: i32) -> bool {
    todo!()
}

// https://leetcode.com/problems/stone-game-v/description/
pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/stone-game-vi/description/
pub fn stone_game_vi(alice_values: Vec<i32>, bob_values: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/stone-game-vii/description/
pub fn stone_game_vii(stones: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/description/
pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/range-sum-query-2d-mutable/description/
mod rmq2d {
    struct NumMatrix {}

    impl NumMatrix {
        fn new(matrix: Vec<Vec<i32>>) -> Self {
            todo!()
        }

        fn update(&self, row: i32, col: i32, val: i32) {
            todo!()
        }

        fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
            todo!()
        }
    }
}

// https://leetcode.com/problems/maximize-grid-happiness/description/
pub fn get_max_grid_happiness(m: i32, n: i32, introverts_count: i32, extroverts_count: i32) -> i32 {
    todo!()
}
