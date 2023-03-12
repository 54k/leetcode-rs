use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/move-zeroes/
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut j = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(i, j);
            j += 1;
        }
    }
}

// https://leetcode.com/problems/symmetric-tree/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn is_symmetric(l: Option<Rc<RefCell<TreeNode>>>, r: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if l.is_none() && r.is_none() {
            true
        } else if l.is_some() && r.is_some() {
            let l = l.unwrap();
            let l = l.borrow();
            let r = r.unwrap();
            let r = r.borrow();

            l.val == r.val
                && is_symmetric(l.right.clone(), r.left.clone())
                && is_symmetric(l.left.clone(), r.right.clone())
        } else {
            false
        }
    }
    is_symmetric(root.clone(), root)
}

// https://leetcode.com/problems/missing-number/description/
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut arithmetic_sum = n as i32 * (n as i32 + 1) / 2;
    for x in nums {
        arithmetic_sum -= x;
    }
    arithmetic_sum
}

// https://leetcode.com/problems/palindrome-number/
// https://leetcode.com/problems/palindrome-number/editorial/
pub fn is_palindrome(mut x: i32) -> bool {
    // Special cases:
    // As discussed above, when x < 0, x is not a palindrome.
    // Also if the last digit of the number is 0, in order to be a palindrome,
    // the first digit of the number also needs to be 0.
    // Only 0 satisfy this property.
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    let mut n = 0;
    while x > n {
        n = n * 10 + x % 10;
        x /= 10;
    }
    n == x || n / 10 == x
}

// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/
pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn rec(nums: &Vec<i32>, lo: i32, hi: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if lo > hi {
            return None;
        }
        let m = (lo + hi) / 2;
        let mut mid = TreeNode::new(nums[m as usize]);
        mid.left = rec(nums, lo, m - 1);
        mid.right = rec(nums, m + 1, hi);
        Some(Rc::new(RefCell::new(mid)))
    }
    rec(&nums, 0, nums.len() as i32 - 1)
}

// https://leetcode.com/problems/reverse-bits/
pub fn reverse_bits(x: u32) -> u32 {
    // x.reverse_bits() cheat lol
    let mut ans = 0u32;
    for i in 0..=31 {
        let bit_left = x >> i & 1;
        let bit_pos = 31 - i;
        ans |= bit_left << bit_pos;
    }
    ans
}

// https://leetcode.com/problems/subtree-of-another-tree/description/
pub fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    fn is_symmetric(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root.is_none() && sub_root.is_none() {
            true
        } else if root.is_some() && sub_root.is_none() || root.is_none() && sub_root.is_some() {
            false
        } else {
            let r = root.unwrap();
            let r = r.borrow();
            let sr = sub_root.unwrap();
            let sr = sr.borrow();

            r.val == sr.val
                && is_symmetric(r.left.clone(), sr.left.clone())
                && is_symmetric(r.right.clone(), sr.right.clone())
        }
    }

    fn find(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() || sub_root.is_none() {
            false
        } else if root.is_some() && sub_root.is_some() {
            let r = root.clone().unwrap();
            let r = r.borrow();
            if r.val == sub_root.as_ref().unwrap().borrow().val
                && is_symmetric(root, sub_root.clone())
            {
                true
            } else {
                find(r.left.clone(), sub_root.clone()) || find(r.right.clone(), sub_root)
            }
        } else {
            false
        }
    }

    find(root, sub_root)
}

// https://leetcode.com/problems/squares-of-a-sorted-array/description/
// Follow up: Squaring each element and sorting the new array is very trivial,
// could you find an O(n) solution using a different approach?
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    fn approach1(nums: Vec<i32>) -> Vec<i32> {
        let mut l_half = vec![];
        let mut r_half = vec![];
        for n in nums {
            if n < 0 {
                l_half.push(n * n);
            } else {
                r_half.push(n * n);
            }
        }
        let mut ans = vec![];
        let mut i = l_half.len() as i32 - 1;
        let mut j = 0;
        while i >= 0 || j < r_half.len() as i32 {
            if i >= 0 && j < r_half.len() as i32 {
                if l_half[i as usize] < r_half[j as usize] {
                    ans.push(l_half[i as usize]);
                    i -= 1;
                } else {
                    ans.push(r_half[j as usize]);
                    j += 1;
                }
            } else if i >= 0 {
                ans.push(l_half[i as usize]);
                i -= 1;
            } else {
                ans.push(r_half[j as usize]);
                j += 1;
            }
        }
        ans
    }

    fn approach2(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let mut i = 0;
        let mut j = nums.len() - 1;
        let mut k = j as i32;
        while k >= 0 {
            if nums[i].abs() > nums[j].abs() {
                ans.push(nums[i] * nums[i]);
                i += 1;
            } else {
                ans.push(nums[j] * nums[j]);
                j -= 1;
            }
            k -= 1;
        }
        ans.reverse();
        ans
    }

    approach2(nums)
}

// https://leetcode.com/problems/maximum-subarray/
fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut ans = nums[0];
    let mut running_sum = 0;
    for n in nums {
        running_sum += n;
        ans = ans.max(running_sum);
        if running_sum < 0 {
            running_sum = 0;
        }
    }
    ans
}

// https://leetcode.com/problems/insert-interval/
pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    intervals.push(new_interval);
    intervals.sort();

    let mut start = intervals[0][0];
    let mut end = intervals[0][1];

    for i in intervals.into_iter().skip(1) {
        if i[0] <= end {
            end = end.max(i[1]);
        } else {
            ans.push(vec![start, end]);
            start = i[0];
            end = i[1];
        }
    }
    ans.push(vec![start, end]);
    ans
}

pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    fn dp(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut dist = vec![vec![i32::MAX - 10000; mat[0].len()]; mat.len()];
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 0 {
                    dist[i][j] = 0;
                } else {
                    if i > 0 {
                        dist[i][j] = dist[i][j].min(dist[i - 1][j] + 1);
                    }
                    if j > 0 {
                        dist[i][j] = dist[i][j].min(dist[i][j - 1] + 1);
                    }
                }
            }
        }

        for i in (0..mat.len()).rev() {
            for j in (0..mat[0].len()).rev() {
                if i < mat.len() - 1 {
                    dist[i][j] = dist[i][j].min(dist[i + 1][j] + 1);
                }
                if j < mat[0].len() - 1 {
                    dist[i][j] = dist[i][j].min(dist[i][j + 1] + 1);
                }
            }
        }

        dist
    }

    fn bfs(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::*;
        const DIR: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        let mut queue = VecDeque::new();
        let mut dist = vec![vec![i32::MAX; mat[0].len()]; mat.len()];

        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 0 {
                    dist[i][j] = 0;
                    queue.push_back((i as i32, j as i32));
                }
            }
        }

        while let Some((i, j)) = queue.pop_front() {
            for d in DIR {
                let i1 = i + d.0;
                let j1 = j + d.1;
                if (i1 >= 0 && i1 < mat.len() as i32)
                    && (j1 >= 0 && j1 < mat[0].len() as i32)
                    && dist[i as usize][j as usize] + 1 < dist[i1 as usize][j1 as usize]
                {
                    dist[i1 as usize][j1 as usize] = dist[i as usize][j as usize] + 1;
                    queue.push_back((i1, j1));
                }
            }
        }
        dist
    }

    bfs(mat)
}

// https://leetcode.com/problems/k-closest-points-to-origin/description/
pub fn k_closest(points: Vec<Vec<i32>>, mut k: i32) -> Vec<Vec<i32>> {
    use std::cmp::*;
    use std::collections::*;
    let mut ans = vec![];
    let mut pq = points
        .into_iter()
        .map(|p| Reverse((p[0] * p[0] + p[1] * p[1], vec![p[0], p[1]]))) // (distance, point)
        .collect::<BinaryHeap<Reverse<(i32, Vec<i32>)>>>();
    while k > 0 && !pq.is_empty() {
        ans.push(pq.pop().unwrap().0 .1);
        k -= 1;
    }
    ans
}

// https://leetcode.com/problems/longest-substring-without-repeating-characters/
// https://leetcode.com/problems/longest-substring-without-repeating-characters/editorial/
pub fn length_of_longest_substring(s: String) -> i32 {
    fn using_hashset(s: String) -> i32 {
        use std::collections::*;
        let s = s.chars().collect::<Vec<_>>();
        let mut sliding_window = HashSet::new();
        let mut ans = 0;
        let mut start = 0;
        for end in 0..s.len() {
            while sliding_window.contains(&s[end]) {
                sliding_window.remove(&s[start]);
                start += 1;
            }
            sliding_window.insert(s[end]);
            ans = ans.max(end - start + 1);
        }
        ans.max(s.len() - start) as i32
    }

    fn optimized_sliding_window(s: String) -> i32 {
        use std::collections::HashMap;
        let mut char_to_last_idx = HashMap::new();
        let mut s = s.chars().collect::<Vec<_>>();
        let mut ans = 0;
        let mut left = 0;
        for right in 0..s.len() {
            if char_to_last_idx.contains_key(&s[right]) {
                left = left.max(*char_to_last_idx.get(&s[right]).unwrap());
            }
            ans = ans.max(right - left + 1);
            char_to_last_idx.insert(s[right], right + 1);
        }
        ans as i32
    }

    optimized_sliding_window(s)
}

// https://leetcode.com/problems/3sum/
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn approach_with_set(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::*;
        let mut set = HashSet::new();
        let mut ans = HashSet::new();
        for i in 0..nums.len() - 1 {
            let a = nums[i];
            set.clear();
            for j in i + 1..nums.len() {
                let b = nums[j];
                if set.contains(&-(a + b)) {
                    let c = *set.get(&-(a + b)).unwrap();
                    let mut triplet = vec![a, b, c];
                    triplet.sort();
                    ans.insert(triplet);
                }
                set.insert(b);
            }
        }
        ans.into_iter().collect()
    }

    fn approach_with_two_pointers(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::*;
        nums.sort();
        let mut ans = vec![];
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                match sum.cmp(&0) {
                    Ordering::Greater => {
                        right -= 1;
                    }
                    Ordering::Less => {
                        left += 1;
                    }
                    _ => {
                        ans.push(vec![nums[i], nums[left], nums[right]]);
                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }
                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }
                        left += 1;
                        right -= 1;
                    }
                }
            }
        }
        ans
    }

    approach_with_two_pointers(nums)
}

// https://leetcode.com/problems/binary-tree-level-order-traversal/
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, depth: usize, levels: &mut Vec<Vec<i32>>) {
            if let Some(r) = root {
                if levels.len() == depth {
                    levels.push(vec![]);
                }
                let r = r.borrow();
                levels[depth].push(r.val);
                dfs(r.left.clone(), depth + 1, levels);
                dfs(r.right.clone(), depth + 1, levels);
            }
        }
        let mut levels = vec![];
        dfs(root, 0, &mut levels);
        levels
    }

    fn bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut levels = vec![];
        let mut queue = VecDeque::new();
        if root.is_some() {
            queue.push_back(root);
        }
        let mut lvl = 0;
        while !queue.is_empty() {
            let mut k = queue.len();
            if levels.len() == lvl {
                levels.push(vec![]);
            }
            while k > 0 {
                k -= 1;
                if let Some(Some(n)) = queue.pop_front() {
                    let n = n.borrow();
                    levels[lvl].push(n.val);
                    if n.left.is_some() {
                        queue.push_back(n.left.clone());
                    }
                    if n.right.is_some() {
                        queue.push_back(n.right.clone());
                    }
                }
            }
            lvl += 1;
        }
        levels
    }
    bfs(root)
}

// https://leetcode.com/problems/evaluate-reverse-polish-notation/
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = vec![];
    for t in tokens {
        match t.as_str() {
            "+" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a + b);
            }
            "*" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a * b);
            }
            "-" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a - b);
            }
            "/" => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a / b);
            }
            _ => {
                let mut sign = 1;
                let mut num = 0;
                for ch in t.chars() {
                    match ch {
                        '-' => sign *= -1,
                        _ => num = num * 10 + ch as i32 - '0' as i32,
                    }
                }
                stack.push(sign * num)
            }
        }
    }
    stack.pop().unwrap()
}

// https://leetcode.com/problems/course-schedule/
pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    fn dfs_topological_sort(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        fn dfs(
            v: usize,
            adj: &Vec<Vec<usize>>,
            visited: &mut Vec<u8>,
            courses_order: &mut Vec<usize>,
        ) -> bool {
            if visited[v] == 1 {
                return false;
            }
            visited[v] = 1;
            for &u in &adj[v] {
                if visited[u] != 2 && !dfs(u, adj, visited, courses_order) {
                    return false;
                }
            }
            visited[v] = 2;
            courses_order.push(v);
            true
        }
        let mut visited = vec![0; num_courses as usize];
        let mut adj = vec![vec![]; num_courses as usize];
        for p in prerequisites {
            let (from, to) = (p[0], p[1]);
            adj[from as usize].push(to as usize);
        }
        let mut courses_order = vec![];
        for course in 0..num_courses as usize {
            if visited[course] == 0 && !dfs(course, &adj, &mut visited, &mut courses_order) {
                return false;
            }
        }
        courses_order.reverse();
        println!("{:?}", courses_order);
        true
    }

    fn khan_topological_sort(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        use std::collections::*;
        let mut queue = VecDeque::new();
        let mut in_degrees = vec![0; num_courses as usize];
        let mut adj = vec![vec![]; num_courses as usize];

        for p in prerequisites {
            let (from, to) = (p[0], p[1]);
            in_degrees[to as usize] += 1;
            adj[from as usize].push(to as usize);
        }

        let mut courses_order = vec![];
        for (course, &degree) in in_degrees.iter().enumerate() {
            if degree == 0 {
                queue.push_back(course);
            }
        }

        while let Some(course) = queue.pop_front() {
            courses_order.push(course);
            for &next_course in &adj[course] {
                in_degrees[next_course] -= 1;
                if in_degrees[next_course] == 0 {
                    queue.push_back(next_course);
                }
            }
        }

        println!("{:?}", courses_order);
        courses_order.len() == num_courses as usize
    }
    khan_topological_sort(num_courses, prerequisites)
}

// https://leetcode.com/problems/implement-trie-prefix-tree/
// https://leetcode.com/problems/implement-trie-prefix-tree/editorial/
#[derive(Debug, Clone)]
struct TrieNode {
    links: Vec<Option<TrieNode>>,
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            links: vec![None; 26],
            is_end: false,
        }
    }

    fn contains_key(&self, ch: char) -> bool {
        self.links[ch as usize - 'a' as usize].is_some()
    }

    fn get(&self, ch: char) -> Option<&TrieNode> {
        self.links[ch as usize - 'a' as usize].as_ref()
    }

    fn get_mut(&mut self, ch: char) -> Option<&mut TrieNode> {
        self.links[ch as usize - 'a' as usize].as_mut()
    }

    fn put(&mut self, ch: char, node: TrieNode) {
        self.links[ch as usize - 'a' as usize] = Some(node);
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            if !node.contains_key(ch) {
                node.put(ch, TrieNode::new());
            }
            node = node.get_mut(ch).unwrap();
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for ch in word.chars() {
            if node.contains_key(ch) {
                node = node.get(ch).unwrap();
            } else {
                return false;
            }
        }
        node.is_end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for ch in prefix.chars() {
            if node.contains_key(ch) {
                node = node.get(ch).unwrap();
            } else {
                return false;
            }
        }
        true
    }
}

// https://leetcode.com/problems/coin-change/description/
// https://leetcode.com/problems/coin-change/editorial/
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    fn dp_bottom_up(coins: Vec<i32>, amount: i32) -> i32 {
        let max = amount + 1;
        let mut dp = vec![max; max as usize];
        dp[0] = 0;
        for i in 1..max as usize {
            for j in 0..coins.len() {
                if coins[j] as usize <= i {
                    dp[i] = dp[i].min(dp[i - coins[j] as usize] + 1);
                }
            }
        }
        if dp[amount as usize] > amount {
            -1
        } else {
            dp[amount as usize]
        }
    }
    dp_bottom_up(coins, amount)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_zeroes() {
        let mut v = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut v);
        println!("{:?}", v); // [1,3,12,0,0]
    }

    #[test]
    fn test_is_symmetric() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{}", is_symmetric(root)); // true
    }

    #[test]
    fn test_missing_number() {
        println!("{}", missing_number(vec![3, 0, 1])); // 2
    }

    #[test]
    fn test_is_palindrome() {
        println!("{}", is_palindrome(121)); // true
        println!("{}", is_palindrome(-121)); // false
        println!("{}", is_palindrome(10)); // false
    }

    #[test]
    fn test_sorted_array_to_bst() {
        println!("{:?}", sorted_array_to_bst(vec![-10, -3, 0, 5, 9]));
    }

    #[test]
    fn test_reverse_bits() {
        println!("{}", reverse_bits(4294967293)); // 3221225471
    }

    #[test]
    fn test_is_subtree() {
        let sub = Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: sub.clone(),
        })));

        println!("{}", is_subtree(root, sub)); // true
    }

    #[test]
    fn test_sorted_squares() {
        println!("{:?}", sorted_squares(vec![-4, -1, 0, 3, 10])); // [0,1,9,16,100]
        println!("{:?}", sorted_squares(vec![-7, -3, 2, 3, 11])); // [4,9,9,49,121]
    }

    #[test]
    fn test_max_sub_array() {
        println!("{}", max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])); // 6
        println!("{}", max_sub_array(vec![5, 4, -1, 7, 8])); // 23
        println!("{}", max_sub_array(vec![1])); // 1
    }

    #[test]
    fn test_insert() {
        println!("{:?}", insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]));
    }

    #[test]
    fn test_update_matrix() {
        println!(
            "{:?}",
            update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]])
        ); // [[0,0,0],[0,1,0],[1,2,1]]
    }

    #[test]
    fn test_k_closest() {
        println!("{:?}", k_closest(vec![vec![1, 3], vec![-2, 2]], 1)); // [-2, 2]
        println!(
            "{:?}",
            k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2) // [[3,3],[-2,4]]
        );
    }

    #[test]
    fn test_length_of_longest_substring() {
        println!("{}", length_of_longest_substring("abcabcbb".to_string())); // 3
        println!("{}", length_of_longest_substring("bbbbb".to_string())); // 1
        println!("{}", length_of_longest_substring("pwwkew".to_string())); // 3
    }

    #[test]
    fn test_three_sum() {
        println!("{:?}", three_sum(vec![-2, 0, 0, 2, 2])); // [[-2, 0, 2]]
        println!("{:?}", three_sum(vec![1, -1, -1, 0])); // [[0,0,0]]
        println!("{:?}", three_sum(vec![0, 0, 0, 0])); // [[0,0,0]]
        println!("{:?}", three_sum(vec![-1, 0, 1, 2, -1, -4])); // [[-1,-1,2],[-1,0,1]]
        println!("{:?}", three_sum(vec![0, 1, 1])); // []
    }

    #[test]
    fn test_level_order() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
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

        println!("{:?}", level_order(root));
    }

    #[test]
    fn test_eval_rpn() {
        println!(
            "{}",
            eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ])
        ); // 9

        println!(
            "{}",
            eval_rpn(
                vec!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"]
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect()
            )
        ); // 22
    }

    #[test]
    fn test_can_finish() {
        println!("{}", can_finish(2, vec![vec![1, 0]])); // true
        println!("{}", can_finish(2, vec![vec![1, 0], vec![0, 1]])); // false
    }

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        trie.insert("apple".to_string());
        println!("{}", trie.search("apple".to_string()));
        println!("{}", trie.search("app".to_string()));
        println!("{}", trie.starts_with("app".to_string()));
        trie.insert("app".to_string());
        println!("{}", trie.search("app".to_string()));
    }

    #[test]
    fn test_coin_change() {
        println!("{}", coin_change(vec![1, 2, 5], 11)); // 3
        println!("{}", coin_change(vec![2], 3)); // -1
        println!("{}", coin_change(vec![1], 0)); // 0
    }
}
