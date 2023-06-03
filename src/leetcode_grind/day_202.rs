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