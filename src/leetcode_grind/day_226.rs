// https://leetcode.com/problems/total-cost-to-hire-k-workers/description/
pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
    use std::collections::BinaryHeap;
    let mut head = BinaryHeap::new();
    let mut tail = BinaryHeap::new();
    for i in 0..candidates as usize {
        head.push(-costs[i]);
    }
    for i in (candidates as usize).max(costs.len() - candidates as usize)..costs.len() {
        tail.push(-costs[i]);
    }
    let mut cost = 0;
    let mut i = candidates as usize;
    let mut j = costs.len() - candidates as usize - 1;

    for _ in 0..k {
        if tail.is_empty() || (!head.is_empty() && -head.peek().unwrap() <= -tail.peek().unwrap()) {
            cost += -head.pop().unwrap() as i64;
            if i < costs.len() && i <= j {
                head.push(-costs[i]);
                i += 1;
            }
        } else {
            cost += -tail.pop().unwrap() as i64;
            if i <= j {
                tail.push(-costs[j]);
                j -= 1;
            }
        }
    }
    cost
}

// https://leetcode.com/problems/destroy-sequential-targets/description/
pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
    todo!()
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
