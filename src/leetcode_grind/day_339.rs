// https://leetcode.com/problems/validate-binary-tree-nodes/description
pub fn validate_binary_tree_nodes_dfs(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
    use std::collections::HashSet;
    fn find_root(n: i32, left_child: &Vec<i32>, right_child: &Vec<i32>) -> i32 {
        let mut children = HashSet::new();
        for &left in left_child.iter() {
            children.insert(left);
        }
        for &right in right_child.iter() {
            children.insert(right);
        }
        for i in 0..n {
            if !children.contains(&i) {
                return i;
            }
        }
        -1
    }

    let root = find_root(n, &left_child, &right_child);
    if root == -1 {
        return false;
    }

    let mut seen = HashSet::new();
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        if seen.contains(&node) {
            return false;
        }
        seen.insert(node);
        let right = right_child[node as usize];
        if right > -1 {
            stack.push(right);
        }
        let left = left_child[node as usize];
        if left > -1 {
            stack.push(left);
        }
    }

    seen.len() == n as usize
}

pub fn validate_binary_tree_nodes_bfs(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
    use std::collections::HashSet;
    use std::collections::VecDeque;

    fn find_root(n: i32, left_child: &Vec<i32>, right_child: &Vec<i32>) -> i32 {
        let mut children = HashSet::new();
        children.extend(left_child.iter().copied());
        children.extend(right_child.iter().copied());
        for ref i in 0..n {
            if !children.contains(i) {
                return *i;
            }
        }
        -1
    }
    let root = find_root(n, &left_child, &right_child);
    if root == -1 {
        return false;
    }

    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while let Some(node) = queue.pop_front() {
        if seen.contains(&node) {
            return false;
        }
        seen.insert(node);
        let children = vec![left_child[node as usize], right_child[node as usize]];
        for ch in children {
            if ch != -1 {
                queue.push_back(ch);
            }
        }
    }
    seen.len() == n as usize
}

pub fn validate_binary_tree_nodes_dsu(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
    struct UF {
        repr: Vec<usize>,
        cmp: usize,
    }
    impl UF {
        fn new(n: usize) -> Self {
            let mut repr = vec![];
            for i in 0..n {
                repr.push(i);
            }
            Self { repr, cmp: n }
        }
        fn find(&mut self, x: usize) -> usize {
            if self.repr[x] != x {
                self.repr[x] = self.find(self.repr[x]);
            }
            self.repr[x]
        }
        fn union(&mut self, x: usize, y: usize) -> bool {
            let mut px = self.find(x);
            let mut py = self.find(y);
            if py != y || px == py {
                return false;
            }
            self.repr[py] = px;
            self.cmp -= 1;
            true
        }
    }

    let mut uf = UF::new(n as usize);
    for parent in 0..n as usize {
        let children = vec![left_child[parent], right_child[parent]];
        for ch in children {
            if ch == -1 {
                continue;
            }
            if !uf.union(parent, ch as usize) {
                return false;
            }
        }
    }

    uf.cmp == 1
}
