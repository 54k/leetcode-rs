// https://leetcode.com/problems/detonate-the-maximum-bombs/
pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let mut graph = HashMap::new();

    for i in 0..bombs.len() {
        let entry = graph.entry(i).or_insert(vec![]);
        for j in 0..bombs.len() {
            if i == j {
                continue;
            }
            let (xi, yi, ri) = (bombs[i][0] as i64, bombs[i][1] as i64, bombs[i][2] as i64);
            let (xj, yj) = (bombs[j][0] as i64, bombs[j][1] as i64);

            if ri * ri >= (xi - xj) * (xi - xj) + (yi - yj) * (yi - yj) {
                entry.push(j);
            }
        }
    }

    fn dfs(v: usize, graph: &HashMap<usize, Vec<usize>>, visited: &mut [bool]) -> i32 {
        let mut cnt = 0;
        let mut stack = vec![v];
        visited[v] = true;
        while let Some(v) = stack.pop() {
            cnt += 1;
            for &u in &graph[&v] {
                if !visited[u] {
                    visited[u] = true;
                    stack.push(u)
                }
            }
        }
        cnt
    }

    let mut ans = 0;
    for i in 0..bombs.len() {
        ans = ans.max(dfs(i, &graph, &mut vec![false; bombs.len()]));
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
