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
pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
    fn top_down(nums: Vec<i32>) -> i32 {
        fn rec(num: i32, points: &HashMap<i32, i32>, cache: &mut HashMap<i32, i32>) -> i32 {
            if num == 0 {
                return 0;
            } else if num == 1 {
                return *points.get(&num).unwrap_or(&0);
            }

            if cache.contains_key(&num) {
                return cache[&num];
            }

            let gain = *points.get(&num).unwrap_or(&0);
            let a = rec(num - 1, points, cache);
            let b = rec(num - 2, points, cache) + gain;
            cache.insert(num, a.max(b));

            return cache[&num];
        }
        use std::collections::HashMap;
        let mut points = HashMap::new();
        let mut cache = HashMap::new();
        let mut max = 0;
        for &num in &nums {
            *points.entry(num).or_insert(0) += num;
            max = max.max(num);
        }
        rec(max, &points, &mut cache)
    }

    fn bottom_up(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut points = HashMap::new();
        let mut max = 0;
        for &num in &nums {
            *points.entry(num).or_insert(0) += num;
            max = max.max(num);
        }

        let mut max_points = vec![0; max as usize + 1];
        max_points[1] = *points.get(&1).unwrap_or(&0);

        for num in 2..max_points.len() as i32 {
            let gain = *points.get(&num).unwrap_or(&0);
            max_points[num as usize] =
                max_points[num as usize - 1].max(max_points[num as usize - 2] + gain);
        }

        max_points[max as usize]
    }

    top_down(nums)
}

// https://leetcode.com/problems/maximum-score-from-performing-multiplication-operations/description/
pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
    pub fn maximum_score_bottom_up(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = multipliers.len();
        let mut dp = vec![vec![0; m + 1]; m + 1];
        for i in (0..m).rev() {
            for left in (0..=i).rev() {
                let right = n - 1 - (i - left);
                let mul = multipliers[i];
                dp[i][left] = (nums[left] * mul + dp[i + 1][left + 1])
                    .max(nums[right] * mul + dp[i + 1][left]);
            }
        }
        dp[0][0]
    }
    maximum_score_bottom_up(nums, multipliers)
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