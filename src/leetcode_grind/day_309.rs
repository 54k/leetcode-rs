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
