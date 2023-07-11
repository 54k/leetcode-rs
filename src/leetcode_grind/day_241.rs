use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/description/
pub fn distance_k(
    root: Option<Rc<RefCell<TreeNode>>>,
    target: Option<Rc<RefCell<TreeNode>>>,
    k: i32,
) -> Vec<i32> {
    type Node = Option<Rc<RefCell<TreeNode>>>;

    use std::collections::{HashMap, HashSet};

    fn build_graph(root: Node, parent: Node, graph: &mut HashMap<i32, Vec<i32>>) {
        if root.is_some() {
            let r = root.as_ref().unwrap().borrow();

            if parent.is_some() {
                let p = parent.as_ref().unwrap().borrow();
                graph.entry(r.val).or_insert(vec![]).push(p.val);
                graph.entry(p.val).or_insert(vec![]).push(r.val);
            }

            build_graph(r.left.clone(), root.clone(), graph);
            build_graph(r.right.clone(), root.clone(), graph);
        }
    }

    fn dfs(
        cur: i32,
        distance: i32,
        k: i32,
        ans: &mut Vec<i32>,
        graph: &HashMap<i32, Vec<i32>>,
        visited: &mut HashSet<i32>,
    ) {
        if distance == k {
            ans.push(cur);
            return;
        }

        for &n in graph.get(&cur).unwrap_or(&vec![]) {
            if !visited.contains(&n) {
                visited.insert(n);
                dfs(n, distance + 1, k, ans, graph, visited);
            }
        }
    }

    let mut graph = HashMap::new();
    build_graph(root.clone(), None, &mut graph);

    let mut ans = vec![];

    let mut visited = &mut HashSet::new();
    visited.insert(target.clone().unwrap().borrow().val);

    dfs(
        target.clone().unwrap().borrow().val,
        0,
        k,
        &mut ans,
        &mut graph,
        &mut visited,
    );

    ans
}
