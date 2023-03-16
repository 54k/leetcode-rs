use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    use std::collections::*;

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, levels: &mut Vec<VecDeque<i32>>, depth: usize) {
        if let Some(r) = root {
            if levels.len() == depth {
                levels.push(VecDeque::new())
            }
            let r = r.borrow();
            if depth % 2 == 0 {
                levels[depth].push_back(r.val);
            } else {
                levels[depth].push_front(r.val);
            }
            dfs(r.left.clone(), levels, depth + 1);
            dfs(r.right.clone(), levels, depth + 1);
        }
    }

    let mut ans = vec![];
    dfs(root, &mut ans, 0);
    ans.into_iter()
        .map(|x| x.into_iter().collect::<Vec<_>>())
        .collect()
}

// https://leetcode.com/problems/groups-of-special-equivalent-strings/
pub fn num_special_equiv_groups(words: Vec<String>) -> i32 {
    use std::collections::HashSet;
    words
        .into_iter()
        .map(|w| {
            let (mut even, mut odd) = (vec![0; 26], vec![0; 26]);
            for (i, ch) in w.chars().enumerate() {
                let idx = ch as usize - 'a' as usize;
                if i % 2 == 0 {
                    even[idx] += 1;
                } else {
                    odd[idx] += 1;
                }
            }
            (even, odd)
        })
        .fold(HashSet::new(), |mut acc, k| {
            acc.insert(k);
            acc
        })
        .len() as i32
}

// https://leetcode.com/problems/all-possible-full-binary-trees/description/
pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    use std::collections::HashMap;
    let mut memo = HashMap::new();
    fn all_possible_fbt(
        n: i32,
        memo: &mut HashMap<i32, Vec<Option<Rc<RefCell<TreeNode>>>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if !memo.contains_key(&n) {
            let mut ans = vec![];
            if n == 1 {
                ans.push(Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))));
            } else if n % 2 == 1 {
                for i in 0..n {
                    let j = n - 1 - i;
                    for left in all_possible_fbt(i, memo) {
                        for right in all_possible_fbt(j, memo) {
                            let mut bns = TreeNode {
                                val: 0,
                                left: None,
                                right: None,
                            };
                            bns.left = left.clone();
                            bns.right = right.clone();
                            ans.push(Some(Rc::new(RefCell::new(bns))));
                        }
                    }
                }
            }
            memo.insert(n, ans);
        }
        memo.get(&n).unwrap().clone()
    }
    all_possible_fbt(n, &mut memo);
    memo.remove(&n).unwrap()
}

pub fn get_startup_name() {
    fn permute(
        keywords: &Vec<String>,
        k: usize,
        results: &mut Vec<String>,
        selected: &mut Vec<usize>,
        len: usize,
    ) {
        if len == k {
            results.push(
                selected
                    .iter()
                    .map(|x| keywords[*x].clone())
                    .collect::<Vec<_>>()
                    .join(""),
            );
            return;
        }
        for i in 0..keywords.len() {
            if !selected.contains(&i) {
                selected.push(i);
                permute(keywords, k, results, selected, len + 1);
                selected.pop();
            }
        }
    }

    let mut keywords = vec![
        "DAO".to_string(),
        "Fund".to_string(),
        "Fortune".to_string(),
        "Crowd".to_string(),
        "Club".to_string(),
        "Coin".to_string(),
        "Token".to_string(),
        "Titan".to_string(),
        "Citadel".to_string(),
        "Fortress".to_string(),
        "Eos".to_string(),
        "Crypto".to_string(),
        "Helios".to_string(),
    ];
    keywords.sort();
    let mut selected = vec![];

    let mut names = vec![];
    let k = 2;

    permute(&keywords, k, &mut names, &mut selected, 0);

    names.into_iter().for_each(|n| {
        println!("{}", n);
    });
}

// https://leetcode.com/problems/first-unique-character-in-a-string/
pub fn first_uniq_char(s: String) -> i32 {
    use std::collections::*;
    let mut map = HashMap::new();
    for ch in s.chars() {
        *map.entry(ch).or_insert(0) += 1;
    }
    for (i, ch) in s.chars().enumerate() {
        if *map.get(&ch).unwrap() == 1 {
            return i as i32;
        }
    }
    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test251() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
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
        })));
        println!("{:?}", zigzag_level_order(root));
    }

    #[test]
    fn test252() {
        println!("{:?}", all_possible_fbt(7));
    }

    #[test]
    fn test253() {
        println!("{}", first_uniq_char("loveleetcode".to_string())); // 2
    }
}
