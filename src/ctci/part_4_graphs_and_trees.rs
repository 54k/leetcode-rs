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
#[derive(Debug)]
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

#[derive(Debug, PartialEq)]
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
}
