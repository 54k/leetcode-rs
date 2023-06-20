// https://leetcode.com/problems/k-radius-subarray-averages/description/
pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut ans = vec![-1; nums.len()];
    let (mut start, mut sum) = (0, 0);
    for (end, &num) in nums.iter().enumerate() {
        sum += num as i64;
        if end as i32 >= 2 * k {
            ans[start + k as usize] = (sum / (2 * k + 1) as i64) as i32;
            sum -= nums[start] as i64;
            start += 1;
        }
    }
    ans
}

// https://leetcode.com/problems/bitwise-and-of-numbers-range/description/
pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
    let mut shift = 0;
    while left < right {
        left >>= 1;
        right >>= 1;
        shift += 1;
    }
    right << shift
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
