// https://leetcode.com/problems/implement-stack-using-queues/description/
use std::collections::VecDeque;

struct MyStack {
    q1: VecDeque<i32>,
    q2: VecDeque<i32>,
    top: i32,
}

impl MyStack {
    fn new() -> Self {
        Self {
            q1: VecDeque::new(),
            q2: VecDeque::new(),
            top: -1,
        }
    }

    fn push(&mut self, x: i32) {
        self.q1.push_back(x);
        self.top = x;
    }

    fn pop(&mut self) -> i32 {
        while self.q1.len() > 1 {
            self.top = self.q1.pop_front().unwrap();
            self.q2.push_back(self.top);
        }
        let res = self.q1.pop_front().unwrap();
        std::mem::swap(&mut self.q1, &mut self.q2);
        res
    }

    fn top(&mut self) -> i32 {
        self.top
    }

    fn empty(&mut self) -> bool {
        self.q1.is_empty()
    }
}
