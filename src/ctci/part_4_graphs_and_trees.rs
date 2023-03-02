use std::cell::RefCell;
use std::rc::Rc;

// Для заданного направленного графа реализуйте алгоритм,
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

// напишите алгоритм создания бинарного дерева с минимальной высотой из отсортированного (по возрастанию)
// массива с уникальными целочисленными элементами
#[derive(Debug)]
struct Node {
    val: i32,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

fn task_4_2(arr: Vec<i32>) -> Option<Rc<RefCell<Node>>> {
    fn create_minimal_bst(arr: &[i32], left: i32, right: i32) -> Option<Rc<RefCell<Node>>> {
        if left > right {
            return None;
        }
        let mid = (left + right) / 2;
        let mut root = Node {
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

// Для бинарного дерева разработайте алгоритм, который создает связный список всех узлов, находящихся
// на каждой глубине (для дерева высоты Д должно получится Д связных списков
fn task_4_3(root: Option<Rc<RefCell<Node>>>) -> Vec<Option<Box<ListNode>>> {
    fn dfs(root: Option<Rc<RefCell<Node>>>, ans: &mut Vec<Option<Box<ListNode>>>, height: usize) {
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
        let root = Some(Rc::new(RefCell::new(Node {
            val: 0,
            left: Some(Rc::new(RefCell::new(Node {
                val: 1,
                left: Some(Rc::new(RefCell::new(Node {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(Node {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(Node {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{:?}", task_4_3(root));
    }
}
