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

pub fn mct_from_leaf_values_iii(arr: Vec<i32>) -> i32 {
    let mut arr = arr;
    let mut res = 0;
    while arr.len() > 1 {
        let i = arr
            .iter()
            .enumerate()
            .map(|(i, x)| (*x, i))
            .min()
            .unwrap()
            .1;
        let (mut l, mut r) = (i32::MAX, i32::MAX);
        if i > 0 {
            l = arr[i - 1];
        }
        if i < arr.len() - 1 {
            r = arr[i + 1];
        }
        res += l.min(r) * arr[i];
        arr.remove(i);
    }
    res
}

pub fn mct_from_leaf_values_iv(arr: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut stack = vec![];
    stack.push(i32::MAX);
    for a in arr {
        while *stack.last().unwrap() <= a {
            let mid = stack.pop().unwrap();
            res += mid * (*stack.last().unwrap()).min(a);
        }
        stack.push(a);
    }
    while stack.len() > 2 {
        res += stack.pop().unwrap() * (*stack.last().unwrap());
    }
    res
}

#[test]
fn test_mct_from_leaf_value() {
    let ans = mct_from_leaf_values_i(vec![6, 2, 4]);
    println!("{ans}"); // 32

    let ans = mct_from_leaf_values_ii(vec![6, 2, 4]);
    println!("{ans}"); // 32
}

// https://leetcode.com/problems/next-greater-element-ii/description/
pub fn next_greater_elements_i(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![-1; nums.len()];

    let mut double_nums = vec![];
    double_nums.extend(nums.iter());
    double_nums.extend(nums.iter());

    for i in 0..nums.len() {
        res[i] = -1;

        for j in i + 1..double_nums.len() {
            if double_nums[j] > double_nums[i] {
                res[i] = double_nums[j];
                break;
            }
        }
    }

    res
}

pub fn next_greater_elements_ii(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut res = vec![-1; nums.len()];
    for i in 0..nums.len() {
        res[i] = -1;
        for j in 1..nums.len() {
            if nums[(j + i) % n] > nums[i] {
                res[i] = nums[(j + i) % n];
                break;
            }
        }
    }
    res
}

pub fn next_greater_elements_iii(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![-1; nums.len()];
    let mut stack = vec![];
    for i in (0..2 * nums.len()).rev() {
        while !stack.is_empty() && nums[stack[stack.len() - 1]] <= nums[i % nums.len()] {
            stack.pop();
        }
        if !stack.is_empty() {
            res[i % nums.len()] = nums[*stack.last().unwrap()];
        }
        stack.push(i % nums.len());
    }
    res
}

// https://leetcode.com/problems/next-greater-element-i/description/
pub fn next_greater_element_i_i(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut next_greater = HashMap::new();
    let mut stack = vec![];
    for &n in nums2.iter().rev() {
        while stack.len() > 0 && stack[stack.len() - 1] <= n {
            stack.pop();
        }
        if stack.len() > 0 {
            next_greater.insert(n, stack[stack.len() - 1]);
        } else {
            next_greater.insert(n, -1);
        }
        stack.push(n);
    }
    let mut ans = vec![];
    for n in nums1 {
        ans.push(next_greater[&n]);
    }
    ans
}

pub fn next_greater_element_i_ii(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut stack = vec![];

    for n in nums2 {
        while stack.len() > 0 && stack[stack.len() - 1] < n {
            map.insert(stack.pop().unwrap(), n);
        }
        stack.push(n);
    }
    let mut res = vec![];
    for n in nums1 {
        if map.contains_key(&n) {
            res.push(map[&n]);
        } else {
            res.push(-1);
        }
    }
    res
}
