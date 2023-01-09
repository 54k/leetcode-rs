// import java.util.ArrayDeque;
// import java.util.ArrayList;
// import java.util.Collections;
// import java.util.List;
//
// // Definition for a Node.
// class Node {
// public int val;
// public List<Node> children;
//
// public Node() {}
//
// public Node(int _val) {
// val = _val;
// }
//
// public Node(int _val, List<Node> _children) {
// val = _val;
// children = _children;
// }
// };
//
//
// class Solution {
// public List<List<Integer>> levelOrder(Node root) {
// if (root == null) {
// return Collections.emptyList();
// }
// var res = new ArrayList<List<Integer>>();
// var q = new ArrayDeque<Node[]>();
// q.push(new Node[] {root, new Node(1)});
// while (!q.isEmpty()) {
// var poll = q.pollFirst();
// if (res.size() < poll[1].val) {
// res.add(new ArrayList<>());
// }
// res.get(res.size() - 1).add(poll[0].val);
// for (var ch : poll[0].children) {
// q.addLast(new Node[] {ch, new Node(poll[1].val + 1)});
// }
// }
// return res;
// }
// }

#[allow(dead_code)]
pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
    fn dfs(
        v: usize,
        p: usize,
        depth: i32,
        seats: i32,
        adj: &Vec<Vec<usize>>,
        res: &mut i64,
    ) -> i64 {
        let mut riders = 1;
        for &u in &adj[v] {
            if u != p {
                riders += dfs(u, v, depth + 1, seats, adj, res);
            }
        }
        if depth == 0 {
            return *res;
        }
        *res += (riders / seats as i64) * depth as i64
            + (if (riders % seats as i64) > 0 { 1 } else { 0 });
        riders % seats as i64
    }
    let mut adj = vec![vec![]; roads.len() + 1];
    for r in roads {
        adj[r[0] as usize].push(r[1] as usize);
        adj[r[1] as usize].push(r[0] as usize);
    }
    dfs(0, 0, 0, seats, &adj, &mut 0)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test150() {
        println!(
            "{}",
            minimum_fuel_cost(
                vec![
                    vec![3, 1],
                    vec![3, 2],
                    vec![1, 0],
                    vec![0, 4],
                    vec![0, 5],
                    vec![4, 6],
                ],
                2
            )
        ); //7
    }
}
