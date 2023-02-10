use std::cell::RefCell;
use std::rc::Rc;

pub fn distinct_names(ideas: Vec<String>) -> i64 {
    fn brute(ideas: Vec<String>) -> i64 {
        use std::collections::HashSet;
        let words_set = ideas.iter().collect::<HashSet<_>>();
        let mut ans = 0;
        for i in 0..ideas.len() {
            for j in 0..ideas.len() {
                let w1 = &ideas[i];
                let w2 = &ideas[j];
                let s1 = format!(
                    "{}{}",
                    w2.chars().take(1).collect::<String>(),
                    w1.chars().skip(1).collect::<String>()
                );
                let s2 = format!(
                    "{}{}",
                    w1.chars().take(1).collect::<String>(),
                    w2.chars().skip(1).collect::<String>()
                );
                if !words_set.contains(&s1) && !words_set.contains(&s2) {
                    ans += 1;
                }
            }
        }
        ans
    }

    fn optimized(ideas: Vec<String>) -> i64 {
        use std::collections::HashSet;
        let mut ans = 0;
        // Group idea by their initials.
        let mut groups = vec![HashSet::<String>::new(); 26];
        for w in ideas {
            let x = w.chars().collect::<Vec<_>>();
            groups[x[0] as usize - 'a' as usize].insert(x.into_iter().skip(1).collect::<String>());
        }
        // Calculate number of valid names from every pair of groups.
        for i in 0..25 {
            for j in i + 1..26 {
                let s1 = &groups[i];
                let s2 = &groups[j];
                if !s1.is_empty() && !s2.is_empty() {
                    // Valid names are only from distinct suffixes in both groups.
                    let count_of_intersections = s1.intersection(s2).count();
                    // Since we can swap a with b and swap b with a to create two valid names, multiple answer by 2.
                    ans += ((s1.len() as i64 - count_of_intersections as i64)
                        * (s2.len() as i64 - count_of_intersections as i64))
                        * 2_i64;
                }
            }
        }
        ans
    }

    optimized(ideas)
}

// https://leetcode.com/problems/find-duplicate-subtrees/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn find_duplicate_subtrees(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    // The function works as follows:
    //
    // Traverse the left subtree of the node and get its representation (call recursively traverse(node->left)).
    // Traverse the right subtree of the node and get its representation (call recursively traverse(node->right)).
    // Compose the representation of the current subtree using the left subtree representation,
    // the value of the node, and the right subtree representation.
    // If the string occurs for the second time, it means there already was the same subtree as the current one (the subtree of the node).
    // In this case, we add the node to the answer.
    // Return the string from the function.
    // We only need to call traverse(root) to solve the problem.
    use std::collections::*;
    type Node = Option<Rc<RefCell<TreeNode>>>;
    let mut ans = vec![];
    fn dfs(root: Node, trees_counts: &mut HashMap<String, i32>, ans: &mut Vec<Node>) -> String {
        if root.is_none() {
            return "".to_string();
        }

        let r = root.clone().unwrap();
        let r = r.borrow();
        let tree_representation = format!(
            "({}){}({})",
            dfs(r.left.clone(), trees_counts, ans),
            r.val,
            dfs(r.right.clone(), trees_counts, ans)
        );

        let entry = trees_counts.entry(tree_representation.clone()).or_insert(0);
        *entry += 1;

        if *entry == 2 {
            ans.push(root);
        }
        tree_representation
    }
    dfs(root, &mut HashMap::new(), &mut ans);
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test216() {
        println!(
            "{}",
            distinct_names(vec![
                "coffee".to_string(),
                "donuts".to_string(),
                "time".to_string(),
                "toffee".to_string()
            ])
        ); // 6
        println!(
            "{}",
            distinct_names(vec!["lack".to_string(), "back".to_string(),])
        ); // 0
    }

    #[test]
    fn test217() {
        println!(
            "{:?}",
            find_duplicate_subtrees(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 4,
                            left: None,
                            right: None,
                        }))),
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))))
        ); // [[2,4],[4]]
    }
}
