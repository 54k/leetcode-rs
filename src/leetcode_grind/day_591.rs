// https://leetcode.com/problems/all-ancestors-of-a-node-in-a-directed-acyclic-graph/description/?envType=daily-question&envId=2024-06-29
pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ans = vec![vec![]; n as usize];
    let mut adj = vec![vec![]; n as usize];
    for e in edges {
        adj[e[0] as usize].push(e[1] as usize);
    }
    fn dfs(v: usize, p: usize, adj: &Vec<Vec<usize>>, ans: &mut Vec<Vec<i32>>) {
        for &u in &adj[v] {
            if ans[u].len() == 0 || *ans[u].last().unwrap() != p as i32 {
                ans[u].push(p as i32);
                dfs(u, p, adj, ans);
            }
        }
    }

    for i in 0..n as usize {
        dfs(i, i, &adj, &mut ans);
    }
    ans
}


