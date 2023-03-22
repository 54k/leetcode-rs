// https://leetcode.com/problems/minimum-score-of-a-path-between-two-cities/editorial/
pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    fn create_adj(n: i32, roads: &Vec<Vec<i32>>) -> (Vec<Vec<(usize, i32)>>, Vec<bool>) {
        let mut adj = vec![vec![]; n as usize + 1];
        let visited = vec![false; n as usize + 1];
        for road in roads {
            let from = road[0] as usize;
            let to = road[1] as usize;
            let weight = road[2];
            adj[from].push((to, weight));
            adj[to].push((from, weight));
        }
        (adj, visited)
    }
    fn using_bfs(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let (adj, mut visited) = create_adj(n, &roads);
        let mut queue = VecDeque::new();
        let mut ans = i32::MAX;

        queue.push_back(1usize);
        visited[1] = true;

        while let Some(v) = queue.pop_front() {
            if adj[v].is_empty() {
                continue;
            }

            for (u, w) in &adj[v] {
                ans = ans.min(*w);
                if !visited[*u] {
                    visited[*u] = true;
                    queue.push_back(*u);
                }
            }
        }
        ans
    }
    fn using_dfs(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let (adj, mut visited) = create_adj(n, &roads);
        fn dfs(v: usize, adj: &Vec<Vec<(usize, i32)>>, visited: &mut Vec<bool>, ans: &mut i32) {
            visited[v] = true;
            for (u, w) in &adj[v] {
                *ans = (*ans).min(*w);
                if !visited[*u] {
                    dfs(*u, adj, visited, ans);
                }
            }
        }
        let mut ans = i32::MAX;
        dfs(1, &adj, &mut visited, &mut ans);
        ans
    }
    using_dfs(n, roads)
}

// https://leetcode.com/problems/divide-array-in-sets-of-k-consecutive-numbers/description/
pub fn is_possible_divide(nums: Vec<i32>, k: i32) -> bool {
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test362() {
        println!(
            "{}",
            min_score(
                4,
                vec![vec![1, 2, 9], vec![2, 3, 6], vec![2, 4, 5], vec![1, 4, 7]]
            )
        ); // 5
    }

    #[test]
    fn test_is_possible_divide() {
        println!("{}", is_possible_divide(vec![1, 2, 3, 3, 4, 4, 5, 6], 4));
        println!(
            "{}",
            is_possible_divide(vec![3, 2, 1, 2, 3, 4, 3, 4, 5, 9, 10, 11], 3)
        );
    }
}
