// https://leetcode.com/problems/snapshot-array/description/
use std::collections::BTreeMap;

struct SnapshotArray {
    snaps: Vec<BTreeMap<i32, i32>>,
    id: i32,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        let mut snaps = vec![BTreeMap::new(); length as usize];
        for m in &mut snaps {
            m.insert(0, 0);
        }
        Self { snaps, id: 0 }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.snaps[index as usize].insert(self.id, val);
    }

    fn snap(&mut self) -> i32 {
        self.id += 1;
        self.id - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        *self.snaps[index as usize]
            .range(0..=snap_id)
            .last()
            .unwrap()
            .1
    }
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
