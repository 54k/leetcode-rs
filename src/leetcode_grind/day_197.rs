// https://leetcode.com/problems/minimum-cost-to-cut-a-stick/description/
pub fn min_cost(n: i32, cuts: Vec<i32>) -> i32 {
    let mut new_cuts = vec![0];
    new_cuts.extend(cuts.into_iter());
    new_cuts.push(n);
    new_cuts.sort();
    let m = new_cuts.len();

    let mut cache = vec![vec![-1; m + 2]; m + 2];

    fn dp(cuts: &[i32], left: i32, right: i32, cache: &mut Vec<Vec<i32>>) -> i32 {
        if cache[left as usize][right as usize] > -1 {
            return cache[left as usize][right as usize];
        }

        if left + 1 >= right {
            return 0;
        }

        let mut best = i32::MAX;
        let cost = cuts[right as usize] - cuts[left as usize];

        for mid in left + 1..right {
            let total_cost = cost + (dp(cuts, left, mid, cache)) + (dp(cuts, mid, right, cache));
            best = best.min(total_cost);
        }

        cache[left as usize][right as usize] = best;
        cache[left as usize][right as usize]
    }

    dp(&new_cuts, 0, m as i32 - 1, &mut cache)
}

// https://leetcode.com/problems/minimum-difficulty-of-a-job-schedule/description/
pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
    let d = d as usize;
    let n = job_difficulty.len();

    if n < d {
        return -1;
    }

    let mut dp = vec![vec![i32::MAX; d + 1]; n + 1];
    dp[n][d] = 0;
    for i in (0..n).rev() {
        dp[i][d] = dp[i + 1][d].max(job_difficulty[i]);
    }

    for day in (1..d as usize).rev() {
        for i in day - 1..n - (d - day) {
            let mut hardest = 0;
            for j in i..n - (d - day) {
                hardest = hardest.max(job_difficulty[j]);
                dp[i][day] = (hardest + dp[j + 1][day + 1]).min(dp[i][day]);
            }
        }
    }
    dp[0][1]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test508() {
        println!("{}", min_cost(7, vec![1, 3, 4, 5]));
    }

    #[test]
    fn test509() {
        println!("{}", min_difficulty(vec![6, 5, 4, 3, 2, 1], 2)); // 7
    }
}
