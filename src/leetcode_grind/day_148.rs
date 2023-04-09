// https://leetcode.com/problems/largest-color-value-in-a-directed-graph/description/
// https://leetcode.com/problems/largest-color-value-in-a-directed-graph/editorial/
pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
    fn using_dfs_topo_sort(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        fn dfs(
            v: usize,
            adj: &[Vec<usize>],
            visited: &mut [i32],
            count: &mut [Vec<i32>],
            colors: &[char],
        ) -> i32 {
            if visited[v] == 2 {
                return count[v][colors[v] as usize - 'a' as usize];
            }
            if visited[v] == 1 {
                return i32::MAX;
            }
            visited[v] = 1;
            for &u in &adj[v] {
                if dfs(u, adj, visited, count, colors) == i32::MAX {
                    return i32::MAX;
                }
                for i in 0..26 {
                    count[v][i] = count[v][i].max(count[u][i]);
                }
            }
            count[v][colors[v] as usize - 'a' as usize] += 1;
            visited[v] = 2;
            count[v][colors[v] as usize - 'a' as usize]
        }
        let mut adj = vec![vec![]; colors.len()];
        let colors = colors.chars().collect::<Vec<_>>();
        for e in edges {
            adj[e[0] as usize].push(e[1] as usize);
        }
        let mut count = vec![vec![0; 26]; colors.len()];
        let mut visited = vec![0; colors.len()];
        let mut ans = 0;
        for i in 0..colors.len() {
            ans = ans.max(dfs(i, &adj, &mut visited, &mut count, &colors));
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
    fn using_khan_topo_sort(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let mut adj = vec![vec![]; colors.len()];
        let mut in_degree = vec![0; colors.len()];
        let colors = colors.chars().collect::<Vec<_>>();
        for e in edges {
            adj[e[0] as usize].push(e[1] as usize);
            in_degree[e[1] as usize] += 1;
        }
        let mut count = vec![vec![0; 26]; colors.len()];
        let mut visited = 0;
        let mut ans = 0;
        let mut queue = VecDeque::new();
        for node in 0..in_degree.len() {
            if in_degree[node] == 0 {
                queue.push_back(node);
            }
        }
        while let Some(v) = queue.pop_front() {
            visited += 1;
            count[v][colors[v] as usize - 'a' as usize] += 1;
            ans = ans.max(count[v][colors[v] as usize - 'a' as usize]);
            for &u in &adj[v] {
                for i in 0..26usize {
                    count[u][i] = count[u][i].max(count[v][i]);
                }
                in_degree[u] -= 1;
                if in_degree[u] == 0 {
                    queue.push_back(u);
                }
            }
        }
        if visited == colors.len() {
            ans
        } else {
            -1
        }
    }
    using_dfs_topo_sort(colors, edges)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test418() {
        println!(
            "{}",
            largest_path_value(
                "abaca".to_string(),
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]]
            )
        ); // 3
        println!("{}", largest_path_value("a".to_string(), vec![vec![0, 0]])); // -1
    }
}
