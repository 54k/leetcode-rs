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
