// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/description/
pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    let n = prices.len();
    let (mut free, mut hold) = (0, -prices[0]);

    for i in 1..n {
        let tmp = hold;
        hold = hold.max(free - prices[i]);
        free = free.max(tmp + prices[i] - fee);
    }

    free
}

// https://leetcode.com/problems/single-number-iii/description/
pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let mut mask = 0;
    for &num in &nums {
        mask ^= num;
    }

    let diff = mask & (-mask);
    let mut x = 0;

    for &num in &nums {
        if (num & diff) != 0 {
            x ^= num;
        }
    }

    vec![x, mask ^ x]
}

// https://leetcode.com/problems/maximum-students-taking-exam/description/
// https://leetcode.com/problems/maximum-students-taking-exam/solutions/503686/a-simple-tutorial-on-this-bitmasking-problem/
pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
    todo!("wrong");
    let (m, n) = (seats.len(), seats[0].len());

    let mut valid_rows = vec![0; m];
    for i in 0..m {
        for j in 0..n {
            valid_rows[i] = (valid_rows[i] << 1) + if seats[i][j] == '.' { 1 } else { 0 };
        }
    }

    let state_size = 1 << n; // There are 2^n states for n columns in binary format
    let mut dp = vec![vec![-1i32; n]; state_size];

    let mut ans = 0;

    for i in 0..m {
        for j in 0..state_size {
            // (j & valid) == j: check if j is a subset of valid
            // !(j & (j >> 1)): check if there is no adjancent students in the row

            if (j & valid_rows[i]) == j && (j & (j >> 1)) == 0 {
                if i == 0 {
                    dp[i][j] = j.count_ones() as i32;
                } else {
                    for k in 0..state_size {
                        // !(j & (k >> 1)): no students in the upper left positions
                        // !((j >> 1) & k): no students in the upper right positions
                        // dp[i-1][k] != -1: the previous state is valid

                        if (j & (k >> 1)) == 0 && ((j >> 1) & k) == 0 && dp[i - 1][k] != -1 {
                            dp[i][j] = dp[i][j].max(dp[i - 1][k] + j.count_ones() as i32);
                        }
                    }
                }

                ans = ans.max(dp[i][j]);
            }
        }
    }
    ans
}

// https://leetcode.com/problems/minimum-falling-path-sum-ii/
pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
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
