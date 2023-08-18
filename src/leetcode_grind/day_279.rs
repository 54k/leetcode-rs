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
    