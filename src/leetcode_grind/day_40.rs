// https://leetcode.com/problems/sum-of-distances-in-tree/solutions/130611/sum-of-distances-in-tree/?orderBy=most_relevant
#[allow(dead_code)]
pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    fn dfs1(
        node: usize,
        parent: usize,
        adj: &Vec<Vec<usize>>,
        count: &mut Vec<i32>,
        ans: &mut Vec<i32>,
    ) {
        for child in &adj[node] {
            if *child != parent {
                dfs1(*child, node, adj, count, ans);
                count[node] += count[*child];
                ans[node] += ans[*child] + count[*child];
            }
        }
    }

    fn dfs2(
        node: usize,
        parent: usize,
        n: i32,
        adj: &Vec<Vec<usize>>,
        count: &Vec<i32>,
        ans: &mut Vec<i32>,
    ) {
        for child in &adj[node] {
            if *child != parent {
                ans[*child] = ans[node] - count[*child] + n - count[*child];
                dfs2(*child, node, n, adj, count, ans);
            }
        }
    }

    let mut ans = vec![0; n as usize];
    let mut count = vec![1; n as usize];
    let mut adj = vec![vec![]; n as usize];

    for e in &edges {
        adj[e[0] as usize].push(e[1] as usize);
        adj[e[1] as usize].push(e[0] as usize);
    }

    dfs1(0, n as usize + 1, &adj, &mut count, &mut ans);
    dfs2(0, n as usize + 1, n, &adj, &count, &mut ans);
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test124() {
        println!(
            "{:?}",
            sum_of_distances_in_tree(
                6,
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]]
            )
        ); // [8,12,6,10,10,10]
    }
}
