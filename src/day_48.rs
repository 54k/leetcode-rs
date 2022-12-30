#[allow(dead_code)]
pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    fn dfs(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(
            e: i32,
            adj: &Vec<Vec<i32>>,
            res: &mut Vec<Vec<i32>>,
            vis: &mut Vec<bool>,
            path: &mut Vec<i32>,
        ) {
            if vis[e as usize] {
                return;
            }
            vis[e as usize] = true;
            path.push(e);
            for v in adj[e as usize].iter() {
                dfs(*v, adj, res, vis, path);
            }
            if e as usize == adj.len() - 1 {
                res.push(path.clone());
            }
            path.pop();
            vis[e as usize] = false;
        }

        if graph.is_empty() {
            return vec![];
        }
        let mut res = vec![];
        let mut vis = vec![false; graph.len()];
        dfs(0, &graph, &mut res, &mut vis, &mut vec![]);
        res
    }

    fn bfs(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        if graph.is_empty() {
            return vec![];
        }
        let mut res = vec![];
        let mut vis = vec![false; graph.len()];
        let mut q = VecDeque::new();
        q.push_back((0, vec![]));
        while !q.is_empty() {
            let (e, mut path) = q.pop_front().unwrap();
            vis[e as usize] = true;
            path.push(e);
            if e as usize == graph.len() - 1 {
                res.push(path.clone());
            }
            for v in graph[e as usize].iter() {
                if !vis[*v as usize] {
                    q.push_back((*v, path.clone()));
                }
            }
            vis[e as usize] = false;
        }
        res
    }

    bfs(graph)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test135() {
        println!("{:?}", all_paths_source_target(vec![])); // []
        println!(
            "{:?}",
            all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]])
        ); // [[0, 1, 3], [0, 2, 3]]
        println!(
            "{:?}",
            all_paths_source_target(vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]])
        ); // [[0, 4], [0, 3, 4], [0, 1, 3, 4], [0, 1, 2, 3, 4], [0, 1, 4]]
    }
}
