// https://leetcode.com/problems/shortest-path-to-get-all-keys/
pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
    use std::collections::{HashMap, HashSet, VecDeque};

    let grid = grid
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    const DIR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let (m, n) = (grid.len(), grid[0].len());

    let mut key_set = HashSet::new();
    let mut lock_set = HashSet::new();

    let mut all_keys = 0;
    let (mut start_row, mut start_col) = (-1, -1);

    for i in 0..m {
        for j in 0..n {
            let cell = grid[i][j];

            if cell >= 'a' && cell <= 'f' {
                all_keys += (1 << (cell as i32 - 'a' as i32));
                key_set.insert(cell);
            }

            if cell >= 'A' && cell <= 'F' {
                lock_set.insert(cell);
            }

            if cell == '@' {
                start_row = i as i32;
                start_col = j as i32;
            }
        }
    }

    let mut seen: HashMap<i32, HashSet<(i32, i32)>> = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back((start_row, start_col, 0, 0)); // [row, col, key_state, distance]
    seen.insert(0, HashSet::new());
    seen.get_mut(&0).unwrap().insert((start_row, start_col));

    while let Some((cur_r, cur_c, keys, dist)) = queue.pop_front() {
        for d in &DIR {
            let (new_r, new_c) = (cur_r + d.0, cur_c + d.1);

            if 0 <= new_r
                && new_r < m as i32
                && 0 <= new_c
                && new_c < n as i32
                && grid[new_r as usize][new_c as usize] != '#'
            {
                let cell = grid[new_r as usize][new_c as usize];
                // if it is a key
                if key_set.contains(&cell) {
                    // If we have collected it before, no need to revisit this cell.
                    if (1 << (cell as i32 - 'a' as i32)) & keys != 0 {
                        continue;
                    }
                    // Otherwise, we can walk to this cell and pick it up.
                    let new_keys = keys | (1 << (cell as i32 - 'a' as i32));

                    // If we collect all keys, return dist + 1.
                    // Otherwise, just add this state to seen and queue.
                    if new_keys == all_keys {
                        return dist + 1;
                    }
                    seen.entry(new_keys).or_insert(HashSet::new());
                    seen.get_mut(&new_keys).unwrap().insert((new_r, new_c));
                    queue.push_back((new_r, new_c, new_keys, dist + 1));
                }

                // If it is a lock and we don't have its key, continue.
                if lock_set.contains(&cell) && (keys & (1 << (cell as i32 - 'A' as i32))) == 0 {
                    continue;
                }
                // We can walk to this cell if we haven't been here before with the same key state.
                else if !seen.get(&keys).unwrap().contains(&(new_r, new_c)) {
                    seen.get_mut(&keys).unwrap().insert((new_r, new_c));
                    queue.push_back((new_r, new_c, keys, dist + 1));
                }
            }
        }
    }

    -1
}

// https://leetcode.com/problems/minimum-cost-to-merge-stones/description/
// https://leetcode.com/problems/minimum-cost-to-merge-stones/solutions/247567/java-c-python-dp/
pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
    let n = stones.len();
    if (n as i32 - 1) % (k - 1) > 0 {
        return -1;
    }

    let mut prefix = vec![0; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + stones[i];
    }

    let mut dp = vec![vec![0; n]; n];

    for m in k..=n as i32 {
        for i in 0..=n as i32 - m {
            let j = i + m - 1;
            dp[i as usize][j as usize] = i32::MAX;
            let mut mid = i;
            while mid < j {
                dp[i as usize][j as usize] = dp[i as usize][j as usize]
                    .min(dp[i as usize][mid as usize] + dp[mid as usize + 1][j as usize]);
                mid += k - 1;
            }

            if (j - i) % (k - 1) == 0 {
                dp[i as usize][j as usize] += prefix[j as usize + 1] - prefix[i as usize];
            }
        }
    }

    dp[0][n - 1]
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
