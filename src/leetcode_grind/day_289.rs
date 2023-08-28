// https://leetcode.com/problems/implement-stack-using-queues/description/
mod stack_2_queues {
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
}

mod stack_1_queue {
    use std::collections::VecDeque;
    struct MyStack {
        queue: VecDeque<i32>,
    }

    impl MyStack {
        fn new() -> Self {
            Self {
                queue: VecDeque::new(),
            }
        }

        fn push(&mut self, x: i32) {
            self.queue.push_back(x);
            let mut sz = self.queue.len();
            while sz > 1 {
                let f = self.queue.pop_front().unwrap();
                self.queue.push_back(f);
                sz -= 1;
            }
        }

        fn pop(&mut self) -> i32 {
            self.queue.pop_front().unwrap()
        }

        fn top(&self) -> i32 {
            *self.queue.front().unwrap()
        }

        fn empty(&self) -> bool {
            self.queue.is_empty()
        }
    }
}
