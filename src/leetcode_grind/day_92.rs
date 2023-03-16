use std::cell::RefCell;
use std::rc::Rc;

pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
    use std::collections::VecDeque;
    let n = roads.len() + 1;
    let mut adj = vec![vec![]; n];
    let mut degrees = vec![0; n];
    for from_to in roads {
        let from = from_to[0] as usize;
        let to = from_to[1] as usize;
        adj[from].push(to);
        adj[to].push(from);
        degrees[to] += 1;
        degrees[from] += 1;
    }
    let mut representatives = vec![1; n];
    let mut fuel = 0;
    let mut queue = VecDeque::new();
    for i in 1..n {
        if degrees[i] == 1 {
            queue.push_back(i);
        }
    }
    while let Some(v) = queue.pop_front() {
        // Add the amount of fuel needed to transport all of the representatives from node to its parent,
        // fuel += ceil(representatives[node] / seats).
        fuel += (representatives[v] as f64 / seats as f64).ceil() as i64;
        for &u in &adj[v] {
            degrees[u] -= 1;
            representatives[u] += representatives[v];
            if degrees[u] == 1 && u != 0
            /*not a root node*/
            {
                queue.push_back(u);
            }
        }
    }
    fuel
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() || root == p || root == q {
        return root;
    }
    let r = root.clone().unwrap();
    let r = r.borrow();
    let left = lowest_common_ancestor(r.left.clone(), p.clone(), q.clone());
    let right = lowest_common_ancestor(r.right.clone(), p, q);
    if left.is_some() && right.is_some() {
        return root;
    }
    if left.is_some() {
        left
    } else {
        right
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test221() {
        println!(
            "{}",
            minimum_fuel_cost(vec![vec![0, 1], vec![0, 2], vec![0, 3]], 5)
        ); // 3
        println!("{}", minimum_fuel_cost(vec![vec![0, 1], vec![1, 2]], 3)); // 2
        println!(
            "{}",
            minimum_fuel_cost(
                vec![
                    vec![3, 1],
                    vec![3, 2],
                    vec![1, 0],
                    vec![0, 4],
                    vec![0, 5],
                    vec![4, 6]
                ],
                2
            )
        ); // 7
    }
}
