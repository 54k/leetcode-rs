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

// https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/description/
pub fn leads_to_destination(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut graph = vec![vec![]; n as usize];
    for edge in edges {
        graph[edge[0] as usize].push(edge[1] as usize);
    }
    let mut states = vec![0; n as usize];

    fn lead_to_dest(
        graph: &Vec<Vec<usize>>,
        node: usize,
        dest: usize,
        states: &mut Vec<i32>,
    ) -> bool {
        if states[node] != 0 {
            return states[node] == 2;
        }

        if graph[node].is_empty() {
            return node == dest;
        }

        states[node] = 1;

        for &next in &graph[node] {
            if !lead_to_dest(graph, next, dest, states) {
                return false;
            }
        }

        states[node] = 2;
        return true;
    }

    lead_to_dest(&graph, source as usize, destination as usize, &mut states)
}
