// https://leetcode.com/problems/minimum-cost-to-make-array-equal/description/
pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
    pub fn using_prefix_sum(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut nums_and_cost = vec![];
        for i in 0..n {
            nums_and_cost.push((nums[i], cost[i]));
        }
        nums_and_cost.sort_by(|a, b| a.0.cmp(&b.0));

        let mut prefix_cost = vec![0; n];
        prefix_cost[0] = nums_and_cost[0].1 as i64;
        for i in 1..n {
            prefix_cost[i] = prefix_cost[i - 1] as i64 + nums_and_cost[i].1 as i64;
        }

        let mut total_cost = 0;
        for i in 1..n {
            total_cost += (nums_and_cost[i].1 as i64
                * (nums_and_cost[i].0 as i64 - nums_and_cost[0].0 as i64))
                as i64;
        }
        let mut ans = total_cost;

        for i in 1..n {
            let gap = (nums_and_cost[i].0 - nums_and_cost[i - 1].0) as i64;
            total_cost += prefix_cost[i - 1] as i64 * gap;
            total_cost -= (prefix_cost[n - 1] as i64 - prefix_cost[i - 1] as i64) * gap;
            ans = ans.min(total_cost);
        }

        ans
    }
    pub fn using_convex_func(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        fn get_cost(nums: &Vec<i32>, cost: &Vec<i32>, base: i32) -> i64 {
            let mut ans = 0;
            for i in 0..nums.len() {
                ans += (nums[i] - base).abs() as i64 * cost[i] as i64;
            }
            ans
        }

        let (mut left, mut right) = (1000001, 0);
        for &num in &nums {
            left = left.min(num);
            right = right.max(num);
        }
        let mut ans = get_cost(&nums, &cost, nums[0]);

        while left < right {
            let mid = (right + left) / 2;
            let cost1 = get_cost(&nums, &cost, mid);
            let cost2 = get_cost(&nums, &cost, mid + 1);
            ans = cost1.min(cost2);

            if cost1 > cost2 {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        ans
    }
    using_prefix_sum(nums, cost)
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
