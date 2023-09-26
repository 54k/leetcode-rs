// https://leetcode.com/problems/remove-duplicate-letters/description
pub fn remove_duplicate_letters(s: String) -> String {
    use std::collections::HashMap;
    use std::collections::HashSet;
    let s = s.chars().collect::<Vec<_>>();
    let mut last_idx = HashMap::new();
    let mut on_stack = HashSet::new();

    for i in 0..s.len() {
        last_idx.insert(s[i], i);
    }

    let mut stack = vec![];
    for i in 0..s.len() {
        if !on_stack.contains(&s[i]) {
            while stack.len() > 0
                && i < last_idx[&stack[stack.len() - 1]]
                && stack[stack.len() - 1] >= s[i]
            {
                on_stack.remove(&stack.pop().unwrap());
            }
            stack.push(s[i]);
            on_stack.insert(s[i]);
        }
    }

    stack.into_iter().collect::<_>()
}
