use std::{cell::RefCell, rc::Rc};

// https://leetcode.com/problems/implement-stack-using-queues/description/
mod stack_2q {
    use std::collections::VecDeque;

    pub struct MyStack {
        q1: VecDeque<i32>,
        q2: VecDeque<i32>,
        top: i32,
    }

    impl MyStack {
        pub fn new() -> Self {
            Self {
                q1: VecDeque::new(),
                q2: VecDeque::new(),
                top: 0,
            }
        }

        pub fn push(&mut self, x: i32) {
            self.q1.push_back(x);
            self.top = x;
        }

        pub fn pop(&mut self) -> i32 {
            while self.q1.len() > 1 {
                self.top = self.q1.pop_front().unwrap();
                self.q2.push_back(self.top);
            }
            let rem = self.q1.pop_front().unwrap();
            std::mem::swap(&mut self.q1, &mut self.q2);
            rem
        }

        pub fn top(&self) -> i32 {
            self.top
        }

        pub fn empty(&self) -> bool {
            self.q1.is_empty()
        }
    }
}

mod stack_1q {
    use std::collections::VecDeque;
    pub struct MyStack {
        q: VecDeque<i32>,
    }

    impl MyStack {
        pub fn new() -> Self {
            Self { q: VecDeque::new() }
        }

        pub fn push(&mut self, x: i32) {
            self.q.push_back(x);
            let mut sz = self.q.len();
            while sz > 1 {
                let pop = self.q.pop_front().unwrap();
                self.q.push_back(pop);
                sz -= 1;
            }
        }

        pub fn pop(&mut self) -> i32 {
            self.q.pop_front().unwrap()
        }

        pub fn top(&self) -> i32 {
            *self.q.front().unwrap()
        }

        pub fn empty(&self) -> bool {
            self.q.is_empty()
        }
    }
}

// https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/description/
pub fn min_remove_to_make_valid(s: String) -> String {
    fn stack_approach(s: String) -> String {
        use std::collections::HashSet;
        let s = s.chars().collect::<Vec<_>>();
        let mut set = HashSet::new();
        let mut stack = vec![];

        for i in 0..s.len() {
            if s[i] == '(' {
                stack.push(i);
            } else if s[i] == ')' {
                if stack.is_empty() {
                    set.insert(i);
                } else {
                    stack.pop();
                }
            }
        }

        while !stack.is_empty() {
            set.insert(stack.pop().unwrap());
        }

        let mut ans = String::new();
        for i in 0..s.len() {
            if !set.contains(&i) {
                ans.push(s[i]);
            }
        }
        ans
    }
    fn sb_approach(s: String) -> String {
        fn rem(s: Vec<char>, open: char, close: char) -> Vec<char> {
            let mut ans = String::new();
            let mut bal = 0;
            for i in 0..s.len() {
                if s[i] == open {
                    bal += 1;
                }
                if s[i] == close {
                    if bal == 0 {
                        continue;
                    }
                    bal -= 1;
                }
                ans.push(s[i]);
            }
            ans.chars().collect()
        }
        let s = s.chars().collect::<Vec<_>>();
        let mut s = rem(s, '(', ')');
        s.reverse();
        let mut s = rem(s, ')', '(');
        s.reverse();
        s.into_iter().collect()
    }
    fn simplified_sb_approach(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();

        let mut sb = String::new();
        let mut open_seen = 0;
        let mut bal = 0;

        for i in 0..s.len() {
            if s[i] == '(' {
                open_seen += 1;
                bal += 1;
            } else if s[i] == ')' {
                if bal == 0 {
                    continue;
                }
                bal -= 1;
            }
            sb.push(s[i]);
        }

        let s = sb.chars().collect::<Vec<_>>();
        let mut ans = String::new();
        let mut open_to_keep = open_seen - bal;
        for i in 0..s.len() {
            if s[i] == '(' {
                open_to_keep -= 1;
                if open_to_keep < 0 {
                    continue;
                }
            }
            ans.push(s[i]);
        }
        ans
    }
    simplified_sb_approach(s)
}

// https://leetcode.com/problems/similar-string-groups/description/
pub fn num_similar_groups(strs: Vec<String>) -> i32 {
    fn is_sim(s1: &Vec<char>, s2: &Vec<char>) -> bool {
        let mut diff = 0;
        for i in 0..s1.len() {
            if s1[i] != s2[i] {
                diff += 1;
            }
            if diff > 2 {
                return false;
            }
        }
        true
    }

    struct UF {
        cp: Vec<usize>,
        sz: Vec<usize>,
    }
    impl UF {
        fn new(n: usize) -> Self {
            Self {
                cp: (0..n).collect(),
                sz: vec![1; n],
            }
        }
        fn find(&mut self, x: usize) -> usize {
            if x != self.cp[x] {
                self.cp[x] = self.find(self.cp[x]);
            }
            self.cp[x]
        }
        fn union(&mut self, x: usize, y: usize) {
            let mut x = self.find(x);
            let mut y = self.find(y);
            if x == y {
                return;
            }
            if self.sz[x] < self.sz[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.cp[y] = self.cp[x];
            self.sz[x] += self.sz[y];
        }
    }

    let mut uf = UF::new(strs.len());
    let strs = strs
        .into_iter()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut groups = strs.len() as i32;
    for i in 0..strs.len() {
        for j in i + 1..strs.len() {
            if is_sim(&strs[i], &strs[j]) && uf.find(i) != uf.find(j) {
                uf.union(i, j);
                groups -= 1;
            }
        }
    }

    groups
}

// https://leetcode.com/problems/max-consecutive-ones/description/
pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut cur = 0;
    for n in nums {
        if n == 0 {
            cur = 0
        } else {
            cur += 1;
            ans = ans.max(cur);
        }
    }
    ans
}

// https://leetcode.com/problems/find-numbers-with-even-number-of-digits/
pub fn find_numbers(nums: Vec<i32>) -> i32 {
    fn numd(mut n: i32) -> i32 {
        let mut ans = 0;
        while n > 0 {
            n /= 10;
            ans += 1;
        }
        ans
    }
    nums.into_iter()
        .map(|x| numd(x))
        .filter(|x| x % 2 == 0)
        .count() as i32
}
// https://leetcode.com/problems/even-odd-tree/description/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut lvl = vec![];
    lvl.push(root.clone());
    let mut d = 0;

    while !lvl.is_empty() {
        let mut next = vec![];

        for r in lvl.iter().map(|x| x.clone()).flatten() {
            let r = r.borrow();
            if r.left.is_some() {
                next.push(r.left.clone());
            }
            if r.right.is_some() {
                next.push(r.right.clone());
            }
        }

        for i in 0..lvl.len() {
            let v = lvl[i].as_ref().unwrap().borrow().val;
            let good = if d % 2 == 0 { v % 2 == 1 } else { v % 2 == 0 };
            if !good {
                return false;
            }

            if i > 0 {
                let p = lvl[i - 1].as_ref().unwrap().borrow().val;
                if d % 2 == 0 && v <= p {
                    return false;
                } else if d % 2 == 1 && v >= p {
                    return false;
                }
            }
        }

        lvl = next;
        d += 1;
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_459() {
        let mut stack = stack_1q::MyStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);

        println!("{}", stack.pop());
        println!("{}", stack.pop());
        println!("{}", stack.pop());
    }

    #[test]
    fn test460() {
        println!("{}", min_remove_to_make_valid("lee(t(c)o)de)".to_string()));
    }

    #[test]
    fn test461() {
        println!(
            "{}",
            num_similar_groups(vec![
                "tars".to_owned(),
                "rats".to_owned(),
                "arts".to_owned(),
                "star".to_owned(),
            ])
        ); // 2

        println!(
            "{}",
            num_similar_groups(vec!["omv".to_owned(), "ovm".to_owned()])
        ); // 1

        println!(
            "{}",
            num_similar_groups(vec!["star".to_owned(), "tars".to_owned()])
        ); // 2
    }
}
