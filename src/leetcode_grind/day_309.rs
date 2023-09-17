pub fn shortest_path_length_i(graph: Vec<Vec<i32>>) -> i32 {
    let n = graph.len();
    let ending_mask = (1 << n) - 1;
    let mut cache = vec![vec![0; ending_mask + 1]; n + 1];

    fn dp(node: usize, mask: usize, cache: &mut Vec<Vec<i32>>, graph: &Vec<Vec<i32>>) -> i32 {
        if cache[node][mask] != 0 {
            return cache[node][mask];
        }

        if (mask & (mask - 1)) == 0 {
            return 0;
        }

        cache[node][mask] = i32::MAX - 1;
        for &neighbor in &graph[node] {
            let neighbor = neighbor as usize;
            if (mask & (1 << neighbor)) != 0 {
                let already_visited = dp(neighbor, mask, cache, graph);
                let not_visited = dp(neighbor, mask ^ (1 << node), cache, graph);
                let better_option = already_visited.min(not_visited);
                cache[node][mask] = cache[node][mask].min(better_option + 1);
            }
        }
        cache[node][mask]
    }

    let mut best = i32::MAX;
    for node in 0..n {
        best = best.min(dp(node, ending_mask, &mut cache, &graph));
    }

    best
}

pub fn shortest_path_length_ii(graph: Vec<Vec<i32>>) -> i32 {
    if graph.len() == 1 {
        return 0;
    }

    let n = graph.len();
    let ending_mask = (1 << n) - 1;
    let mut seen = vec![vec![false; ending_mask]; n];
    let mut queue = vec![];

    for i in 0..n {
        queue.push((i, 1 << i));
        seen[i][1 << i] = true;
    }

    let mut steps = 0;
    while queue.len() > 0 {
        let mut next_queue = vec![];

        for (node, mask) in queue {
            for &neighbor in &graph[node] {
                let neighbor = neighbor as usize;
                let next_mask = mask | (1 << neighbor);
                if next_mask == ending_mask {
                    return 1 + steps;
                }

                if !seen[neighbor][next_mask] {
                    seen[neighbor][next_mask] = true;
                    next_queue.push((neighbor, next_mask));
                }
            }
        }
        steps += 1;
        queue = next_queue;
    }

    -1
}

pub fn shortest_path_length_iii(graph: Vec<Vec<i32>>) -> i32 {
    let n = graph.len();
    let mut dist_matrix = vec![vec![n as i32 + 1; n]; n];

    for node in 0..n {
        dist_matrix[node][node] = 0;
        for &neighbor in &graph[node] {
            let neighbor = neighbor as usize;
            dist_matrix[node][neighbor] = 1;
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist_matrix[i][j] = dist_matrix[i][j].min(dist_matrix[i][k] + dist_matrix[k][j]);
            }
        }
    }

    println!("{:?}", dist_matrix);

    let end_mask = (1 << n) - 1;
    let mut memo = vec![vec![-1; end_mask + 1]; n];

    fn solve(
        node: usize,
        mask: usize,
        end_mask: usize,
        memo: &mut Vec<Vec<i32>>,
        graph: &Vec<Vec<i32>>,
        dist_matrix: &Vec<Vec<i32>>,
    ) -> i32 {
        if mask == end_mask {
            return 0;
        }

        if memo[node][mask] != -1 {
            return memo[node][mask];
        }

        let mut shortest_path = 10e5 as i32;

        for neighbor in 0..graph.len() {
            if (mask & (1 << neighbor)) != 0 {
                continue;
            }

            shortest_path = shortest_path.min(
                solve(
                    neighbor,
                    mask | (1 << neighbor),
                    end_mask,
                    memo,
                    graph,
                    dist_matrix,
                ) + dist_matrix[node as usize][neighbor],
            );
        }

        memo[node][mask] = shortest_path;
        shortest_path
    }

    let mut ans = i32::MAX;
    for start in 0..n {
        ans = ans.min(solve(
            start,
            1 << start,
            end_mask,
            &mut memo,
            &graph,
            &dist_matrix,
        ));
    }

    ans
}

#[test]
fn test_shortest_path_length() {
    let res = shortest_path_length_iii(vec![vec![1, 2, 3], vec![0], vec![0], vec![0]]);
    println!("{res}");
}
