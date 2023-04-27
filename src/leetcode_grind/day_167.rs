use std::collections::VecDeque;

// https://leetcode.com/problems/implement-stack-using-queues/
struct MyStack {
    q1: VecDeque<i32>,
    q2: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        Self {
            q1: VecDeque::new(),
            q2: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {}

    fn pop(&self) -> i32 {
        0
    }

    fn top(&self) -> i32 {
        0
    }

    fn empty(&self) -> bool {
        false
    }
}
