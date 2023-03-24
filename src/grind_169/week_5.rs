use crate::grind_169::week_3::find_anagrams;
use std::cell::RefCell;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

// https://leetcode.com/problems/path-sum-ii/description/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    fn using_dfs(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        fn dfs(
            root: Option<Rc<RefCell<TreeNode>>>,
            target_sum: i32,
            path: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
        ) {
            if root.is_none() {
                return;
            }
            let r = root.as_ref().unwrap().borrow();
            let val = r.val;
            path.push(val);
            if r.left.is_none() && r.right.is_none() && target_sum - r.val == 0 {
                ans.push(path.clone());
            }
            dfs(r.left.clone(), target_sum - val, path, ans);
            dfs(r.right.clone(), target_sum - val, path, ans);
            path.pop();
        }
        let mut ans = vec![];
        dfs(root, target_sum, &mut vec![], &mut ans);
        ans
    }

    fn using_bfs(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();
        let mut res = vec![];
        queue.push_back((root, vec![], target_sum));
        while let Some((Some(r), mut path, remaining_sum)) = queue.pop_front() {
            let val = r.as_ref().borrow().val;
            path.push(val);

            let left = r.as_ref().borrow().left.clone();
            let right = r.as_ref().borrow().right.clone();

            if remaining_sum - val == 0 && left.is_none() && right.is_none() {
                res.push(path.clone());
            }

            if left.is_some() {
                queue.push_back((left, path.clone(), remaining_sum - val));
            }
            if right.is_some() {
                queue.push_back((right, path.clone(), remaining_sum - val));
            }
        }
        res
    }

    using_bfs(root, target_sum)
}

// https://leetcode.com/problems/longest-consecutive-sequence/description/
// https://leetcode.com/problems/longest-consecutive-sequence/editorial/
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    fn using_sort(mut nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut max_streak = 1;
        let mut current_streak = 1;
        nums.sort();
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] {
                continue;
            }
            if nums[i] + 1 == nums[i + 1] {
                current_streak += 1;
                max_streak = max_streak.max(current_streak);
            } else {
                current_streak = 1
            }
        }
        max_streak
    }
    fn using_set(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let set = nums.iter().copied().collect::<HashSet<i32>>();
        let mut longest_streak = 0;
        for num in nums {
            if !set.contains(&(num - 1)) {
                let mut current_num = num;
                let mut current_streak = 1;
                while set.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_streak += 1;
                }
                longest_streak = longest_streak.max(current_streak);
            }
        }
        longest_streak
    }
    using_sort(nums)
}

// https://leetcode.com/problems/rotate-array/description/
// https://leetcode.com/problems/rotate-array/solutions/54249/3-line-using-reverse/
fn rotate(nums: &mut Vec<i32>, k: i32) {
    fn short(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let k = k as usize % len;
        nums.reverse();
        nums[0..k].reverse();
        nums[k..].reverse();
    }
    fn using_reverse(nums: &mut Vec<i32>, k: i32) {
        fn reverse(nums: &mut [i32], mut from: i32, mut to: i32) {
            while from < to {
                nums.swap(from as usize, to as usize);
                from += 1;
                to -= 1;
            }
        }
        let len = nums.len() as i32;
        let k = k % len;
        reverse(nums, 0, len - 1);
        reverse(nums, 0, k - 1);
        reverse(nums, k, len - 1);
    }
    using_reverse(nums, k)
}

// https://leetcode.com/problems/odd-even-linked-list/
// https://leetcode.com/problems/odd-even-linked-list/editorial/
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut odd_head = None;
    let mut odd_tail = &mut odd_head;
    let mut even_head = None;
    let mut even_tail = &mut even_head;
    let mut i = 0;
    while let Some(mut node) = head.take() {
        head = node.next.take();
        if i % 2 == 0 {
            odd_tail = &mut odd_tail.insert(node).next;
        } else {
            even_tail = &mut even_tail.insert(node).next;
        }
        i += 1;
    }
    if let Some(eh) = even_head {
        let _ = odd_tail.insert(eh);
    }
    odd_head
}

// https://leetcode.com/problems/decode-string/
// https://leetcode.com/problems/decode-string/solutions/941553/rust-stack-solution/
pub fn decode_string(s: String) -> String {
    fn my_decode_string(s: String) -> String {
        fn rec_parse(s: &Vec<char>, mut i: usize, parts: &mut Vec<String>) {
            if i >= s.len() {
                return;
            }
            match s[i] {
                _ if s[i].is_alphabetic() => {
                    let d_start = i;
                    while i < s.len() && s[i].is_alphabetic() {
                        i += 1;
                    }
                    let string = s[d_start..i].iter().copied().collect::<String>();
                    parts.push(string);
                    rec_parse(s, i, parts);
                }
                _ if s[i].is_ascii_digit() => {
                    let d_start = i;
                    while i < s.len() && s[i].is_ascii_digit() {
                        i += 1;
                    }
                    let repeat_num = s[d_start..i].iter().copied().collect::<String>();
                    parts.push(repeat_num);
                    rec_parse(s, i + 1, parts);
                }
                '[' => {
                    rec_parse(s, i + 1, parts);
                }
                ']' => {
                    let mut string = "".to_string();
                    while parts.last().unwrap().parse::<usize>().is_err() {
                        string = format!("{}{}", parts.pop().unwrap().as_str(), string);
                    }
                    let num = parts.pop().unwrap().parse::<usize>().unwrap();
                    parts.push(string.repeat(num));
                    rec_parse(s, i + 1, parts);
                }
                _ => panic!(),
            }
        }

        let mut parts = vec![];
        let s = s.chars().collect::<Vec<_>>();
        rec_parse(&s, 0, &mut parts);
        parts.join("")
    }
    fn short_decode_string(s: String) -> String {
        let mut stack = vec![];
        let (mut n, mut str) = (0, String::new()); // result and current expression
        for ch in s.chars() {
            match ch {
                '[' => {
                    stack.push((n, str.clone()));
                    n = 0;
                    str.clear();
                }
                ']' => {
                    if let Some(last) = stack.pop() {
                        str = last.1 + str.repeat(last.0).as_str();
                    }
                }
                '0'..='9' => {
                    n = 10 * n + (ch as u8 - b'0') as usize;
                }
                _ => {
                    str.push(ch);
                }
            }
        }
        str
    }
    short_decode_string(s)
}

// https://leetcode.com/problems/contiguous-array/description/
// https://leetcode.com/problems/contiguous-array/editorial/
// TODO revisit
pub fn find_max_length(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut max_len = 0;
    let mut count = 0;
    let mut map = vec![(0, -1)].into_iter().collect::<HashMap<i32, i32>>();
    for i in 0..nums.len() {
        count += if nums[i] == 0 { -1 } else { 1 };
        if let std::collections::hash_map::Entry::Vacant(e) = map.entry(count) {
            e.insert(i as i32);
        } else {
            max_len = max_len.max(i as i32 - map[&count]);
        }
    }
    max_len
}

// https://leetcode.com/problems/maximum-width-of-binary-tree/description/
// https://leetcode.com/problems/maximum-width-of-binary-tree/solutions/106645/c-java-bfs-dfs-3liner-clean-code-with-explanation/
pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn using_dfs_indicies(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs_record_indicies(
            root: Option<Rc<RefCell<TreeNode>>>,
            height: usize,
            idx: usize,
            indicies: &mut Vec<Vec<usize>>,
        ) {
            if height == indicies.len() {
                indicies.push(vec![]);
            }
            if let Some(r) = root {
                indicies[height].push(idx);
                dfs_record_indicies(
                    r.as_ref().borrow().left.clone(),
                    height + 1,
                    idx * 2 + 1,
                    indicies,
                );
                dfs_record_indicies(
                    r.as_ref().borrow().right.clone(),
                    height + 1,
                    idx * 2 + 2,
                    indicies,
                );
            }
        }
        let mut indicies = vec![];
        let mut ans = 0;
        dfs_record_indicies(root, 0, 0, &mut indicies);

        for id in indicies {
            if id.len() > 1 {
                ans = ans.max(id[id.len() - 1] as i32 - id[0] as i32 + 1);
            } else {
                ans = ans.max(1);
            }
        }
        ans
    }
    fn using_bfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let mut queue = vec![(root, 0)]; // node, idx
        while !queue.is_empty() {
            let mut next = vec![];
            let mut min = 0;
            let mut max = 0;

            for (node, idx) in queue {
                if let Some(n) = node {
                    if next.is_empty() {
                        min = idx;
                    }
                    max = idx;
                    next.push((n.as_ref().borrow().left.clone(), idx * 2));
                    next.push((n.as_ref().borrow().right.clone(), idx * 2 + 1));
                }
            }
            ans = ans.max(max - min + 1);
            queue = next;
        }
        ans
    }
    using_bfs(root)
}

// https://leetcode.com/problems/find-k-closest-elements/
// https://leetcode.com/problems/find-k-closest-elements/solutions/106419/o-log-n-java-1-line-o-log-n-k-ruby/
// https://leetcode.com/problems/find-k-closest-elements/solutions/106422/concise-c-solution-5-lines-o-k-logn-time/
pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    // I binary-search for where the resulting elements start in the array.
    // It's the first index i so that arr[i] is better than arr[i+k] (with "better" meaning closer to or equally close to x).
    // Then I just return the k elements starting there.
    let mut lo = 0;
    let mut hi = arr.len() - k as usize;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if x - arr[mid] > arr[mid + k as usize] - x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    arr[lo..hi + k as usize].to_vec()
}

// https://leetcode.com/problems/longest-repeating-character-replacement/description/
// https://leetcode.com/problems/longest-repeating-character-replacement/editorial/
pub fn character_replacement(s: String, k: i32) -> i32 {
    fn using_bin_search(s: String, k: i32) -> i32 {
        fn has_valid_sting_of_len(s: &[char], len: usize, k: usize) -> bool {
            let mut freq = vec![0; 26];
            let mut max_freq = 0;

            let mut start = 0;
            for end in 0..s.len() {
                freq[s[end] as usize - 'A' as usize] += 1;

                if end + 1 - start > len {
                    freq[s[start] as usize - 'A' as usize] -= 1;
                    start += 1;
                }

                max_freq = max_freq.max(freq[s[end] as usize - 'A' as usize]);
                if len - max_freq <= k {
                    return true;
                }
            }
            false
        }
        // binary search over the length of substring
        // lo contains the valid value, and hi contains the
        // invalid value
        let s = s.chars().collect::<Vec<_>>();
        let mut lo = 1usize;
        let mut hi = s.len() + 1;
        while lo + 1 < hi {
            let mid = lo + (hi - lo) / 2;
            if has_valid_sting_of_len(&s, mid, k as usize) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        lo as i32
    }
    fn using_sliding_window_slow(s: String, k: i32) -> i32 {
        use std::collections::HashSet;
        fn is_window_valid(start: i32, end: i32, count: i32, k: i32) -> bool {
            end + 1 - start - count <= k
        }
        let mut ans = 0;
        let s = s.chars().collect::<Vec<_>>();
        let set = s.iter().copied().collect::<HashSet<_>>();
        for ch in set {
            let mut start = 0;
            let mut count = 0;
            // initialize a sliding window for each unique letter
            for end in 0..s.len() {
                if s[end] == ch {
                    count += 1;
                }
                while !is_window_valid(start as i32, end as i32, count, k) {
                    if s[start] == ch {
                        count -= 1;
                    }
                    start += 1;
                }
                ans = ans.max(end as i32 + 1 - start as i32);
            }
        }
        ans
    }
    fn using_sliding_window(s: String, k: i32) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut freq = vec![0; 26];
        let mut max_frequency = 0;
        let mut start = 0;
        let mut longest_substring_len = 0;

        for end in 0..s.len() {
            freq[s[end] as usize - 'A' as usize] += 1;
            max_frequency = max_frequency.max(freq[s[end] as usize - 'A' as usize]);
            if end + 1 - start - max_frequency > k as usize {
                freq[s[start] as usize - 'A' as usize] -= 1;
                start += 1;
            }
            longest_substring_len = end + 1 - start;
        }
        longest_substring_len as i32
    }
    using_sliding_window(s, k)
}

// https://leetcode.com/problems/jump-game/
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut prev = nums[0] as usize;
    let mut cur = prev;
    for i in 1..nums.len() {
        cur = if prev >= i {
            (nums[i] as usize + i).max(prev)
        } else {
            return false;
        };
        prev = cur;
    }
    cur >= nums.len() - 1
}

// https://leetcode.com/problems/add-two-numbers/description/
pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // fn reverse(mut l: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     let mut prev = None;
    //     while let Some(mut cur) = l.take() {
    //         let next = cur.next.take();
    //         cur.next = prev;
    //         prev = Some(cur);
    //         l = next;
    //     }
    //     prev
    // }

    let mut result_head: Option<Box<ListNode>> = None;
    let mut result_tail = &mut result_head;

    // let mut l1 = reverse(l1);
    let mut l1_tail = l1.as_mut();
    // let mut l2 = reverse(l2);
    let mut l2_tail = l2.as_mut();
    let mut carry = 0;
    loop {
        let num1 = if let Some(n) = l1_tail {
            l1_tail = n.next.as_mut();
            n.val
        } else {
            0
        };
        let num2 = if let Some(n) = l2_tail {
            l2_tail = n.next.as_mut();
            n.val
        } else {
            0
        };

        let sum = num1 + num2 + carry;
        let x = result_tail.insert(Box::new(ListNode {
            val: sum % 10,
            next: None,
        }));
        result_tail = &mut x.next;
        carry = sum / 10;

        if l1_tail.is_none() && l2_tail.is_none() {
            if carry > 0 {
                let _ = result_tail.insert(Box::new(ListNode {
                    val: carry,
                    next: None,
                }));
            }
            break;
        }
    }

    result_head
}

// https://leetcode.com/problems/generate-parentheses/
// https://leetcode.com/problems/generate-parentheses/editorial/
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn rec(n: i32) -> Vec<String> {
        let mut ans = vec![];
        if n == 0 {
            ans.push(String::new());
        } else {
            for i in 0..n {
                for left in rec(i) {
                    for right in rec(n - i - 1) {
                        ans.push(format!("({}){}", left, right));
                    }
                }
            }
        }
        ans
    }
    rec(n)
}

// https://leetcode.com/problems/sort-list/
pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn get_mid(head: Option<&mut Box<ListNode>>) -> Option<Box<ListNode>> {
        fn get_len(head: Option<&Box<ListNode>>) -> i32 {
            let mut len = 0;
            let mut cur = head;
            while let Some(c) = cur {
                cur = c.next.as_ref();
                len += 1;
            }
            len
        }
        let len = get_len(head.as_deref());
        let mut pre_mid_idx = len / 2 - 1;
        let mut cur = head;
        while pre_mid_idx > 0 {
            cur = cur.unwrap().next.as_mut();
            pre_mid_idx -= 1;
        }
        cur.unwrap().next.take()
    }

    fn merge_list(
        mut left: Option<Box<ListNode>>,
        mut right: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if let Some(mut left) = left.take() {
            if let Some(mut right) = right.take() {
                if left.val <= right.val {
                    left.next = merge_list(left.next.take(), Some(right));
                    Some(left)
                } else {
                    right.next = merge_list(Some(left), right.next.take());
                    Some(right)
                }
            } else {
                Some(left)
            }
        } else {
            right
        }
    }

    fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        let mid = get_mid(head.as_mut());

        // println!("h {:?} m {:?}", head, mid);

        let left = sort_list(head);
        let right = sort_list(mid);

        // println!("l {:?} r {:?}", left, right);

        merge_list(left, right)
    }

    sort_list(head)
}

// https://leetcode.com/problems/subarray-sum-equals-k/
// https://leetcode.com/problems/subarray-sum-equals-k/editorial/
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    fn using_brute_force(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            for j in i + 1..=nums.len() {
                if nums[i..j].iter().copied().sum::<i32>() == k {
                    count += 1;
                }
            }
        }
        count
    }
    fn using_prefix_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix = vec![0; nums.len() + 1];
        for i in 1..=nums.len() {
            prefix[i] += prefix[i - 1] + nums[i - 1];
        }
        let mut count = 0;
        for i in 0..nums.len() {
            for j in i + 1..=nums.len() {
                if prefix[j] - prefix[i] == k {
                    count += 1;
                }
            }
        }
        count
    }
    fn using_sum_without_space(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            let mut sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                if sum == k {
                    count += 1;
                }
            }
        }
        count
    }
    fn using_hash_map(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut num_of_sum_occurrences = vec![(0, 1)].into_iter().collect::<HashMap<i32, i32>>();
        let mut count = 0;
        let mut running_sum = 0;
        for i in 0..nums.len() {
            running_sum += nums[i];
            if num_of_sum_occurrences.contains_key(&(running_sum - k)) {
                count += num_of_sum_occurrences[&(running_sum - k)];
            }
            *num_of_sum_occurrences.entry(running_sum).or_insert(0) += 1;
        }
        count
    }
    using_hash_map(nums, k)
}

// https://leetcode.com/problems/asteroid-collision/
// https://leetcode.com/problems/asteroid-collision/editorial/
pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];
    'ast_l: for ast in asteroids {
        while !stack.is_empty() && 0 < *stack.last().unwrap() && ast < 0 {
            let top = *stack.last().unwrap();
            if top < -ast {
                stack.pop();
                continue;
            } else if top == -ast {
                stack.pop();
            }
            continue 'ast_l;
        }
        stack.push(ast);
    }
    stack
}

// https://leetcode.com/problems/random-pick-with-weight/
// https://leetcode.com/problems/random-pick-with-weight/solutions/154006/o-n-memory-binary-search/
mod rpw {
    use rand::rngs::ThreadRng;
    use rand::Rng;

    pub struct Solution {
        prefix: Vec<i32>,
        max: i32,
        rng: ThreadRng,
    }

    impl Solution {
        pub fn new(w: Vec<i32>) -> Self {
            let mut max = 0;
            let mut prefix = vec![];
            for wi in w {
                max += wi;
                prefix.push(max);
            }
            Self {
                prefix,
                max,
                rng: rand::thread_rng(),
            }
        }

        pub fn pick_index(&mut self) -> i32 {
            let prob = self.rng.gen_range(0..self.max) + 1;
            let mut lo = 0;
            let mut hi = self.prefix.len() - 1;
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if self.prefix[mid] < prob {
                    lo = mid + 1;
                } else {
                    hi = mid
                }
            }
            lo as i32
        }
    }
}

// https://leetcode.com/problems/kth-largest-element-in-an-array/
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    fn using_sorting(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        nums[nums.len() - k as usize]
    }
    fn using_count_sort(nums: Vec<i32>, mut k: i32) -> i32 {
        const PAD: usize = 10000;
        let mut arr = vec![0; 2 * PAD + 1];
        for num in nums {
            arr[num as usize + PAD] += 1;
        }
        let mut i = arr.len() - 1;
        while k > 0 {
            if arr[i] == 0 {
                i -= 1;
                continue;
            } else {
                let to_delete = arr[i].min(k);
                arr[i] -= to_delete;
                k -= to_delete;
            }
        }
        i as i32 - PAD as i32
    }
    using_count_sort(nums, k)
}

// https://leetcode.com/problems/maximal-square/
// https://leetcode.com/problems/maximal-square/editorial/
pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    fn using_brute_force(matrix: Vec<Vec<char>>) -> i32 {
        let mut max_sqrt_len = 0;
        let rows = matrix.len();
        let cols = matrix[0].len();
        for i in 0..rows {
            for j in 0..cols {
                if matrix[i][j] == '1' {
                    let mut sqrt_len = 1;
                    let mut stop = false;
                    while sqrt_len + i < rows && sqrt_len + j < cols && !stop {
                        for k in i..=i + sqrt_len {
                            for h in j..=j + sqrt_len {
                                if matrix[k][h] == '0' {
                                    stop = true;
                                    break;
                                }
                            }
                        }

                        if !stop {
                            sqrt_len += 1;
                        }
                    }
                    max_sqrt_len = max_sqrt_len.max(sqrt_len as i32);
                }
            }
        }
        max_sqrt_len * max_sqrt_len
    }
    fn using_dp_i(matrix: Vec<Vec<char>>) -> i32 {
        let mut max_sqrt_len = 0;
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut dp = vec![vec![0; cols + 1]; rows + 1];
        for i in 1..=rows {
            for j in 1..=cols {
                if matrix[i - 1][j - 1] == '1' {
                    dp[i][j] = dp[i][j - 1].min(dp[i - 1][j]).min(dp[i - 1][j - 1]) + 1;
                    max_sqrt_len = max_sqrt_len.max(dp[i][j]);
                }
            }
        }
        max_sqrt_len * max_sqrt_len
    }
    fn using_dp_ii(matrix: Vec<Vec<char>>) -> i32 {
        let mut max_sqrt_len = 0;
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut dp = vec![0; cols + 1];
        let mut prev = 0;
        for i in 1..=rows {
            for j in 1..=cols {
                let temp = dp[j];
                if matrix[i - 1][j - 1] == '1' {
                    dp[j] = dp[j].min(dp[j - 1].min(prev)) + 1;
                    max_sqrt_len = max_sqrt_len.max(dp[j]);
                } else {
                    dp[j] = 0;
                }
                prev = temp;
            }
        }
        max_sqrt_len * max_sqrt_len
    }
    using_dp_ii(matrix)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_path_sum() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 11,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: None,
                    }))),
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 8,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 13,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));
        println!("{:?}", path_sum(root, 22));
    }

    #[test]
    fn test_longest_consecutive() {
        // println!("{}", longest_consecutive(vec![100, 4, 200, 1, 3, 2])); // 4
        println!(
            "{}",
            longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1])
        ); // 9
    }

    #[test]
    fn test_rotate() {
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut arr, 3);
        println!("{:?}", arr); // [5,6,7,1,2,3,4]
        let mut arr = vec![-1];
        rotate(&mut arr, 3);
        println!("{:?}", arr); // [-1]
    }

    #[test]
    fn test_odd_even_list() {
        let root = Some(Box::new(ListNode {
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
        println!("{:?}", odd_even_list(root));
    }

    #[test]
    fn test_decode_string() {
        println!("{}", decode_string("3[a]2[bc]".to_string())); // aaabcbc
        println!("{}", decode_string("3[a2[c]]".to_string())); // accaccacc
        println!("{}", decode_string("2[abc]3[cd]ef".to_string())); // abcabccdcdcdef
    }

    #[test]
    fn test_find_max_length() {
        println!("{}", find_max_length(vec![0, 1, 0])); // 2
    }

    #[test]
    fn test_width_of_binary_tree() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));
        println!("{}", width_of_binary_tree(root));

        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
            }))),
        })));
        println!("{}", width_of_binary_tree(root));
    }

    #[test]
    fn test_find_closest_elements() {
        println!("{:?}", find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3)); // [1,2,3,4]
    }

    #[test]
    fn test_character_replacement() {
        println!("{}", character_replacement("AABA".to_string(), 0)); // 2
        println!("{}", character_replacement("ABAB".to_string(), 2)); // 4
        println!("{}", character_replacement("AABABBA".to_string(), 1)); // 4
    }

    #[test]
    fn test_can_jump() {
        can_jump(vec![2, 3, 1, 1, 4]); // true
        can_jump(vec![3, 2, 1, 0, 4]); // false
    }

    #[test]
    fn test_add_two_numbers() {
        let l1 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode {
                        val: 9,
                        next: Some(Box::new(ListNode {
                            val: 9,
                            next: Some(Box::new(ListNode {
                                val: 9,
                                next: Some(Box::new(ListNode { val: 9, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 9,
            next: Some(Box::new(ListNode {
                val: 9,
                next: Some(Box::new(ListNode {
                    val: 9,
                    next: Some(Box::new(ListNode { val: 9, next: None })),
                })),
            })),
        }));
        println!("{:?}", add_two_numbers(l1, l2));

        let l1 = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode { val: 9, next: None })),
            })),
        }));
        let l2 = Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 9, next: None })),
                })),
            })),
        }));

        println!("{:?}", add_two_numbers(l1, l2)); // [7,0,4,0,1]
    }

    #[test]
    fn test_generate_parenthesis() {
        println!("{:?}", generate_parenthesis(3)); // ["((()))","(()())","(())()","()(())","()()()"]
        println!("{:?}", generate_parenthesis(1)); // ["()"]
    }

    #[test]
    fn test_sort_list() {
        let head = Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        }));
        println!("{:?}", sort_list(head));
    }

    #[test]
    fn test_subarray_sum() {
        println!("{}", subarray_sum(vec![1, 1, 1], 2)); // 2
        println!("{}", subarray_sum(vec![1, 2, 3], 3)); // 2
    }

    #[test]
    fn test_asteroid_collision() {
        println!("{:?}", asteroid_collision(vec![5, 10, -5])); // 5, 10
        println!("{:?}", asteroid_collision(vec![8, -8])); // 0
        println!("{:?}", asteroid_collision(vec![10, 2, -5])); // 10
    }

    #[test]
    fn test_random_pick_with_weight() {
        let mut picker = rpw::Solution::new(vec![1]);
        println!("{}", picker.pick_index()); // 0

        let mut picker = rpw::Solution::new(vec![1, 3]);
        println!("{}", picker.pick_index()); // 1
        println!("{}", picker.pick_index()); // 1
        println!("{}", picker.pick_index()); // 1
        println!("{}", picker.pick_index()); // 1
        println!("{}", picker.pick_index()); // 0
    }

    #[test]
    fn test_find_kth_largest() {
        println!("{}", find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2)); // 5
        println!("{}", find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4)); // 4
    }

    #[test]
    fn test_maximal_square() {
        println!(
            "{}",
            maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0'],
            ])
        ); // 4
        println!("{}", maximal_square(vec![vec!['0', '1'], vec!['1', '0']])); // 1
        println!("{}", maximal_square(vec![vec!['0']])); // 0

        println!(
            "{}",
            maximal_square(vec![
                vec!['1', '1', '1', '1', '1', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1', '1', '1', '0'],
                vec!['1', '1', '1', '1', '1', '1', '1', '0'],
                vec!['1', '1', '1', '1', '1', '0', '0', '0'],
                vec!['0', '1', '1', '1', '1', '0', '0', '0'],
            ])
        ); // 16
    }
}
