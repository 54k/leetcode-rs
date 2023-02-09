use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

// https://leetcode.com/problems/jump-game-iii/description/
pub fn can_reach(mut arr: Vec<i32>, start: i32) -> bool {
    fn rec(arr: &mut Vec<i32>, cur_pos: i32) -> bool {
        if cur_pos < 0 || cur_pos >= arr.len() as i32 {
            return false;
        }
        if arr[cur_pos as usize] == -1 {
            return false;
        }
        if arr[cur_pos as usize] == 0 {
            return true;
        }
        let t = arr[cur_pos as usize];
        arr[cur_pos as usize] = -1;
        let ans = rec(arr, cur_pos + t) || rec(arr, cur_pos - t);
        arr[cur_pos as usize] = t;
        ans
    }

    rec(&mut arr, start)
}

// https://leetcode.com/problems/group-anagrams/description/
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    use std::collections::*;
    let mut ans = HashMap::new();
    for s in strs {
        let mut hist = vec![0; 26];
        for c in s.chars() {
            hist[c as usize - 'a' as usize] += 1;
        }
        ans.entry(hist).or_insert(vec![]).push(s);
    }
    ans.into_values().collect()
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// https://leetcode.com/problems/count-good-nodes-in-binary-tree/
pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    type Node = Option<Rc<RefCell<TreeNode>>>;
    fn good_nodes1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Node, path: &mut Vec<i32>) -> i32 {
            if root.is_none() {
                return 0;
            }
            let r = root.unwrap();
            let r = r.borrow();
            path.push(r.val);
            let mut ans = 0;
            let mut good = true;
            for i in 0..path.len() {
                if path[i] > r.val {
                    good = false;
                }
            }
            ans += good as i32;
            if r.left.is_some() {
                ans += dfs(r.left.clone(), path);
                path.pop();
            }
            if r.right.is_some() {
                ans += dfs(r.right.clone(), path);
                path.pop();
            }
            ans
        }
        dfs(root, &mut vec![])
    }
    fn good_nodes2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Node, max: i32) -> i32 {
            if root.is_none() {
                0
            } else {
                let r = root.unwrap();
                let r = r.borrow();
                let mut ans = if r.val >= max { 1 } else { 0 };
                ans += dfs(r.left.clone(), r.val);
                ans += dfs(r.right.clone(), r.val);
                ans
            }
        }
        dfs(root, i32::MIN)
    }
    good_nodes2(root)
}

// https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree/solutions/?orderBy=most_relevant
pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    use std::cmp::*;
    use std::collections::*;
    type Node = Option<Rc<RefCell<TreeNode>>>;

    let mut heap = BinaryHeap::new();

    fn dfs(root: Node, coord: (i32, i32), heap: &mut BinaryHeap<Reverse<((i32, i32), i32)>>) {
        if root.is_none() {
            return;
        }
        let r = root.unwrap();
        let r = r.borrow();
        heap.push(Reverse(((coord.1, coord.0), r.val)));
        dfs(r.left.clone(), (coord.0 + 1, coord.1 - 1), heap);
        dfs(r.right.clone(), (coord.0 + 1, coord.1 + 1), heap);
    }

    dfs(root, (0, 0), &mut heap);
    let mut ans = vec![];
    let mut prev = i32::MIN;

    while let Some(Reverse(((lvl, _), val))) = heap.pop() {
        if lvl != prev {
            ans.push(vec![]);
        }
        ans.last_mut().unwrap().push(val);
        prev = lvl;
    }
    ans
}

// https://leetcode.com/problems/cinema-seat-allocation/
pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
    fn max_number_of_families1(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        // doesn't work with n = 10^9
        let mut seats = vec![0; n as usize];
        const PADDING: i32 = i32::MAX << 10;
        const FAM: i32 = (1 << 4) - 1;
        for s in reserved_seats {
            let row = s[0];
            let occupied = s[1];
            seats[row as usize - 1] |= (PADDING) | (1 << (10 - occupied));
        }
        for i in 0..n {
            seats[i as usize] = PADDING | !seats[i as usize];
        }

        const FAM_TWO: i32 = PADDING | FAM << 1 | FAM << 5;
        const FAM_RIGHT: i32 = PADDING | FAM << 1;
        const FAM_LEFT: i32 = PADDING | FAM << 5;
        const FAM_MID: i32 = PADDING | FAM << 3;

        let mut ans = 0;
        for s in seats {
            if FAM_TWO & s == FAM_TWO {
                ans += 2;
            } else if (FAM_RIGHT & s == FAM_RIGHT)
                || (FAM_LEFT & s == FAM_LEFT)
                || (FAM_MID & s == FAM_MID)
            {
                ans += 1
            }
        }
        ans
    }
    max_number_of_families1(n, reserved_seats)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test211() {
        println!("{}", can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5)); // true
        println!("{}", can_reach(vec![3, 0, 2, 1, 2], 2)); // false
    }

    #[test]
    fn test212() {
        println!(
            "{:?}",
            group_anagrams(vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string()
            ])
        ); // [["bat"],["nat","tan"],["ate","eat","tea"]]
    }

    #[test]
    fn test213() {
        println!(
            "{}",
            good_nodes(Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))))
        ); // 1

        println!(
            "{}",
            good_nodes(Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))))
        ); // 4

        println!(
            "{}",
            good_nodes(Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
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
                    right: None,
                }))),
            }))))
        ); // 3
    }

    #[test]
    fn test214() {
        println!(
            "{:?}",
            vertical_traversal(Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 20,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 15,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))))
        ); // [[9],[3,15],[20],[7]]

        println!(
            "{:?}",
            vertical_traversal(Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 0,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
            }))))
        ); // [[0],[1],[3,2,2],[4]]
    }

    #[test]
    fn test215() {
        println!(
            "{}",
            max_number_of_families(
                3,
                vec![
                    vec![1, 2],
                    vec![1, 3],
                    vec![1, 8],
                    vec![2, 6],
                    vec![3, 1],
                    vec![3, 10]
                ]
            )
        ); //4

        println!(
            "{}",
            max_number_of_families(2, vec![vec![2, 1], vec![1, 8], vec![2, 6]])
        ); //2

        println!(
            "{}",
            max_number_of_families(
                3,
                vec![
                    vec![1, 2],
                    vec![1, 3],
                    vec![1, 8],
                    vec![2, 6],
                    vec![3, 1],
                    vec![3, 10]
                ]
            )
        ); // 4
    }
}
