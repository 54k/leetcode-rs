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
// class Solution {
// public List<List<Integer>> levelOrder(Node root) {
// return dfs(root, null, 0, new ArrayList<>());
// }
//
// List<List<Integer>> dfs(Node v, Node p, int depth, List<List<Integer>> res) {
// if (res.size() == depth) {
// res.add(new ArrayList<>());
// }
// for (var ch : v.children) {
// if (ch != p) {
// dfs(ch, v, depth + 1, res);
// }
// }
// if (depth == 0) {
// return res;
// }
// res.get(depth).add(v.val);
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

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    use std::collections::HashMap;
    let mut cache = HashMap::new();
    fn gen(
        first: i32,
        last: i32,
        cache: &mut HashMap<(i32, i32), Vec<Option<Rc<RefCell<TreeNode>>>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if cache.contains_key(&(first, last)) {
            return cache.get(&(first, last)).unwrap().clone();
        }
        if first >= last {
            return if first == last {
                vec![Some(Rc::new(RefCell::new(TreeNode {
                    val: first,
                    left: None,
                    right: None,
                })))]
            } else {
                vec![None]
            };
        }

        let mut trees = vec![];
        for root in first..=last {
            let left_tree = gen(first, root - 1, cache);
            let right_tree = gen(root + 1, last, cache);
            for left in &left_tree {
                for right in &right_tree {
                    let tree = TreeNode {
                        val: root,
                        left: left.clone(),
                        right: right.clone(),
                    };
                    trees.push(Some(Rc::new(RefCell::new(tree))));
                }
            }
        }
        if trees.is_empty() {
            vec![None]
        } else {
            cache.entry((first, last)).or_insert_with(|| trees.clone());
            trees
        }
    }

    gen(1, n, &mut cache)
}

// https://leetcode.com/problems/unique-binary-search-trees/solutions/31666/dp-solution-in-6-lines-with-explanation-f-i-n-g-i-1-g-n-i/?orderBy=most_relevant
pub fn num_trees(n: i32) -> i32 {
    let mut g = vec![0; n as usize + 1];
    g[0] = 1;
    g[1] = 1;
    for i in 2..=n as usize {
        for j in 1..=i {
            g[i] += g[j - 1] * g[i - j];
        }
    }
    g[n as usize]
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

    #[test]
    fn test151() {
        let trees = generate_trees(5);
        println!("trees {:?}", trees.len()); // 5
        println!("{:?}", trees);
    }

    #[test]
    fn test152() {
        println!("{}", num_trees(5)); // 5
    }
}
