// https://leetcode.com/problems/maximum-distance-in-arrays/description/?envType=study-plan-v2&id=premium-algo-100
pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    let mut n = arrays[0].len();
    let mut min = arrays[0][0];
    let mut max = arrays[0][n - 1];

    for i in 1..arrays.len() {
        n = arrays[i].len();
        res = res.max(
            (arrays[i][n - 1] - min)
                .abs()
                .max((max - arrays[i][0]).abs()),
        );
        min = min.min(arrays[i][0]);
        max = max.max(arrays[i][n - 1]);
    }
    res
}

// https://leetcode.com/problems/delete-and-earn/description
pub fn delete_and_earn(mut nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for &num in nums.iter() {
        *map.entry(num).or_insert(0) += 1;
    }
    let mut dp = vec![0; nums.len()];
    for i in 0..nums.len() {
        
    }
    dp[nums.len() - 1]
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

// https://leetcode.com/problems/stone-game/description/
pub fn stone_game(piles: Vec<i32>) -> bool {
    let n = piles.len();
    let mut dp = vec![vec![0; n + 2]; n + 2];
    for size in 1..=n {
        for i in 0..=n - size {
            let j = i + size - 1;
            let parity = (j + i + n) % 2;
            if parity == 1 {
                dp[i + 1][j + 1] = (piles[i] + dp[i + 2][j + 1]).max(piles[j] + dp[i + 1][j]);
            } else {
                dp[i + 1][j + 1] = (-piles[i] + dp[i + 2][j + 1]).min(-piles[j] + dp[i + 1][j]);
            }
        }
    }
    return dp[1][n] > 0; // return true, alice always wins
}

// https://leetcode.com/problems/stone-game-iii/description/
pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
    let n = stone_value.len();
    let mut dp = vec![0; n + 1];
    for i in (0..n).rev() {
        dp[i] = stone_value[i] - dp[i + 1];
        if i + 2 <= n {
            dp[i] = dp[i].max(stone_value[i] + stone_value[i + 1] - dp[i + 2]);
        }
        if i + 3 <= n {
            dp[i] = dp[i].max(stone_value[i] + stone_value[i + 1] + stone_value[i + 2] - dp[i + 3]);
        }
    }
    if dp[0] > 0 {
        return "Alice".to_string();
    }
    if dp[0] < 0 {
        return "Bob".to_string();
    }
    "Tie".to_string()
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
