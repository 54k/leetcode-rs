// https://leetcode.com/problems/number-of-ways-to-select-buildings/description
pub fn number_of_ways(s: String) -> i64 {
    let s = s.as_bytes();
    let mut ans = 0i64;

    let mut total_zeros = 0i64;
    for i in 0..s.len() {
        total_zeros += if s[i] == b'0' { 1 } else { 0 };
    }
    let total_ones = s.len() as i64 - total_zeros;

    let mut curr_zeros = if s[0] == b'0' { 1 } else { 0 };
    let mut curr_ones = if s[0] == b'1' { 1 } else { 0 };

    for i in 1..s.len() {
        if s[i] == b'1' {
            ans += (curr_zeros * (total_zeros - curr_zeros));
            curr_ones += 1;
        }
        if s[i] == b'0' {
            ans += (curr_ones * (total_ones - curr_ones));
            curr_zeros += 1;
        }
    }

    ans
}

// https://leetcode.com/problems/restore-the-array-from-adjacent-pairs/description
pub fn restore_array_dfs(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut adj = HashMap::new();
    for edge in adjacent_pairs {
        adj.entry(edge[0]).or_insert(vec![]).push(edge[1]);
        adj.entry(edge[1]).or_insert(vec![]).push(edge[0]);
    }

    let mut root = 0;
    for (k, v) in &adj {
        if v.len() == 1 {
            root = *k;
            break;
        }
    }
    let mut ans = vec![];
    fn dfs(node: i32, prev: i32, ans: &mut Vec<i32>, adj: &HashMap<i32, Vec<i32>>) {
        ans.push(node);
        for &next in &adj[&node] {
            if next != prev {
                dfs(next, node, ans, adj);
            }
        }
    }

    dfs(root, -1, &mut ans, &adj);
    ans
}

pub fn restore_array_iterative(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut adj = HashMap::new();
    for edge in adjacent_pairs {
        adj.entry(edge[0]).or_insert(vec![]).push(edge[1]);
        adj.entry(edge[1]).or_insert(vec![]).push(edge[0]);
    }

    let mut root = 0;
    for (k, v) in &adj {
        if v.len() == 1 {
            root = *k;
            break;
        }
    }

    let mut ans = vec![0; adj.len()];
    let mut curr = root;
    ans[0] = root;
    let mut i = 1;
    let mut prev = i32::MAX;

    while i < adj.len() {
        for &neighbor in &adj[&curr] {
            if neighbor != prev {
                ans[i] = neighbor;
                i += 1;
                prev = curr;
                curr = neighbor;
                break;
            }
        }
    }

    ans
}
