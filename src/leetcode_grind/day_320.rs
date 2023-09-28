// https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/description/
pub fn find_maximum_xor_hashset(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut max_len = 0;
    let mut max_num = *nums.iter().max().unwrap();
    while max_num > 0 {
        max_num >>= 1;
        max_len += 1;
    }

    let mut max_xor = 0;
    let mut prefixes = HashSet::new();
    for i in (0..max_len).rev() {
        max_xor <<= 1;
        let cur_xor = max_xor | 1;
        prefixes.clear();

        for &num in &nums {
            prefixes.insert(num >> i);
        }

        for &p in &prefixes {
            if prefixes.contains(&(cur_xor ^ p)) {
                max_xor = cur_xor;
                break;
            }
        }
    }
    max_xor
}

pub fn find_maximum_xor_trie(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    use std::{cell::RefCell, rc::Rc};

    struct TrieNode {
        children: HashMap<char, Rc<RefCell<TrieNode>>>,
    }

    let mut max_len = 0;
    let mut max_num = *nums.iter().max().unwrap();
    while max_num > 0 {
        max_num >>= 1;
        max_len += 1;
    }

    let n = nums.len();
    let bit_mask = 1 << max_len;

    let mut str_nums = vec!["".to_string(); n];
    for i in 0..n {
        str_nums[i] = format!("{:b}", bit_mask | nums[i]);
    }

    let trie = Rc::new(RefCell::new(TrieNode {
        children: HashMap::new(),
    }));

    let mut max_xor = 0;
    for num in str_nums {
        let mut node = trie.clone();
        let mut xor_node = trie.clone();

        let mut cur_xor = 0;

        for bit in num.chars() {
            node.borrow_mut()
                .children
                .entry(bit)
                .or_insert(Rc::new(RefCell::new(TrieNode {
                    children: HashMap::new(),
                })));
            let next = node.borrow_mut().children.get_mut(&bit).unwrap().clone();
            node = next;

            let toggle_bit = if bit == '1' { '0' } else { '1' };

            if xor_node.borrow().children.contains_key(&toggle_bit) {
                cur_xor = (cur_xor << 1) | 1;
                let next = xor_node.borrow().children.get(&toggle_bit).unwrap().clone();
                xor_node = next;
            } else {
                cur_xor = cur_xor << 1;
                let next = xor_node.borrow().children.get(&bit).unwrap().clone();
                xor_node = next;
            }
        }
        max_xor = max_xor.max(cur_xor);
    }
    max_xor
}
