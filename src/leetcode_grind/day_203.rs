// https://leetcode.com/problems/time-needed-to-inform-all-employees/description/
pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut adj = vec![vec![]; n as usize];
    for (i, &e) in manager.iter().enumerate() {
        if i != head_id as usize {
            adj[e as usize].push((i, inform_time[i]));
        }
    }

    let mut lvl: Vec<(usize, i32)> = vec![];
    lvl.push((head_id as usize, inform_time[head_id as usize]));

    while lvl.len() > 0 {
        let mut next = vec![];
        for (v, time) in &lvl {
            for (u, t) in &adj[*v] {
                let time0 = *time + *t;
                ans = ans.max(time0);
                next.push((*u, time0));
            }
        }
        lvl = next;
    }

    ans
}

// https://leetcode.com/problems/maximum-sum-circular-subarray/description/
pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    fn approach1(nums: Vec<i32>) -> i32 {
        // As a circular array, the maximum subarray sum can be either the maximum "normal sum"
        // which is the maximum sum of the ordinary array or a "special sum" which is the maximum sum
        // of a prefix sum and a suffix sum of the ordinary array where the prefix and suffix don't overlap.
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        let mut right_max = vec![0; n];
        right_max[n - 1] = nums[n - 1];
        let mut suffix_sum = nums[n - 1];
        for i in (0..=(n - 2)).rev() {
            suffix_sum += nums[i];
            right_max[i] = right_max[i + 1].max(suffix_sum);
        }

        let mut max_sum = nums[0];
        let mut special_sum = nums[0];
        let mut cur_max = 0;
        let mut prefix_sum = 0;
        for i in 0..n {
            cur_max = cur_max.max(0) + nums[i];
            max_sum = max_sum.max(cur_max);
            prefix_sum += nums[i];
            if i + 1 < n {
                special_sum = special_sum.max(prefix_sum + right_max[i + 1]);
            }
        }

        max_sum.max(special_sum)
    }
    fn approach2(nums: Vec<i32>) -> i32 {
        let (mut cur_max, mut cur_min, mut sum) = (0, 0, 0);
        let (mut max_sum, mut min_sum) = (nums[0], nums[0]);
        for num in nums {
            cur_max = cur_max.max(0) + num;
            max_sum = max_sum.max(cur_max);
            cur_min = cur_min.min(0) + num;
            min_sum = min_sum.min(cur_min);
            sum += num;
        }
        if sum == min_sum {
            max_sum
        } else {
            max_sum.max(sum - min_sum)
        }
    }
    approach1(nums)
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
