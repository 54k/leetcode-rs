// https://leetcode.com/problems/path-with-maximum-probability/description/?envType=daily-question&envId=2024-08-31
pub fn max_probability(
    n: i32,
    edges: Vec<Vec<i32>>,
    succ_prob: Vec<f64>,
    start_node: i32,
    end_node: i32,
) -> f64 {
    let mut path_prob = vec![0.0; n as usize];
    path_prob[start_node as usize] = 1.0;

    for _ in 0..n {
        let mut has_update = false;

        for (i, e) in edges.iter().enumerate() {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let p = succ_prob[i];

            if path_prob[u] < path_prob[v] * p {
                path_prob[u] = path_prob[v] * p;
                has_update = true;
            }
            if path_prob[v] < path_prob[u] * p {
                path_prob[v] = path_prob[u] * p;
                has_update = true;
            }
        }

        if !has_update {
            break;
        }
    }

    path_prob[end_node as usize]
}

pub fn max_probability_ii(
    n: i32,
    edges: Vec<Vec<i32>>,
    succ_prob: Vec<f64>,
    start_node: i32,
    end_node: i32,
) -> f64 {
    use std::collections::HashMap;
    use std::collections::VecDeque;

    let mut adj = HashMap::new();
    for (i, e) in edges.iter().enumerate() {
        let u = e[0];
        let v = e[1];
        let p = succ_prob[i];
        adj.entry(u).or_insert(vec![]).push((v, p));
        adj.entry(v).or_insert(vec![]).push((u, p));
    }

    let mut max_prob = vec![0.0; n as usize];
    max_prob[start_node as usize] = 1.0;

    let mut q = VecDeque::new();
    q.push_back(start_node);

    while !q.is_empty() {
        let u = q.pop_front().unwrap();
        if !adj.contains_key(&u) {
            continue;
        }
        for nei in &adj[&u] {
            let v = nei.0;
            let p = nei.1;
            if max_prob[u as usize] * p > max_prob[v as usize] {
                max_prob[v as usize] = max_prob[u as usize] * p;
                q.push_back(v);
            }
        }
    }

    max_prob[end_node as usize]
}
