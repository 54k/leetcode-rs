// https://leetcode.com/problems/tallest-billboard/description/
pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
    use std::collections::{HashMap, HashSet};
    let n = rods.len();

    fn subsets(rods: &Vec<i32>, left: usize, right: usize) -> HashMap<i32, i32> {
        let mut states = HashSet::new();
        states.insert((0, 0));

        for i in left..right {
            let r = rods[i];

            let mut new_states = HashSet::new();
            for (k, v) in &states {
                new_states.insert((*k + r, *v));
                new_states.insert((*k, *v + r));
            }

            states.extend(new_states);
        }

        let mut hm = HashMap::new();
        for (l, r) in states {
            let mut e = hm.entry(l - r).or_insert(0);
            *e = (*e).max(l);
        }
        hm
    }

    let left = subsets(&rods, 0, n / 2);
    let right = subsets(&rods, n / 2, n);

    // println!("{:?}", left);
    // println!("{:?}", right);

    let mut ans = 0;
    for &diff in left.keys() {
        if right.contains_key(&(-diff)) {
            ans = ans.max(left[&diff] + right[&(-diff)]);
        }
    }

    ans
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
