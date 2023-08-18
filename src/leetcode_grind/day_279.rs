use std::{cell::RefCell, rc::Rc};

// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/description/
pub fn max_depth(s: String) -> i32 {
    let mut stack = vec![];
    let mut dep = 0;
    for ch in s.chars() {
        if ch == '(' {
            stack.push('(');
        } else if ch == ')' {
            dep = dep.max(stack.len() as i32);
            stack.pop();
        }
    }
    dep
}

// https://leetcode.com/problems/remove-outermost-parentheses/description/
pub fn remove_outer_parentheses(s: String) -> String {
    let mut bal = 0;
    let mut ans: Vec<char> = vec![];
    let s = s.chars().collect::<Vec<_>>();
    let mut last_open = 0;
    for i in 0..s.len() {
        let add = if s[i] == ')' { 1 } else { -1 };
        bal += add;
        if bal == 0 {
            ans.extend(&s[last_open + 1..i]);
            last_open = i + 1;
        }
    }
    ans.into_iter().collect()
}

// https://leetcode.com/problems/baseball-game/description/
pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut stack = vec![];
    for op in operations {
        let op = op.as_str();
        match op {
            "+" => {
                let f = stack.pop().unwrap();
                let s = *stack.last().unwrap();
                stack.push(f);
                stack.push(f + s);
            }
            "D" => {
                let f = stack.last().unwrap();
                stack.push(f * 2);
            }
            "C" => {
                stack.pop();
            }
            _ => {
                stack.push(i32::from_str_radix(op, 10).unwrap());
            }
        }
        println!("{:?}", stack);
    }
    stack.into_iter().sum::<i32>()
}

// https://leetcode.com/problems/crawler-log-folder/
pub fn min_operations(logs: Vec<String>) -> i32 {
    let mut count = 0;
    for log in logs {
        let log = &log[..log.len() - 1];

        match log {
            ".." => count = 0.max(count - 1),
            "." => {}
            _ => {
                count += 1;
            }
        }
    }
    count
}

// https://leetcode.com/problems/minimum-string-length-after-removing-substrings/
pub fn min_length(s: String) -> i32 {
    let mut s = s.chars().collect::<Vec<_>>();
    loop {
        let mut changed = false;
        let mut stack = vec![];
        for &ch in &s {
            match ch {
                'B' => {
                    if stack.len() > 0 && *stack.last().unwrap() == 'A' {
                        changed = true;
                        stack.pop();
                    } else {
                        stack.push(ch);
                    }
                }
                'D' => {
                    if stack.len() > 0 && *stack.last().unwrap() == 'C' {
                        changed = true;
                        stack.pop();
                    } else {
                        stack.push(ch);
                    }
                }
                _ => {
                    stack.push(ch);
                }
            }
        }
        s = stack;
        if !changed {
            break;
        }
    }
    s.len() as i32
}

#[test]
fn test_min_length() {
    let res = min_length("ABFCACDB".to_string());
    println!("{res}");
}

// https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/
pub fn count_students(students: Vec<i32>, mut sandwiches: Vec<i32>) -> i32 {
    use std::collections::VecDeque;

    let mut students = students.into_iter().collect::<VecDeque<_>>();
    sandwiches.reverse();

    loop {
        let mut k = students.len();
        let mut flag = false;
        while k > 0 {
            if *students.front().unwrap() != *sandwiches.last().unwrap() {
                let top = students.pop_front().unwrap();
                students.push_back(top);
            } else {
                sandwiches.pop();
                students.pop_front();
                flag = true;
            }
            k -= 1;
        }
        if !flag {
            break;
        }
    }

    students.len() as i32
}

// https://leetcode.com/problems/time-needed-to-buy-tickets/
pub fn time_required_to_buy(mut tickets: Vec<i32>, mut k: i32) -> i32 {
    let mut time = 0;

    loop {
        for i in 0..tickets.len() {
            if tickets[i] == 0 {
                continue;
            }
            tickets[i] -= 1;
            time += 1;
            if i as i32 == k && tickets[i] == 0 {
                return time;
            }
        }
    }
}

// https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/description/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    type Node = Option<Rc<RefCell<TreeNode>>>;
    use std::collections::HashMap;
    let mut inorder = preorder.clone();
    inorder.sort();
    let val_idx = inorder
        .iter()
        .copied()
        .enumerate()
        .map(|(i, v)| (v, i as i32))
        .collect::<HashMap<_, _>>();

    fn construct(
        pre_idx: &mut usize,
        left: i32,
        right: i32,
        inorder: &Vec<i32>,
        preorder: &Vec<i32>,
        val_idx: &HashMap<i32, i32>,
    ) -> Node {
        if left >= right {
            return None;
        }

        let root_val = preorder[*pre_idx];
        let mut node = TreeNode {
            val: root_val,
            left: None,
            right: None,
        };
        *pre_idx += 1;

        node.left = construct(
            pre_idx,
            left,
            val_idx[&root_val],
            inorder,
            preorder,
            val_idx,
        );
        node.right = construct(
            pre_idx,
            val_idx[&root_val] + 1,
            right,
            inorder,
            preorder,
            val_idx,
        );

        Some(Rc::new(RefCell::new(node)))
    }

    let mut pre_idx = 0;
    construct(
        &mut pre_idx,
        0,
        inorder.len() as i32,
        &inorder,
        &preorder,
        &val_idx,
    )
}
