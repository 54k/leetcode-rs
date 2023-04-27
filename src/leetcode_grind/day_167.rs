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
}
