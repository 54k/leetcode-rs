use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;

// 4.1 Для заданного направленного графа реализуйте алгоритм,
// проверяющий существование маршрута между двумя вершинами
fn task_4_1(edges: Vec<(usize, usize)>, start: usize, end: usize) -> bool {
    fn bfs(edges: Vec<(usize, usize)>, start: usize, end: usize) -> bool {
        use std::collections::*;
        let mut adj = HashMap::new();
        let mut queue = VecDeque::new();
        for (from, to) in edges {
            adj.entry(from).or_insert(Vec::new()).push(to);
        }
        let mut visited = vec![false; adj.len()];
        queue.push_back(start);
        visited[start] = true;
        while let Some(v) = queue.pop_front() {
            for &u in adj.get(&v).unwrap() {
                if u == end {
                    return true;
                }
                if !visited[u] {
                    visited[u] = true;
                    queue.push_back(u);
                }
            }
        }
        false
    }

    fn dfs(edges: Vec<(usize, usize)>, start: usize, end: usize) -> bool {
        use std::collections::*;
        fn dfs(
            v: usize,
            end: usize,
            adj: &HashMap<usize, Vec<usize>>,
            visited: &mut Vec<bool>,
        ) -> bool {
            if v == end {
                return true;
            }
            let mut res = false;
            for &u in adj.get(&v).unwrap() {
                if !visited[u] {
                    visited[u] = true;
                    res |= dfs(u, end, adj, visited);
                }
            }
            res
        }
        let mut adj = HashMap::new();
        for (from, to) in edges {
            adj.entry(from).or_insert(Vec::new()).push(to);
        }
        let mut visited = vec![false; adj.len()];
        visited[start] = true;
        dfs(start, end, &adj, &mut visited)
    }
    dfs(edges, start, end)
}

// 4.2 напишите алгоритм создания бинарного дерева с минимальной высотой из отсортированного (по возрастанию)
// массива с уникальными целочисленными элементами
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

fn task_4_2(arr: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn create_minimal_bst(arr: &[i32], left: i32, right: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right {
            return None;
        }
        let mid = (left + right) / 2;
        let mut root = TreeNode {
            val: arr[mid as usize],
            left: None,
            right: None,
        };
        root.left = create_minimal_bst(arr, left, mid - 1);
        root.right = create_minimal_bst(arr, mid + 1, right);
        Some(Rc::new(RefCell::new(root)))
    }
    create_minimal_bst(&arr, 0, arr.len() as i32 - 1)
}

#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

// 4.3 Для бинарного дерева разработайте алгоритм, который создает связный список всех узлов, находящихся
// на каждой глубине (для дерева высоты Д должно получится Д связных списков
fn task_4_3(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Box<ListNode>>> {
    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        ans: &mut Vec<Option<Box<ListNode>>>,
        height: usize,
    ) {
        if let Some(r) = root {
            let r = r.borrow();
            if ans.len() == height {
                ans.push(Some(Box::new(ListNode {
                    val: r.val,
                    next: None,
                })));
            } else {
                let mut n = ans[height].as_mut().unwrap();
                n.next = Some(Box::new(ListNode {
                    val: r.val,
                    next: None,
                }));
            }
            dfs(r.left.clone(), ans, height + 1);
            dfs(r.right.clone(), ans, height + 1);
        }
    }
    let mut ans = vec![];
    dfs(root, &mut ans, 0);
    ans
}

// 4.4 Реализуйте функцию проверяющую сбалансированность бинарного дерева.
// Будем считать дерево сбалансированным,
// если разницы высот двух поддеревьев любого узла не превышает 1
fn task_4_4(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn check_height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(r) = root {
            let r = r.borrow();
            let left_height = check_height(r.left.clone());
            if left_height == i32::MIN {
                return i32::MIN;
            }
            let right_height = check_height(r.right.clone());
            if right_height == i32::MIN {
                return i32::MIN;
            }
            if left_height.abs_diff(right_height) > 1 {
                i32::MIN
            } else {
                left_height.max(right_height) + 1
            }
        } else {
            -1
        }
    }
    check_height(root) != i32::MIN
}

// 4.5 Реализуйте функцию проверки того, является ли бинарное дерево бинарным деревом поиска
fn task_4_5(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn check_bst_inorder(root: Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>) -> bool {
        if let Some(r) = root {
            let r = r.borrow();
            if !check_bst_inorder(r.left.clone(), prev) {
                return false;
            }
            if prev.is_some() && prev.unwrap() >= r.val {
                return false;
            }
            *prev = Some(r.val);
            check_bst_inorder(r.right.clone(), prev)
        } else {
            true
        }
    }

    fn check_bst_tree_props(
        root: Option<Rc<RefCell<TreeNode>>>,
        min: Option<i32>,
        max: Option<i32>,
    ) -> bool {
        if let Some(r) = root {
            let r = r.borrow();
            if min.is_some() && r.val <= min.unwrap() {
                return false;
            }
            if max.is_some() && r.val > max.unwrap() {
                return false;
            }
            check_bst_tree_props(r.left.clone(), min, Some(r.val))
                && check_bst_tree_props(r.right.clone(), Some(r.val), max)
        } else {
            true
        }
    }
    check_bst_tree_props(root, None, None)
}

#[derive(Debug, PartialEq, Eq)]
struct TreeNodeWithParent {
    val: i32,
    parent: Option<Rc<RefCell<TreeNodeWithParent>>>, // pfff doesn't work in rust with eq
    left: Option<Rc<RefCell<TreeNodeWithParent>>>,
    right: Option<Rc<RefCell<TreeNodeWithParent>>>,
}

// 4.6 Напишите алгоритм поиска "следующего" узла для заданного узла в бинарном дереве поиска.
// Считайте, что у каждого узла есть ссылка на его родителя.
fn task_4_6(
    root: Option<Rc<RefCell<TreeNodeWithParent>>>,
) -> Option<Rc<RefCell<TreeNodeWithParent>>> {
    fn leftmost_child(
        mut root: Option<Rc<RefCell<TreeNodeWithParent>>>,
    ) -> Option<Rc<RefCell<TreeNodeWithParent>>> {
        root.as_ref()?;
        while let Some(r) = root {
            let r = r.borrow();
            root = r.left.clone();
        }
        root
    }
    fn inorder_successor(
        root: Option<Rc<RefCell<TreeNodeWithParent>>>,
    ) -> Option<Rc<RefCell<TreeNodeWithParent>>> {
        if let Some(r) = root.clone() {
            let r = r.borrow();
            if r.right.is_some() {
                leftmost_child(r.right.clone())
            } else {
                let mut q = root;
                let mut x = q.clone().unwrap().borrow().parent.clone();
                while x.is_some() && x.clone().unwrap().borrow().left != q {
                    q = x.clone();
                    x = x.clone().unwrap().borrow().parent.clone();
                }
                x
            }
        } else {
            None
        }
    }
    inorder_successor(root)
}

// 4.7 Имеется список проектов и список зависимостей (список пар проектов,
// для которого первый проект зависит от второго проекта). Проект может быть построен
// только после построения всех его зависимостей. Найдите такой порядок построения,
// который позволит построить все проекты. Если действительного порядка не существует,
// верните признак ошибки.
fn task_4_7(n: usize, deps: Vec<(char, char)>) -> Vec<char> {
    fn khan_topological_sort(n: usize, deps: Vec<(char, char)>) -> Vec<char> {
        use std::collections::VecDeque;
        let mut result = vec![];
        let mut adj = vec![vec![]; n];
        let mut in_degrees = vec![0; n];
        for (from, to) in deps {
            let from = from as usize - 'a' as usize;
            let to = to as usize - 'a' as usize;
            adj[from].push(to);
            in_degrees[to] += 1;
        }
        let mut queue = VecDeque::new();
        for (i, &d) in in_degrees.iter().enumerate() {
            if d == 0 {
                queue.push_back(i);
            }
        }
        while let Some(v) = queue.pop_front() {
            result.push(v);
            for &u in &adj[v] {
                in_degrees[u] -= 1;
                if in_degrees[u] == 0 {
                    queue.push_back(u);
                }
            }
        }
        if result.len() != n {
            return vec![]; // cycle found
        }
        result
            .into_iter()
            .map(|x| (x as u8 + b'a') as char)
            .collect()
    }

    fn dfs_topological_sort(n: usize, deps: Vec<(char, char)>) -> Vec<char> {
        fn dfs(
            v: usize,
            adj: &Vec<Vec<usize>>,
            visited: &mut Vec<usize>,
            result: &mut Vec<usize>,
        ) -> bool {
            if visited[v] == 1 {
                return true; // has cycle
            }
            visited[v] = 1;
            for &u in &adj[v] {
                if visited[u] != 2 && dfs(u, adj, visited, result) {
                    // if has cycle
                    return true;
                }
            }
            visited[v] = 2;
            result.push(v);
            false
        }
        let mut result = vec![];
        let mut adj = vec![vec![]; n];
        let mut visited = vec![0; n]; // 0 not visited, 1 visiting, 2 visited
        for (from, to) in deps {
            adj[from as usize - 'a' as usize].push(to as usize - 'a' as usize);
        }
        for i in 0..n {
            if visited[i] == 0 && dfs(i, &adj, &mut visited, &mut result) {
                return vec![]; // has cycle
            }
        }
        result
            .into_iter()
            .rev()
            .map(|x| (x as u8 + b'a') as char)
            .collect()
    }
    dfs_topological_sort(n, deps)
    // khan_topological_sort(n, deps)
}

// 4.8 Создайте алгоритм и напишите код поиска первого общего предка двух узлов бинарного дерева.
// Постарайтесь избежать хранения дополнительных узлов в структуре данных. Примечание - бинарное дерево
// не обязательно является бинарным деревом поиска
fn task_4_8_1(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn lowest_common_ancestor_binary_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            return root;
        }
        let r = root.clone().unwrap();
        let r = r.borrow();
        let left = lowest_common_ancestor_binary_tree(r.left.clone(), p.clone(), q.clone());
        let right = lowest_common_ancestor_binary_tree(r.right.clone(), p, q);
        if left.is_some() && right.is_some() {
            root
        } else if left.is_some() {
            left
        } else {
            right
        }
    }
    lowest_common_ancestor_binary_tree(root, p, q)
}

// для дерева в графе
// https://www.baeldung.com/cs/tree-lowest-common-ancestor
// The idea of this approach is to have two pointers, one on each node.
// We need to move these pointers towards the root in a way that allows them to meet at the LCA.
//
// The first thing we can notice is that these pointers should be in the same depth.
// In addition to that, the deeper pointer can never be the LCA.
// Therefore, our first step should be to keep moving the deeper pointer to its parent until the two pointers are on the same depth.
//
// Now, we have two pointers that are on the same depth.
// We can keep moving both pointers towards the root one step at a time until they meet at one node.
// Since this node is the deepest node that our pointers can meet at, therefore,
// it’s the deepest common ancestor of our starting nodes, which is the LCA.

// 3.2. Preprocessing
//
// In order to implement this solution, we will need to calculate the depth and the parent of each node.
// This can be done with a simple DFS traversal starting from the root.

// 3.3. Finding the LCA
//
// After calling the previous function starting from the root, we will have the depth and parent of each node.
// We can now apply the approach that we discussed earlier.
fn task_4_8_2(edges: Vec<(usize, usize)>, n: usize, mut p: usize, mut q: usize) -> usize {
    fn dfs(
        v: usize,
        adj: &Vec<Vec<usize>>,
        depth: &mut Vec<usize>,
        parent: &mut Vec<usize>,
        visited: &mut Vec<bool>,
    ) {
        visited[v] = true;
        for &u in &adj[v] {
            if !visited[u] {
                parent[u] = v;
                depth[u] = depth[v] + 1;
                dfs(u, adj, depth, parent, visited);
            }
        }
    }
    let mut visited = vec![false; n];
    let mut depth = vec![0; n];
    let mut parent = vec![0; n];
    let mut adj = vec![vec![]; n];
    for (from, to) in edges {
        adj[from].push(to);
        adj[to].push(from);
    }
    dfs(0, &adj, &mut depth, &mut parent, &mut visited);
    // Our first step should be to keep moving the deeper pointer to its parent
    // until the two pointers are on the same depth.
    while depth[p] != depth[q] {
        if depth[p] < depth[q] {
            q = parent[q];
        } else {
            p = parent[p];
        }
    }
    while p != q {
        p = parent[p];
        q = parent[q];
    }
    p
}

// 4.9 Бинарное дерево было создано обходом массива слева направо и вставкой каждого элемента.
// Для заданного бинарного дерева поиска с разными элементами выведите все возможное массивы, которые
// могли привести к созданию этого дерева.
fn task_4_9(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn weave_lists(
        first: &mut Vec<i32>,
        second: &mut Vec<i32>,
        results: &mut Vec<Vec<i32>>,
        prefix: &mut Vec<i32>,
    ) {
        if first.is_empty() || second.is_empty() {
            let mut result = prefix.clone();
            result.extend(first.clone());
            result.extend(second.clone());
            results.push(result);
            return;
        }

        let head_first = first.remove(0);
        prefix.push(head_first);
        weave_lists(first, second, results, prefix);
        prefix.pop();
        first.insert(0, head_first);

        let head_second = second.remove(0);
        prefix.push(head_second);
        weave_lists(first, second, results, prefix);
        prefix.pop();
        second.insert(0, head_second);
    }

    fn all_subsequences(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        if root.is_none() {
            result.push(vec![]);
            return result;
        }
        let root = root.unwrap();
        let root = root.borrow();

        let mut prefix = vec![];
        prefix.push(root.val);

        let mut left = all_subsequences(root.left.clone());
        let mut right = all_subsequences(root.right.clone());

        for l in &mut left {
            for r in &mut right {
                let mut weaved = vec![];
                weave_lists(l, r, &mut weaved, &mut prefix);
                result.extend(weaved);
            }
        }
        result
    }
    all_subsequences(root)
}

// 4.10 T1 и T2 два очень больших бинарных дерева, причем Т1 значительно больше Т2.
// Создайте алгоритм, проверяющий является ли Т2 поддеревом Т1.
// Дерево Т2 считается поддеревом Т1, если существует такой узел n в Т1, что
// поддерево, "растущее" из n, идентично дереву Т2.
// (Иначе говоря, если вырезать дерево у узле n, оно будет идентично Т2.
fn task_4_10(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn serialization_approach(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn serialize_tree_preorder(root: Option<Rc<RefCell<TreeNode>>>) -> String {
            if let Some(r) = root {
                let r = r.borrow();
                format!(
                    "{}{}{}",
                    r.val,
                    serialize_tree_preorder(r.left.clone()),
                    serialize_tree_preorder(r.right.clone())
                )
            } else {
                "X".to_string()
            }
        }
        let s1 = serialize_tree_preorder(root1);
        let s2 = serialize_tree_preorder(root2);
        println!("{} {}", s1, s2);
        s1.contains(&s2)
    }

    fn find_same_tree_approach(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn is_same_tree(
            root1: Option<Rc<RefCell<TreeNode>>>,
            root2: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if root1.is_none() && root2.is_none() {
                true
            } else if (root1.is_some() && root2.is_none()) || (root1.is_none() && root2.is_some()) {
                false
            } else {
                let r1 = root1.unwrap();
                let r1 = r1.borrow();
                let r2 = root2.unwrap();
                let r2 = r2.borrow();

                r1.val == r2.val
                    && is_same_tree(r1.left.clone(), r2.left.clone())
                    && is_same_tree(r1.right.clone(), r2.right.clone())
            }
        }
        fn find(
            root1: Option<Rc<RefCell<TreeNode>>>,
            root2: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            if root2.is_none() || root1.is_none() {
                false
            } else {
                let r1 = root1.clone().unwrap();
                let r1 = r1.borrow();
                let r2 = root2.clone().unwrap();
                let r2 = r2.borrow();

                if r1.val == r2.val && is_same_tree(root1, root2.clone()) {
                    true
                } else {
                    find(r1.left.clone(), root2.clone()) || find(r1.right.clone(), root2)
                }
            }
        }

        find(root1, root2)
    }
    assert_eq!(
        serialization_approach(root1.clone(), root2.clone()),
        find_same_tree_approach(root1.clone(), root2.clone())
    );
    find_same_tree_approach(root1, root2)
}

// 4.11 Вы пишете с нуля класс бинарного дерева поиска, который помимо методов
// вставки, поиска и удаления содержит метод getRandomNode() для получения случайного
// узла дерева. Вероятность выбора всех узлов должна быть одинаковой.
// Разработайте и реализуйте алгоритм getRandomNode; обьясните, как вы реализуете
// остальные методы.
#[derive(Clone, Debug, PartialEq, Eq)]
struct TreeNodeWithSize {
    val: i32,
    size: usize,
    left: Option<Rc<RefCell<TreeNodeWithSize>>>,
    right: Option<Rc<RefCell<TreeNodeWithSize>>>,
}

impl TreeNodeWithSize {
    fn new(val: i32) -> Self {
        Self {
            val,
            size: 1,
            left: None,
            right: None,
        }
    }

    fn insert_inorder(&mut self, val: i32) {
        if val <= self.val {
            if self.left.is_none() {
                self.left = Some(Rc::new(RefCell::new(TreeNodeWithSize {
                    val,
                    size: 1,
                    left: None,
                    right: None,
                })));
            } else {
                self.left.clone().unwrap().borrow_mut().insert_inorder(val);
            }
        } else if self.right.is_none() {
            self.right = Some(Rc::new(RefCell::new(TreeNodeWithSize {
                val,
                size: 1,
                left: None,
                right: None,
            })));
        } else {
            self.right.clone().unwrap().borrow_mut().insert_inorder(val);
        }
        self.size += 1;
    }

    fn find(&self, val: i32) -> Option<Rc<RefCell<TreeNodeWithSize>>> {
        if self.val == val {
            Some(Rc::new(RefCell::new(self.clone())))
        } else if val <= self.val {
            if let Some(l) = self.left.clone() {
                l.borrow().find(val)
            } else {
                None
            }
        } else if let Some(r) = self.right.clone() {
            r.borrow().find(val)
        } else {
            None
        }
    }

    fn get_by_idx(&self, idx: usize) -> Option<Rc<RefCell<TreeNodeWithSize>>> {
        let left_size = if self.left.is_none() {
            0
        } else {
            self.left.clone().unwrap().borrow().size
        };

        if idx < left_size {
            if self.left.is_some() {
                self.left.clone().unwrap().borrow().get_by_idx(idx)
            } else {
                None
            }
        } else if idx == left_size {
            Some(Rc::new(RefCell::new(self.clone())))
        } else if self.right.is_some() {
            self.right
                .clone()
                .unwrap()
                .borrow()
                .get_by_idx(idx - (left_size + 1))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct BinaryTree {
    root: Option<Rc<RefCell<TreeNodeWithSize>>>,
}

impl BinaryTree {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert_inorder(&mut self, val: i32) {
        if self.root.is_none() {
            self.root = Some(Rc::new(RefCell::new(TreeNodeWithSize::new(val))));
        } else {
            self.root.clone().unwrap().borrow_mut().insert_inorder(val);
        }
    }

    fn get_random_node(&self) -> Option<Rc<RefCell<TreeNodeWithSize>>> {
        if self.root.is_none() {
            None
        } else {
            use rand;
            let mut rng = rand::thread_rng();
            let i = rng.gen::<usize>() % self.root.clone().unwrap().borrow().size;
            self.root.clone().unwrap().borrow().get_by_idx(i)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task_4_1() {
        assert!(task_4_1(vec![(0, 1), (1, 2), (2, 3), (3, 0)], 0, 3));
        assert!(!task_4_1(vec![(0, 1), (1, 2), (2, 0), (3, 3)], 0, 3));
    }

    #[test]
    fn test_task_4_2() {
        println!("{:?}", task_4_2(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_task_4_3() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{:?}", task_4_3(root));
    }

    #[test]
    fn test_task_4_4() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{:?}", task_4_4(root)); // false
    }

    #[test]
    fn test_task_4_5() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        println!("{:?}", task_4_5(root)); // true

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        println!("{:?}", task_4_5(root)); // false
    }

    #[test]
    fn test_task_4_6() {
        let r = Some(Rc::new(RefCell::new(TreeNodeWithParent {
            val: 0,
            parent: None,
            left: None,
            right: None,
        })));
        let right = Some(Rc::new(RefCell::new(TreeNodeWithParent {
            val: 2,
            parent: r.clone(),
            left: None,
            right: None,
        })));
        let left = Some(Rc::new(RefCell::new(TreeNodeWithParent {
            val: 0,
            parent: r.clone(),
            left: None,
            right: None,
        })));
        r.clone().unwrap().borrow_mut().left = left.clone();
        r.unwrap().borrow_mut().right = right;

        println!("{:?}", task_4_6(left));
    }

    #[test]
    fn test_task_4_7() {
        println!(
            "{:?}",
            task_4_7(
                6,
                vec![
                    ('d', 'a'),
                    ('b', 'f'),
                    ('d', 'b'),
                    ('a', 'f'),
                    ('c', 'd'),
                    // ('f', 'c') // cycle
                ]
            )
        ); // ['e', 'c', 'd', 'b', 'a', 'f'] or ['c', 'e', 'd', 'a', 'b', 'f']
    }

    #[test]
    fn test_task_4_8_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 0,
            left: None,
            right: None,
        })));
        let left = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        let right = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })));
        root.clone().unwrap().borrow_mut().left = left.clone();
        root.clone().unwrap().borrow_mut().right = right.clone();

        println!("{:?}", task_4_8_1(root, left, right));
    }

    #[test]
    fn test_task_4_8_2() {
        println!(
            "{}",
            task_4_8_2(vec![(0, 1), (0, 2), (2, 3), (2, 4), (4, 5)], 6, 3, 5)
        ); // 2
        println!(
            "{}",
            task_4_8_2(vec![(0, 1), (0, 2), (2, 3), (2, 4), (4, 5)], 6, 3, 1)
        ); // 0
    }

    #[test]
    fn test_task_4_9() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        println!("{:?}", task_4_9(root)); // [[2, 1, 3], [2, 3, 1]]
    }

    #[test]
    fn test_task_4_10() {
        let root1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let root2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        })));
        println!("{:?}", task_4_10(root1, root2)); // true

        let root1 = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let root2 = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{:?}", task_4_10(root1, root2)); // false
    }

    #[test]
    fn test_task_4_11() {
        let mut tree = BinaryTree::new();
        tree.insert_inorder(3);
        println!("{:?}", tree);
        tree.insert_inorder(4);
        println!("{:?}", tree);
        tree.insert_inorder(1);
        println!("{:?}", tree);
        tree.insert_inorder(2);
        println!("{:?}", tree);
        println!("and random node:");
        println!("{:?}", tree.get_random_node());
    }
}
