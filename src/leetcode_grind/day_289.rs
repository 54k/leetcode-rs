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

// https://leetcode.com/problems/minimum-cost-tree-from-leaf-values/
pub fn mct_from_leaf_values_i(arr: Vec<i32>) -> i32 {
    fn rec(i: usize, j: usize, arr: &Vec<i32>, memo: &mut Vec<Vec<i32>>) -> i32 {
        if i == j {
            return 0;
        }
        if memo[i][j] != -1 {
            return memo[i][j];
        }
        let mut ans = i32::MAX;
        for k in i..j {
            ans = ans.min(
                arr[i..=k].iter().max().unwrap() * arr[k + 1..=j].iter().max().unwrap()
                    + rec(i, k, arr, memo)
                    + rec(k + 1, j, arr, memo),
            );
        }
        memo[i][j] = ans;
        ans
    }
    let mut memo = vec![vec![-1; arr.len()]; arr.len()];
    rec(0, arr.len() - 1, &arr, &mut memo)
}

pub fn mct_from_leaf_values_ii(arr: Vec<i32>) -> i32 {
    let mut dp = vec![vec![0; arr.len()]; arr.len()];
    for len in 1..arr.len() {
        for i in 0..=arr.len() - len - 1 {
            let j = i + len;

            let mut ans = i32::MAX;
            for k in i..j {
                ans = ans.min(
                    arr[i..=k].iter().max().unwrap() * arr[k + 1..=j].iter().max().unwrap()
                        + dp[i][k]
                        + dp[k + 1][j],
                );
            }
            dp[i][j] = ans;
        }
    }
    dp[0][arr.len() - 1]
}

#[test]
fn test_mct_from_leaf_value() {
    let ans = mct_from_leaf_values_i(vec![6, 2, 4]);
    println!("{ans}"); // 32

    let ans = mct_from_leaf_values_ii(vec![6, 2, 4]);
    println!("{ans}"); // 32
}
