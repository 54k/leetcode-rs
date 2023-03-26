// https://leetcode.com/problems/longest-cycle-in-a-graph/
// https://leetcode.com/problems/longest-cycle-in-a-graph/editorial/
pub fn longest_cycle(edges: Vec<i32>) -> i32 {
    fn using_dfs(edges: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        fn dfs(
            node: i32,
            edges: &Vec<i32>,
            dist: &mut HashMap<i32, i32>,
            visit: &mut Vec<bool>,
            ans: &mut i32,
        ) {
            visit[node as usize] = true;
            let neighbor = edges[node as usize];

            if neighbor != -1 && !visit[neighbor as usize] {
                dist.insert(neighbor, dist[&node] + 1);
                dfs(neighbor, edges, dist, visit, ans);
            } else if neighbor != -1 && dist.contains_key(&neighbor) {
                *ans = (*ans).max(dist[&node] - dist[&neighbor] + 1);
            }
        }

        let mut ans = 0;
        let n = edges.len();
        let mut visit = vec![false; n];
        for i in 0..n {
            if !visit[i] {
                let mut dist = HashMap::new();
                dist.insert(i as i32, 1);
                dfs(i as i32, &edges, &mut dist, &mut visit, &mut ans);
            }
        }
        ans
    }
    fn using_khan(edges: Vec<i32>) -> i32 {
        use std::collections::VecDeque;
        let mut visit = vec![false; edges.len()];
        let mut indegree = vec![0; edges.len()];
        let mut queue = VecDeque::new();

        for &e in &edges {
            if e != -1 {
                indegree[e as usize] += 1;
            }
        }
        for i in 0..indegree.len() {
            if indegree[i] == 0 {
                queue.push_back(i);
            }
        }
        while let Some(n) = queue.pop_front() {
            visit[n] = true;
            let neighbor = edges[n];
            if neighbor != -1 {
                let id = &mut indegree[neighbor as usize];
                *id -= 1;
                if *id == 0 {
                    queue.push_back(neighbor as usize);
                }
            }
        }

        let mut ans = -1;
        for i in 0..edges.len() {
            if !visit[i] {
                let mut neighbor = edges[i] as usize;
                visit[i] = true;
                let mut count = 1;
                while neighbor != i {
                    visit[neighbor] = true;
                    count += 1;
                    neighbor = edges[neighbor] as usize;
                }
                ans = ans.max(count);
            }
        }
        ans
    }
    using_khan(edges)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test376() {
        println!("{}", longest_cycle(vec![3, 3, 4, 2, 3])); // 3
    }
}
