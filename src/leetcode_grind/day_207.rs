// https://leetcode.com/problems/minimum-flips-to-make-a-or-b-equal-to-c/
pub fn min_flips(mut a: i32, mut b: i32, mut c: i32) -> i32 {
    // return Integer.bitCount((a | b) ^ c) + Integer.bitCount(a & b & ((a | b) ^ c));
    let mut ans = 0;
    while a != 0 || b != 0 || c != 0 {
        if (c & 1) == 1 {
            if (a & 1) == 0 && (b & 1) == 0 {
                ans += 1;
            }
        } else {
            ans += (a & 1) + (b & 1);
        }
        a >>= 1;
        b >>= 1;
        c >>= 1;
    }
    ans
}

// https://leetcode.com/problems/number-of-dice-rolls-with-target-sum/description/
pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut dp = vec![vec![0; target as usize + 1]; n as usize + 1];
    dp[n as usize][target as usize] = 1;
    for i in (0..n as usize).rev() {
        for sum in 0..=target as usize {
            let mut ways = 0;
            for j in 1..=(k as usize).min(target as usize - sum) {
                ways = (ways + dp[i + 1][sum + j]) % MOD
            }
            dp[i][sum] = ways;
        }
    }
    dp[0][0] as i32
}

// https://leetcode.com/problems/minimum-falling-path-sum-ii/
pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    todo!()
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
