// https://leetcode.com/problems/path-with-maximum-probability/description/
pub fn max_probability_bellman_ford(
    n: i32,
    edges: Vec<Vec<i32>>,
    succ_prob: Vec<f64>,
    start: i32,
    end: i32,
) -> f64 {
    let mut max_prob = vec![0.0; n as usize];
    max_prob[start as usize] = 1.0;

    for _ in 0..n - 1 {
        let mut has_update = false;
        for j in 0..edges.len() {
            let (u, v) = (edges[j][0] as usize, edges[j][1] as usize);
            let path_prob = succ_prob[j];

            if max_prob[u] * path_prob > max_prob[v] {
                max_prob[v] = max_prob[u] * path_prob;
                has_update = true;
            }

            if max_prob[v] * path_prob > max_prob[u] {
                max_prob[u] = max_prob[v] * path_prob;
                has_update = true;
            }
        }

        if !has_update {
            break;
        }
    }

    max_prob[end as usize]
}

pub fn max_probability_spfa(
    n: i32,
    edges: Vec<Vec<i32>>,
    succ_prob: Vec<f64>,
    start: i32,
    end: i32,
) -> f64 {
    use std::collections::VecDeque;

    let mut graph = vec![vec![]; n as usize];
    for i in 0..edges.len() {
        let (u, v, p) = (edges[i][0] as usize, edges[i][1] as usize, succ_prob[i]);
        graph[u].push((v, p));
        graph[v].push((u, p));
    }

    let mut max_prob = vec![0.0; n as usize];
    max_prob[start as usize] = 1.0;
    let mut queue = VecDeque::new();
    queue.push_back(start as usize);

    while let Some(node) = queue.pop_front() {
        for neighbor in &graph[node] {
            let (next, prob) = (neighbor.0, neighbor.1);

            if max_prob[node] * prob > max_prob[next] {
                max_prob[next] = max_prob[node] * prob;
                queue.push_back(next);
            }
        }
    }
    max_prob[end as usize]
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
