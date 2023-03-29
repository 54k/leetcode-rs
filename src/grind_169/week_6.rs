use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn dfs_approach(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, height: usize, levels: &mut Vec<Vec<i32>>) {
            if let Some(r) = root {
                if levels.len() == height {
                    levels.push(vec![]);
                }
                dfs(r.as_ref().borrow().left.clone(), height + 1, levels);
                dfs(r.as_ref().borrow().right.clone(), height + 1, levels);
                levels[height].push(r.as_ref().borrow().val);
            }
        }
        dfs(root, 0, &mut ans);
        for i in 0..ans.len() {
            if i % 2 == 1 {
                ans[i].reverse();
            }
        }
        ans
    }
    fn bfs_approach(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut ans = vec![];
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }
        while !queue.is_empty() {
            ans.push(vec![]);
            let mut n = queue.len();
            while n > 0 {
                if let Some(Some(r)) = queue.pop_front() {
                    ans.last_mut().unwrap().push(r.as_ref().borrow().val);
                    let left = r.as_ref().borrow().left.clone();
                    if left.is_some() {
                        queue.push_back(left);
                    }
                    let right = r.as_ref().borrow().right.clone();
                    if right.is_some() {
                        queue.push_back(right);
                    }
                }
                n -= 1;
            }
        }
        for i in 0..ans.len() {
            if i % 2 == 1 {
                ans[i].reverse();
            }
        }
        ans
    }
    bfs_approach(root)
}

// https://leetcode.com/problems/path-sum-iii/description/
pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
    fn optimized_approach(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            cur_path: &mut Vec<i64>,
            cur_sum: i64,
            target_sum: i64,
            path_count: &mut i32,
        ) {
            if let Some(r) = root {
                let r = r.as_ref().borrow();
                cur_path.push(r.val as i64);
                let mut total_sum = cur_sum + r.val as i64;
                if total_sum == target_sum {
                    *path_count += 1;
                }
                for i in 0..cur_path.len() - 1 {
                    total_sum -= cur_path[i];
                    if total_sum == target_sum {
                        *path_count += 1;
                    }
                }
                dfs(
                    r.left.clone(),
                    cur_path,
                    cur_sum + r.val as i64,
                    target_sum,
                    path_count,
                );
                dfs(
                    r.right.clone(),
                    cur_path,
                    cur_sum + r.val as i64,
                    target_sum,
                    path_count,
                );
                cur_path.pop();
            }
        }
        let mut cur_path = vec![];
        let mut path_count = 0;
        dfs(root, &mut cur_path, 0, target_sum as i64, &mut path_count);
        path_count
    }
    fn brute_force_approach(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        if root.is_none() {
            return 0;
        }
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            cur_sum: i64,
            target_sum: i64,
            path_count: &mut i32,
        ) {
            if let Some(r) = root {
                let r = r.as_ref().borrow();
                let sum = cur_sum + r.val as i64;
                if sum == target_sum {
                    *path_count += 1;
                }
                dfs(r.left.clone(), sum, target_sum, path_count);
                dfs(r.right.clone(), sum, target_sum, path_count);
            }
        }
        let mut path_count = 0;
        dfs(root.clone(), 0, target_sum as i64, &mut path_count);
        path_count += brute_force_approach(
            root.clone().and_then(|x| x.as_ref().borrow().left.clone()),
            target_sum,
        );
        path_count += brute_force_approach(
            root.and_then(|x| x.as_ref().borrow().right.clone()),
            target_sum,
        );
        path_count
    }
    brute_force_approach(root, target_sum)
}

// https://leetcode.com/problems/powx-n/
pub fn my_pow(x: f64, n: i32) -> f64 {
    fn rec(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        let u = rec(x, n / 2);
        if n % 2 == 0 {
            u * u
        } else {
            u * u * x
        }
    }
    let x = rec(x, n.abs());
    if n < 0 {
        1.0 / x
    } else {
        x
    }
}

// https://leetcode.com/problems/search-a-2d-matrix/
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    fn search_row(matrix: &Vec<Vec<i32>>, target: i32) -> usize {
        let mut lo = 0;
        let mut hi = matrix.len() - 1;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if matrix[mid][matrix[0].len() - 1] >= target {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
    fn search_col(matrix: &[i32], target: i32) -> i32 {
        let mut lo = 0i32;
        let mut hi = matrix.len() as i32 - 1;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if matrix[mid as usize] == target {
                return mid as i32;
            } else if matrix[mid as usize] < target {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        -1
    }
    let row_idx = search_row(&matrix, target);
    let col_idx = search_col(&matrix[row_idx], target);
    col_idx >= 0
}

// https://leetcode.com/problems/largest-number/
// https://leetcode.com/problems/largest-number/editorial/
pub fn largest_number(nums: Vec<i32>) -> String {
    let mut nums = nums.into_iter().map(|x| x.to_string()).collect::<Vec<_>>();
    nums.sort_by(|a, b| {
        let o1 = format!("{}{}", a, b);
        let o2 = format!("{}{}", b, a);
        o1.cmp(&o2)
    });
    nums.reverse();
    if nums[0] == "0" {
        return "0".to_string();
    }
    nums.join("")
}

// https://leetcode.com/problems/decode-ways/description/
// https://leetcode.com/problems/decode-ways/solutions/30451/evolve-from-recursion-to-dp/
pub fn num_decodings(s: String) -> i32 {
    fn top_down_approach(s: String) -> i32 {
        fn rec(i: usize, s: &Vec<char>, memo: &mut Vec<i32>) -> i32 {
            if i == s.len() {
                return 1;
            }
            if s[i] == '0' {
                return 0;
            }
            if memo[i] == -1 {
                let mut res = 0;
                res += rec(i + 1, s, memo);
                if i < s.len() - 1
                    && (s[i] == '1' || (s[i] == '2' && ('0'..='6').contains(&s[i + 1])))
                {
                    res += rec(i + 2, s, memo);
                }
                memo[i] = res
            }
            memo[i]
        }
        let mut memo = vec![-1; s.len()];
        rec(0, &s.chars().collect(), &mut memo)
    }
    fn bottom_up_approach(s: String) -> i32 {
        let mut dp1 = 1;
        let mut dp2 = 0;
        let s = s.chars().collect::<Vec<_>>();
        for i in (0..s.len()).rev() {
            let mut dp = if s[i] == '0' { 0 } else { dp1 };
            if i < s.len() - 1 && (s[i] == '1' || (s[i] == '2' && ('0'..='6').contains(&s[i + 1])))
            {
                dp += dp2;
            }
            dp2 = dp1;
            dp1 = dp;
        }
        dp1
    }
    bottom_up_approach(s)
}

// https://leetcode.com/problems/reverse-integer/description/
// https://leetcode.com/problems/reverse-integer/editorial/
pub fn reverse(mut x: i32) -> i32 {
    let mut rev = 0;
    while x != 0 {
        let pop = x % 10;
        x /= 10;
        if rev > i32::MAX / 10 || (rev == i32::MAX / 10 && pop > 7) {
            return 0;
        }
        if rev < i32::MIN / 10 || (rev == i32::MIN / 10 && pop < -8) {
            return 0;
        }
        rev = 10 * rev + pop;
    }
    rev
}

// https://leetcode.com/problems/set-matrix-zeroes/
// https://leetcode.com/problems/set-matrix-zeroes/editorial/
pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    fn using_additional_memory(matrix: &mut Vec<Vec<i32>>) {
        let mut rows = vec![false; matrix.len()];
        let mut cols = vec![false; matrix[0].len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == 0 {
                    rows[i] = true;
                    cols[j] = true;
                }
            }
        }
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if rows[i] || cols[j] {
                    matrix[i][j] = 0;
                }
            }
        }
    }
    fn using_constant_memory(matrix: &mut Vec<Vec<i32>>) {
        let mut col = false;
        for i in 0..matrix.len() {
            if matrix[i][0] == 0 {
                col = true
            }
            for j in 1..matrix[0].len() {
                if matrix[i][j] == 0 {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for i in 1..matrix.len() {
            for j in 1..matrix[0].len() {
                if matrix[i][0] == 0 || matrix[0][j] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        if matrix[0][0] == 0 {
            for i in 0..matrix[0].len() {
                matrix[0][i] = 0;
            }
        }

        if col {
            for i in 0..matrix.len() {
                matrix[i][0] = 0;
            }
        }
    }
    using_constant_memory(matrix)
}

// https://leetcode.com/problems/reorder-list/
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    fn using_stack(head: &mut Option<Box<ListNode>>) {
        let mut cur = head.take();
        let mut stack = vec![];
        while let Some(mut n) = cur.take() {
            cur = n.next.take();
            stack.push(Some(n));
        }

        let mut new_head = None;
        let mut new_tail = &mut new_head;

        let len = stack.len();
        let half = if len % 2 == 1 { len / 2 } else { (len - 1) / 2 };
        for i in 0..=half {
            let mut h = stack[i].take();
            let t = stack[len - 1 - i].take();
            h = h.map(|mut x| {
                x.next = t;
                x
            });
            let node = new_tail.insert(h.take().unwrap());
            if node.next.is_some() {
                new_tail = &mut node.next.as_mut().unwrap().next;
            } else {
                new_tail = &mut node.next;
            }
        }
        *head = new_head;
    }
    fn using_recursion(head: &mut Option<Box<ListNode>>) {
        fn weave(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if head.is_none() || head.as_mut().unwrap().next.is_none() {
                return head;
            }
            let mut tail = &mut head;
            while tail.is_some() && tail.as_mut().unwrap().next.is_some() {
                tail = &mut tail.as_mut().unwrap().next;
            }
            let tail = tail.take();
            let ret = weave(head.as_mut().unwrap().next.take());
            head.map(|mut x| {
                x.next = tail.map(|mut x| {
                    x.next = ret;
                    x
                });
                x
            })
        }
        *head = weave(head.take());
    }
    using_recursion(head)
}

// https://leetcode.com/problems/cheapest-flights-within-k-stops/description/
// https://leetcode.com/problems/cheapest-flights-within-k-stops/editorial/
pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    fn using_bfs_approach(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        use std::collections::VecDeque;
        let mut adj = vec![vec![]; n as usize];
        for flight in flights {
            adj[flight[0] as usize].push((flight[1] as usize, flight[2]));
        }
        let mut lvl = 0;
        let mut dist = vec![i32::MAX; n as usize];
        let mut queue = VecDeque::new();
        queue.push_back((src as usize, 0));
        while !queue.is_empty() && lvl <= k {
            let mut n = queue.len();
            while n > 0 {
                n -= 1;
                let (from, from_dist) = queue.pop_front().unwrap();
                for (to, to_dist) in &adj[from] {
                    if from_dist + to_dist < dist[*to] {
                        dist[*to] = from_dist + to_dist;
                        queue.push_back((*to, dist[*to]));
                    }
                }
            }
            lvl += 1;
        }
        if dist[dst as usize] == i32::MAX {
            -1
        } else {
            dist[dst as usize]
        }
    }
    fn using_bellman_ford_approach(
        n: i32,
        flights: Vec<Vec<i32>>,
        src: i32,
        dst: i32,
        k: i32,
    ) -> i32 {
        let mut dist = vec![i32::MAX; n as usize];
        dist[src as usize] = 0;
        for _ in 0..=k {
            let mut tmp = dist.clone();
            for flight in &flights {
                if dist[flight[0] as usize] != i32::MAX {
                    tmp[flight[1] as usize] =
                        tmp[flight[1] as usize].min(dist[flight[0] as usize] + flight[2]);
                }
            }
            dist = tmp;
        }
        if dist[dst as usize] == i32::MAX {
            -1
        } else {
            dist[dst as usize]
        }
    }
    fn using_dijkstra_approach(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut adj = vec![vec![]; n as usize];
        for flight in flights {
            adj[flight[0] as usize].push((flight[1] as usize, flight[2]));
        }
        let mut stops = vec![i32::MAX; n as usize];
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0, src as usize, 0)));
        while !queue.is_empty() {
            let Reverse((dist, from, steps)) = queue.pop().unwrap();
            if steps > stops[from] || steps > k + 1 {
                continue;
            }
            stops[from] = steps;
            if from as i32 == dst {
                return dist;
            }
            for (to, to_dist) in &adj[from] {
                queue.push(Reverse((dist + to_dist, *to, steps + 1)));
            }
        }
        -1
    }
    using_dijkstra_approach(n, flights, src, dst, k)
}

// https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/
// https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/editorial/
pub fn distance_k(
    root: Option<Rc<RefCell<TreeNode>>>,
    target: Option<Rc<RefCell<TreeNode>>>,
    k: i32,
) -> Vec<i32> {
    fn percolate_distance_approach(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        fn subtree_add(root: Option<Rc<RefCell<TreeNode>>>, dist: i32, k: i32, ans: &mut Vec<i32>) {
            if let Some(r) = root {
                let r = r.borrow();
                if dist == k {
                    ans.push(r.val);
                } else {
                    subtree_add(r.left.clone(), dist + 1, k, ans);
                    subtree_add(r.right.clone(), dist + 1, k, ans);
                }
            }
        }
        // Return vertex distance from node to target if exists, else -1
        // Vertex distance: the number of vertices on the path from node to target
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            target: Option<Rc<RefCell<TreeNode>>>,
            k: i32,
            ans: &mut Vec<i32>,
        ) -> i32 {
            if root.is_none() {
                return -1;
            }

            if root == target {
                subtree_add(root, 0, k, ans);
                1
            } else {
                let l = dfs(
                    root.as_ref().unwrap().borrow().left.clone(),
                    target.clone(),
                    k,
                    ans,
                );
                let r = dfs(
                    root.as_ref().unwrap().borrow().right.clone(),
                    target,
                    k,
                    ans,
                );

                let root = root.as_ref().unwrap().borrow();
                if l != -1 {
                    if l == k {
                        ans.push(root.val);
                    }
                    subtree_add(root.right.clone(), l + 1, k, ans);
                    l + 1
                } else if r != -1 {
                    if r == k {
                        ans.push(root.val);
                    }
                    subtree_add(root.left.clone(), r + 1, k, ans);
                    r + 1
                } else {
                    -1
                }
            }
        }
        let mut ans = vec![];
        dfs(root, target, k, &mut ans);
        ans
    }
    percolate_distance_approach(root, target, k)
}

// https://leetcode.com/problems/3sum-closest/
pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    let mut ans = nums[0] + nums[1] + nums[nums.len() - 1];
    nums.sort();
    for i in 0..nums.len() - 2 {
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if sum > target {
                right -= 1;
            } else {
                left += 1;
            }
            if (sum - target).abs() < (ans - target).abs() {
                ans = sum;
            }
        }
    }
    ans
}

// https://leetcode.com/problems/rotate-list/description/
pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() || k == 0 {
        return head;
    }
    fn reverse(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, i32) {
        let mut len = 0;
        let mut prev = None;
        while let Some(mut h) = head {
            let next = h.next.take();
            h.next = prev;
            prev = Some(h);
            head = next;
            len += 1;
        }
        (prev, len)
    }
    fn split(
        mut head: Option<Box<ListNode>>,
        mut at: i32,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut pre_tail = &mut head;
        while at > 1 {
            if let Some(pt) = pre_tail {
                pre_tail = &mut pt.next;
            }
            at -= 1;
        }
        let tail = pre_tail.as_mut().unwrap().next.take();
        (head, tail)
    }

    let (head, len) = reverse(head);

    let at = k % len;
    if at == 0 {
        return reverse(head).0;
    }

    let (head, tail) = split(head, at);
    let (mut head, _) = reverse(head);
    let (tail, _) = reverse(tail);

    let mut head_tail = &mut head;
    while let Some(ht) = head_tail {
        head_tail = &mut ht.next;
    }
    if let Some(tail) = tail {
        let _ = head_tail.insert(tail);
    }
    head
}

// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/description/
// https://leetcode.com/problems/find-minimum-in-rotated-sorted-array/editorial/
pub fn find_min(nums: Vec<i32>) -> i32 {
    if nums[0] < nums[nums.len() - 1] {
        return nums[0];
    }
    let mut lo = 0;
    let mut hi = nums.len() - 1;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if nums[mid] < nums[0] {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    nums[lo]
}

// https://leetcode.com/problems/basic-calculator-ii/description/
// https://leetcode.com/problems/basic-calculator-ii/editorial/
pub fn calculate(s: String) -> i32 {
    fn stack_approach(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut stack = vec![];
        let mut cur_num = 0;
        let mut operation = '+';
        for i in 0..s.len() {
            if s[i].is_ascii_digit() {
                cur_num = cur_num * 10 + s[i].to_digit(10).unwrap() as i32;
            }
            if !s[i].is_ascii_digit() && s[i] != ' ' || i == s.len() - 1 {
                if operation == '+' {
                    stack.push(cur_num);
                } else if operation == '-' {
                    stack.push(-cur_num);
                } else if operation == '*' {
                    let top = stack.pop().unwrap();
                    stack.push(top * cur_num);
                } else if operation == '/' {
                    let top = stack.pop().unwrap();
                    stack.push(top / cur_num);
                }
                cur_num = 0;
                operation = s[i];
            }
        }
        let mut result = 0;
        while let Some(num) = stack.pop() {
            result += num;
        }
        result
    }
    fn optimized_approach(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut last_num = 0;
        let mut result = 0;
        let mut cur_num = 0;
        let mut operation = '+';
        for i in 0..s.len() {
            if s[i].is_ascii_digit() {
                cur_num = cur_num * 10 + s[i].to_digit(10).unwrap() as i32;
            }
            if !s[i].is_ascii_digit() && s[i] != ' ' || i == s.len() - 1 {
                if operation == '+' || operation == '-' {
                    result += last_num;
                    last_num = if operation == '+' { cur_num } else { -cur_num };
                } else if operation == '*' {
                    last_num *= cur_num;
                } else if operation == '/' {
                    last_num /= cur_num;
                }
                cur_num = 0;
                operation = s[i];
            }
        }
        result += last_num;
        result
    }
    optimized_approach(s)
}

// https://leetcode.com/problems/combination-sum-iv/description/
pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    const MAX_SUM: usize = 1000 * 200 + 1;
    let mut dp = vec![0; MAX_SUM];
    dp[0] = 1;
    for i in 0..=target as usize {
        for j in 0..nums.len() {
            if i as i32 >= nums[j] {
                dp[i] += dp[i - nums[j] as usize];
            }
        }
    }
    dp[target as usize]
}

// https://leetcode.com/problems/insert-delete-getrandom-o1/
use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::HashMap;

struct RandomizedSet {
    buf: Vec<i32>,
    val_to_idx: HashMap<i32, usize>,
    rnd: ThreadRng,
}
impl RandomizedSet {
    fn new() -> Self {
        Self {
            buf: vec![],
            val_to_idx: HashMap::new(),
            rnd: rand::thread_rng(),
        }
    }
    fn insert(&mut self, val: i32) -> bool {
        if self.val_to_idx.contains_key(&val) {
            return false;
        }
        self.buf.push(val);
        self.val_to_idx.insert(val, self.buf.len() - 1);
        true
    }
    fn remove(&mut self, val: i32) -> bool {
        if self.buf.is_empty() || !self.val_to_idx.contains_key(&val) {
            return false;
        }
        let len = self.buf.len();
        let idx = self.val_to_idx.remove(&val).unwrap();
        if idx == len - 1 {
            self.buf.pop();
            return true;
        }
        self.buf.swap(idx, len - 1);
        self.buf.pop();
        self.val_to_idx.insert(self.buf[idx], idx);
        true
    }
    fn get_random(&mut self) -> i32 {
        self.buf[self.rnd.gen::<usize>() % self.buf.len()]
    }
}

// https://leetcode.com/problems/non-overlapping-intervals/description/
pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zigzag_level_order() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{:?}", zigzag_level_order(root));
    }

    #[test]
    fn test_path_sum() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 10,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: -2,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: -3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{}", path_sum(root, 8)); // 3
    }

    #[test]
    fn test_my_pow() {
        println!("{}", my_pow(2.00000, 10));
    }

    #[test]
    fn test_search_matrix() {
        println!("{}", search_matrix(vec![vec![1]], 1)); // true
        println!("{}", search_matrix(vec![vec![1], vec![3]], 1)); // true

        println!(
            "{}",
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3
            )
        ); // true

        println!(
            "{}",
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13
            )
        ); // false
    }

    #[test]
    fn test_largest_number() {
        println!("{}", largest_number(vec![3, 30, 34, 5, 9])); // 9534330
    }

    #[test]
    fn test_num_decodings() {
        println!("{}", num_decodings("12".to_string())); // 2
        println!("{}", num_decodings("226".to_string())); // 3
        println!("{}", num_decodings("226".to_string())); // 3
        println!("{}", num_decodings("06".to_string())); // 0
        println!("{}", num_decodings("10".to_string())); // 1
        println!("{}", num_decodings("2611055971756562".to_string())); // 4
    }

    #[test]
    fn test_reverse() {
        println!("{}", reverse(123)); // 321
        println!("{}", reverse(-123)); // -321
        println!("{}", reverse(120)); // 21
    }

    #[test]
    fn test_set_zeroes() {
        let mut mat = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        set_zeroes(&mut mat);
        println!("{:?}", mat); // [[1,0,1],[0,0,0],[1,0,1]]
    }

    #[test]
    fn test_reorder_list() {
        let mut head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
        reorder_list(&mut head);
        println!("{:?}", head);

        let mut head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None })),
                })),
            })),
        }));
        reorder_list(&mut head);
        println!("{:?}", head);
    }

    #[test]
    fn test_find_cheapest_price() {
        println!(
            "{}",
            find_cheapest_price(
                4,
                vec![
                    vec![0, 1, 100],
                    vec![1, 2, 100],
                    vec![2, 0, 100],
                    vec![1, 3, 600],
                    vec![2, 3, 200]
                ],
                0,
                3,
                1
            )
        ); // 700
    }

    #[test]
    fn test_distance_k() {
        let target = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: target.clone(),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{:?}", distance_k(root, target, 2));
    }

    #[test]
    fn test_three_sum_closest() {
        println!("{}", three_sum_closest(vec![-1, 2, 1, -4], 1)); // 2
        println!("{}", three_sum_closest(vec![0, 0, 0], 1)); // 0
    }

    #[test]
    fn tst_rotate_right() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
        println!("{:?}", rotate_right(head, 2));
        println!("{:?}", rotate_right(None, 0));

        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));
        println!("{:?}", rotate_right(head, 2));
    }

    #[test]
    fn test_find_min() {
        println!("{}", find_min(vec![3, 4, 5, 1, 2])); // 1
        println!("{}", find_min(vec![4, 5, 6, 7, 0, 1, 2])); // 0
    }

    #[test]
    fn test_calculate() {
        println!("{}", calculate("3+2*2".to_string())); // 7
        println!("{}", calculate(" 3/2 ".to_string())); // 1
        println!("{}", calculate(" 3+5 / 2 ".to_string())); // 5
    }

    #[test]
    fn test_combination_sum4() {
        println!("{}", combination_sum4(vec![1, 2, 3], 4)); // 7
        println!("{}", combination_sum4(vec![9], 3)); // 0
    }

    #[test]
    fn test_randomized_set() {
        let mut rs = RandomizedSet::new();
        println!("{}", rs.remove(0));
        println!("{}", rs.remove(0));
        println!("{}", rs.insert(0));
        println!("{}", rs.get_random());
        println!("{}", rs.remove(0));
        println!("{}", rs.insert(0));

        let mut rs = RandomizedSet::new();
        println!("{}", rs.insert(0));
        println!("{}", rs.insert(1));
        println!("{}", rs.remove(0));
        println!("{}", rs.insert(2));
        println!("{}", rs.remove(1));
        println!("{}", rs.get_random());
    }

    #[test]
    fn test_erase_overlap_intervals() {
        println!(
            "{}",
            erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]])
        ); // 1

        println!(
            "{}",
            erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]])
        ); // 2

        println!("{}", erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]])); // 0
    }
}
