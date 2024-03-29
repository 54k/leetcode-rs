use std::cell::RefCell;
use std::rc::Rc;

// https://leetcode.com/problems/search-in-rotated-sorted-array/description/
// https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/14425/concise-o-log-n-binary-search-solution/
// https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/14419/pretty-short-c-java-ruby-python/?orderBy=most_relevant
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    fn two_bin_search(nums: Vec<i32>, target: i32) -> i32 {
        fn find_pivot(nums: &Vec<i32>) -> usize {
            let mut lo = 0;
            let mut hi = nums.len() - 1;
            while lo < hi {
                let mid = lo + (hi - lo) / 2;
                if nums[mid] > nums[hi] {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            lo
        }
        fn bin_search(nums: &Vec<i32>, target: i32, rot: usize) -> i32 {
            let mut lo = 0_i32;
            let mut hi = nums.len() as i32 - 1;

            while lo <= hi {
                let mid = lo + (hi - lo) / 2;
                let real_mid = (mid as usize + rot) % nums.len();
                if nums[real_mid] < target {
                    lo = mid + 1;
                } else if nums[real_mid] > target {
                    hi = mid - 1;
                } else {
                    return real_mid as i32;
                }
            }
            -1
        }
        let rot = find_pivot(&nums);
        bin_search(&nums, target, rot)
    }
    fn one_bin_search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len() - 1;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;

            let mut mid_element = nums[mid];

            let mid_in_second = mid_element < nums[0]; // false -> left arr, true -> right arr
            let target_in_second = target < nums[0]; // false -> left arr, true -> right arr

            if mid_in_second ^ target_in_second {
                // mid and target in different arrays
                if target_in_second {
                    mid_element = i32::MIN;
                } else {
                    mid_element = i32::MAX;
                }
            }

            if mid_element == target {
                return mid as i32;
            } else if mid_element < target {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        -1
    }
    one_bin_search(nums, target)
}

// https://leetcode.com/problems/search-in-rotated-sorted-array-ii/description/
// https://leetcode.com/problems/search-in-rotated-sorted-array-ii/editorial/
pub fn search_ii(nums: Vec<i32>, target: i32) -> bool {
    fn is_bin_search_helpful(nums: &Vec<i32>, start: usize, element: i32) -> bool {
        nums[start] != element
    }
    fn exists_in_first_array(nums: &Vec<i32>, start: usize, element: i32) -> bool {
        // returns true if element exists in first array, false if it exists in second
        nums[start] <= element
    }
    if nums.is_empty() {
        return false;
    }
    let n = nums.len();
    let mut lo = 0;
    let mut hi = n - 1;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if nums[mid] == target {
            return true;
        }
        if !is_bin_search_helpful(&nums, lo, nums[mid]) {
            lo += 1;
            continue;
        }
        // which array does the pivot belongs to
        let pivot_array = exists_in_first_array(&nums, lo, nums[mid]);
        // which array does the target belongs to
        let target_array = exists_in_first_array(&nums, lo, target);

        if pivot_array ^ target_array {
            // If pivot and target exist in different sorted arrays,
            // recall that xor is true when both operands are distinct
            if pivot_array {
                lo = mid + 1;
                // pivot in the first, target in the second
            } else {
                hi = mid - 1;
                // target in the first, pivot in the second
            }
        } else {
            // if pivot and target exist in same sorted array
            if nums[mid] < target {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
    }
    false
}

// https://leetcode.com/problems/combination-sum/description/
pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn rec(
        candidates: &Vec<i32>,
        target: i32,
        start: usize,
        current_combination: &mut Vec<i32>,
        combinations: &mut Vec<Vec<i32>>,
    ) {
        if target < 0 {
            return;
        }
        if target == 0 {
            combinations.push(current_combination.clone());
            return;
        }
        for i in start..candidates.len() {
            current_combination.push(candidates[i]);
            rec(
                candidates,
                target - candidates[i],
                i,
                current_combination,
                combinations,
            );
            current_combination.pop();
        }
    }
    let mut result = vec![];
    rec(&candidates, target, 0, &mut vec![], &mut result);
    result
}

// https://leetcode.com/problems/combination-sum-ii/description/
pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn rec(
        candidates: &Vec<i32>,
        target: i32,
        start: usize,
        current_combination: &mut Vec<i32>,
        combinations: &mut Vec<Vec<i32>>,
    ) {
        if target < 0 {
            return;
        }
        if target == 0 {
            combinations.push(current_combination.clone());
            return;
        }

        for i in start..candidates.len() {
            let picked_num = candidates[i];
            if i > start && picked_num == candidates[i - 1] {
                continue;
            }
            current_combination.push(picked_num);
            rec(
                candidates,
                target - picked_num,
                i + 1,
                current_combination,
                combinations,
            );
            current_combination.pop();
        }
    }

    candidates.sort();
    let mut result = vec![];
    rec(&candidates, target, 0, &mut vec![], &mut result);
    result
}

// https://leetcode.com/problems/permutations/
pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn permute_recursive(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn rec(
            nums: &Vec<i32>,
            mut occupied: i32,
            current_permutation: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if current_permutation.len() == nums.len() {
                result.push(current_permutation.clone());
                return;
            }
            for i in 0..nums.len() {
                if (occupied & (1 << i)) == 0 {
                    occupied |= 1 << i;
                    current_permutation.push(nums[i]);
                    rec(nums, occupied, current_permutation, result);
                    current_permutation.pop();
                    occupied ^= 1 << i;
                }
            }
        }
        let mut result = vec![];
        rec(&nums, 0, &mut vec![], &mut result);
        result
    }
    permute_recursive(nums)
}

// https://leetcode.com/problems/merge-intervals/
// https://leetcode.com/problems/merge-intervals/editorial/
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    fn emre_solution(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort();
        let mut ans = vec![];
        let mut start = intervals[0][0];
        let mut end = intervals[0][1];
        for interval in intervals.into_iter().skip(1) {
            if interval[0] <= end {
                end = end.max(interval[1]);
            } else {
                ans.push(vec![start, end]);
                start = interval[0];
                end = interval[1];
            }
        }
        ans.push(vec![start, end]);
        ans
    }

    fn leetcode_sort(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort();
        let mut merged: Vec<Vec<i32>> = vec![];
        for interval in intervals {
            // if the list of merged intervals is empty or if the current
            // interval does not overlap with the previous, simply append it.
            if merged.is_empty() || merged.last().unwrap()[1] < interval[0] {
                merged.push(interval);
            } else {
                // otherwise, there is overlap, so we merge the current and previous intervals.
                merged.last_mut().unwrap()[1] = merged.last().unwrap()[1].max(interval[1]);
            }
        }
        merged
    }
    leetcode_sort(intervals)
}

// https://leetcode.com/problems/teemo-attacking/description/
// https://leetcode.com/problems/teemo-attacking/editorial/
pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
    let mut ans = 0;
    for i in 0..time_series.len() {
        if i < time_series.len() - 1 {
            ans += duration.min(time_series[i + 1] - time_series[i]);
        } else {
            ans += duration;
        }
    }
    ans
}

// https://leetcode.com/problems/dota2-senate/description/
pub fn predict_party_victory(senate: String) -> String {
    use std::collections::VecDeque;
    let mut queue = VecDeque::new();
    let mut radiant_banned = 0;
    let mut dire_banned = 0;
    let mut radiant_in_queue = 0;
    let mut dire_in_queue = 0;

    for senator in senate.chars() {
        if senator == 'R' {
            radiant_in_queue += 1;
        } else {
            dire_in_queue += 1;
        }
        queue.push_back(senator);
    }

    while radiant_in_queue > 0 && dire_in_queue > 0 {
        if let Some('R') = queue.pop_front() {
            if radiant_banned > 0 {
                radiant_banned -= 1;
                radiant_in_queue -= 1;
                continue;
            }
            dire_banned += 1;
            queue.push_back('R');
        } else {
            if dire_banned > 0 {
                dire_banned -= 1;
                dire_in_queue -= 1;
                continue;
            }
            radiant_banned += 1;
            queue.push_back('D');
        }
    }

    if let Some('R') = queue.front() {
        "Radiant".to_string()
    } else {
        "Dire".to_string()
    }
}

// https://leetcode.com/problems/2-keys-keyboard/description/
// https://leetcode.com/problems/2-keys-keyboard/editorial/
pub fn min_steps(mut n: i32) -> i32 {
    fn leetcode_min_steps(mut n: i32) -> i32 {
        let mut ans = 0;
        let mut i = 2;
        while n > 1 {
            while n % i == 0 {
                ans += i;
                n /= i;
            }
            i += 1;
        }
        ans
    }
    fn laakonsen_cp_min_steps(mut n: i32) -> i32 {
        let mut ans = 0;
        let mut i = 2;
        while i * i <= n {
            while n % i == 0 {
                ans += i;
                n /= i;
            }
            i += 1;
        }
        if n > 1 {
            ans += n;
        }
        ans
    }
    laakonsen_cp_min_steps(n)
}

// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn lowest_common_ancestor_postorder(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            return root;
        }
        let r = root.clone()?;
        let r = r.borrow();
        let lcp_left = lowest_common_ancestor_postorder(r.left.clone(), p.clone(), q.clone());
        let lcp_right = lowest_common_ancestor_postorder(r.right.clone(), p, q);
        if lcp_left.is_some() && lcp_right.is_some() {
            root
        } else if lcp_left.is_some() {
            lcp_left
        } else {
            lcp_right
        }
    }
    lowest_common_ancestor_postorder(root, p, q)
}

// https://leetcode.com/problems/time-based-key-value-store/
// https://leetcode.com/problems/time-based-key-value-store/editorial/
use std::collections::HashMap;

struct TimeMap {
    buckets: HashMap<String, Vec<(i32, String)>>,
}
impl TimeMap {
    fn new() -> Self {
        Self {
            buckets: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.buckets
            .entry(key)
            .or_insert(vec![])
            .push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if !self.buckets.contains_key(&key) {
            return "".to_string();
        }

        let bucket = &self.buckets[&key];
        let len = bucket.len();
        let mut lo = 0;
        let mut hi = len;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if bucket[mid].0 <= timestamp {
                lo = mid + 1;
            } else {
                hi = mid
            }
        }

        if hi == 0 {
            // no timestamp <= exists
            return "".to_string();
        }
        bucket[hi - 1].1.clone()
    }
}

// https://leetcode.com/problems/accounts-merge/description/
// https://leetcode.com/problems/accounts-merge/editorial/
pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
    fn using_dfs(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        use std::collections::{HashMap, HashSet};
        fn dfs(
            v: String,
            adj: &HashMap<String, Vec<String>>,
            visited: &mut HashSet<String>,
            components: &mut Vec<String>,
        ) {
            if visited.contains(&v) {
                return;
            }
            visited.insert(v.clone());
            components.push(v.to_string());

            for u in &adj[&v] {
                if !visited.contains(u) {
                    dfs(u.clone(), adj, visited, components);
                }
            }
        }

        let mut visited = HashSet::new();
        let mut adj = HashMap::new();

        for account in &accounts {
            let first_email = &account[1];
            adj.entry(first_email.clone()).or_insert(vec![]);
            for email in account.iter().skip(2) {
                adj.get_mut(first_email).unwrap().push(email.clone());
                adj.entry(email.clone())
                    .or_insert(vec![])
                    .push(first_email.clone());
            }
        }

        let mut merged_accounts = vec![];
        for account in &accounts {
            let first_email = &account[1];
            if !visited.contains(first_email) {
                let mut components = vec![account[0].clone()];
                dfs(first_email.clone(), &adj, &mut visited, &mut components);
                components[1..].sort();
                merged_accounts.push(components);
            }
        }
        merged_accounts
    }
    fn using_union_find(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        struct UnionFind {
            links: Vec<usize>,
            sizes: Vec<usize>,
        }
        impl UnionFind {
            fn new(size: usize) -> Self {
                let mut links = vec![0; size];
                for i in 0..size {
                    links[i] = i;
                }
                let sizes = vec![1; size];
                Self { links, sizes }
            }
            fn find(&mut self, x: usize) -> usize {
                if self.links[x] != x {
                    self.links[x] = self.find(self.links[x]);
                }
                self.links[x]
            }
            fn union(&mut self, x: usize, y: usize) {
                let mut x = self.find(x);
                let mut y = self.find(y);
                if x == y {
                    return;
                }
                if self.sizes[x] < self.sizes[y] {
                    std::mem::swap(&mut x, &mut y);
                }
                self.links[y] = x;
                self.sizes[x] += self.sizes[y];
            }
        }
        let mut dsu = UnionFind::new(accounts.len());

        // Maps email to their component index
        let mut email_groups = HashMap::new();
        for i in 0..accounts.len() {
            for j in 1..accounts[i].len() {
                let email = &accounts[i][j];
                if !email_groups.contains_key(email) {
                    email_groups.insert(email.clone(), i);
                } else {
                    dsu.union(i, email_groups[email]);
                }
            }
        }

        // Store emails corresponding to the component's representative
        let mut components = HashMap::new();
        for email in email_groups.keys() {
            let group_representative = dsu.find(email_groups[email]);
            components
                .entry(group_representative)
                .or_insert(vec![])
                .push(email.clone());
        }

        // Sort the components and add the account name
        let mut merged_accounts = vec![];
        for group in components.keys() {
            let mut component = components[group].clone();
            component.sort();
            component.insert(0, accounts[*group][0].clone());
            merged_accounts.push(component);
        }

        merged_accounts
    }

    using_union_find(accounts)
}

// https://leetcode.com/problems/sort-colors/
pub fn sort_colors(nums: &mut Vec<i32>) {
    use std::cmp::Ordering;
    let (mut lt, mut gt, mut i) = (0, nums.len(), 0);
    while i < gt {
        match nums[i].cmp(&1) {
            Ordering::Less => {
                nums.swap(lt, i);
                lt += 1;
                i += 1;
            }
            Ordering::Equal => {
                i += 1;
            }
            Ordering::Greater => {
                gt -= 1;
                nums.swap(gt, i);
            }
        }
    }
}

// https://leetcode.com/problems/word-break/
// https://leetcode.com/problems/word-break/editorial/
pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    fn brute_memo(s: String, word_dict: Vec<String>) -> bool {
        use std::collections::HashSet;
        fn rec(
            s: &[char],
            start: usize,
            word_dict: &HashSet<String>,
            memo: &mut [Option<bool>],
        ) -> bool {
            if start == s.len() {
                return true;
            }

            if memo[start].is_some() {
                return memo[start].unwrap();
            }
            for end in start + 1..=s.len() {
                if word_dict.contains(&s[start..end].iter().copied().collect::<String>())
                    && rec(s, end, word_dict, memo)
                {
                    memo[start] = Some(true);
                    return memo[start].unwrap();
                }
            }
            memo[start] = Some(false);
            memo[start].unwrap()
        }

        let s = s.chars().collect::<Vec<_>>();
        let mut memo = vec![None; s.len()];
        let word_dict = word_dict.into_iter().collect::<HashSet<String>>();
        rec(&s, 0, &word_dict, &mut memo)
    }

    fn bfs(s: String, word_dict: Vec<String>) -> bool {
        use std::collections::{HashSet, VecDeque};
        let s = s.chars().collect::<Vec<_>>();
        let word_dict = word_dict.into_iter().collect::<HashSet<String>>();
        let mut visited = vec![false; s.len()];
        let mut queue = VecDeque::new();
        queue.push_back(0);
        while let Some(start) = queue.pop_front() {
            if visited[start] {
                continue;
            }
            visited[start] = true;
            for end in start + 1..=s.len() {
                if word_dict.contains(&s[start..end].iter().copied().collect::<String>()) {
                    queue.push_back(end);
                    if end == s.len() {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn dp(s: String, word_dict: Vec<String>) -> bool {
        use std::collections::HashSet;
        let s = s.chars().collect::<Vec<_>>();
        let word_dict = word_dict.into_iter().collect::<HashSet<String>>();
        let mut dp = vec![false; s.len() + 1];
        dp[0] = true;
        for start in 0..s.len() {
            for end in start + 1..=s.len() {
                if word_dict.contains(&s[start..end].iter().copied().collect::<String>())
                    && dp[start]
                {
                    dp[end] = true;
                }
            }
        }
        dp[s.len()]
    }

    dp(s, word_dict)
}

// https://leetcode.com/problems/partition-equal-subset-sum/
pub fn can_partition(nums: Vec<i32>) -> bool {
    fn classic_knapsack(nums: Vec<i32>) -> bool {
        let mut sum = nums.iter().sum::<i32>() as usize;
        if sum % 2 == 1 {
            return false;
        }
        sum /= 2;
        let mut dp = vec![vec![false; sum + 1]; nums.len() + 1];
        dp[0][0] = true;

        for i in 1..=nums.len() {
            for j in 1..=sum {
                dp[i][j] |= dp[i - 1][j];
                if j >= nums[i - 1] as usize {
                    dp[i][j] |= dp[i - 1][j - nums[i - 1] as usize];
                }
            }
        }
        dp[nums.len()][sum]
    }
    fn mem_optimized_knapsack(nums: Vec<i32>) -> bool {
        let mut sum = nums.iter().copied().sum::<i32>() as usize;
        if sum % 2 == 1 {
            return false;
        }
        sum /= 2;
        let mut dp = vec![false; sum + 1];
        dp[0] = true;
        for i in 0..nums.len() {
            for j in (0..=sum - nums[i] as usize).rev() {
                dp[j + nums[i] as usize] |= dp[j];
            }
        }
        dp[sum]
    }
    mem_optimized_knapsack(nums)
}

// https://leetcode.com/problems/string-to-integer-atoi/
pub fn my_atoi(s: String) -> i32 {
    let s = s.trim();
    let mut num = 0;
    let mut sign = 1;
    let mut sign_seen = false;

    for ch in s.chars() {
        match ch {
            _ if ch.is_ascii_digit() => {
                sign_seen = true;
                let digit = ch as i32 - 48;
                if num > i32::MAX / 10 {
                    return if sign < 0 { i32::MIN } else { i32::MAX };
                }
                if num * sign == i32::MAX / 10 && digit > i32::MAX % 10 {
                    return i32::MAX;
                } else if num * sign == i32::MIN / 10 && digit >= -(i32::MIN % 10) {
                    return i32::MIN;
                }
                num = num * 10 + digit;
            }
            '+' if !sign_seen => {
                sign_seen = true;
            }
            '-' if !sign_seen => {
                sign_seen = true;
                sign *= -1;
            }
            _ => break,
        }
    }
    num * sign
}

// https://leetcode.com/problems/spiral-matrix/
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    fn simulate_step(
        matrix: &[Vec<i32>],
        coord: &mut (i32, i32),
        dir_idx: &mut usize,
        row: &mut (i32, i32),
        col: &mut (i32, i32),
        path: &mut Vec<i32>,
    ) {
        if coord.1 == col.1 && (*dir_idx % 4) == 0 {
            *col = (col.0, col.1 - 1);
            *dir_idx += 1;
        }
        if coord.1 == col.0 && (*dir_idx % 4) == 2 {
            *col = (col.0 + 1, col.1);
            *dir_idx += 1;
        }
        if coord.0 == row.1 && (*dir_idx % 4) == 1 {
            *row = (row.0 + 1, row.1);
            *dir_idx += 1;
        }
        if coord.0 == row.0 && (*dir_idx % 4) == 3 {
            *row = (row.0, row.1 - 1);
            *dir_idx += 1;
        }

        path.push(matrix[coord.0 as usize][coord.1 as usize]);

        let dir = DIRS[*dir_idx % 4];
        *coord = (coord.0 + dir.0, coord.1 + dir.1);
    }

    let mut path = vec![];
    let mut row = (0_i32, matrix.len() as i32 - 1);
    let mut col = (0_i32, matrix[0].len() as i32 - 1);
    let mut coord = (0, 0);
    let mut dir_idx = 0;

    while path.len() != matrix.len() * matrix[0].len() {
        simulate_step(
            &matrix,
            &mut coord,
            &mut dir_idx,
            &mut row,
            &mut col,
            &mut path,
        );
    }
    path
}

// https://leetcode.com/problems/subsets/description/
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn bit_mask(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        for i in 0..1 << nums.len() {
            let mut subset = vec![];
            for j in 0..nums.len() {
                if (i >> j) & 1 == 1 {
                    subset.push(nums[j]);
                }
            }
            result.push(subset);
        }
        result
    }
    fn recursive(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut cur = vec![];
        fn rec(nums: &[i32], i: usize, cur: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if i == nums.len() {
                result.push(cur.clone());
                return;
            }
            cur.push(nums[i]);
            rec(nums, i + 1, cur, result);
            cur.pop();
            rec(nums, i + 1, cur, result);
        }
        rec(&nums, 0, &mut cur, &mut result);
        result
    }
    recursive(nums)
}

// https://leetcode.com/problems/binary-tree-right-side-view/
pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn preorder_inverted(
        root: Option<Rc<RefCell<TreeNode>>>,
        depth: i32,
        max_depth: &mut i32,
        result: &mut Vec<i32>,
    ) {
        if let Some(r) = root {
            let r = r.borrow();
            if depth > *max_depth {
                result.push(r.val);
            }
            *max_depth = (*max_depth).max(depth);
            preorder_inverted(r.right.clone(), depth + 1, max_depth, result);
            preorder_inverted(r.left.clone(), depth + 1, max_depth, result);
        }
    }
    let mut result = vec![];
    preorder_inverted(root, 0, &mut -1, &mut result);
    result
}

// https://leetcode.com/problems/longest-palindromic-substring/
// https://leetcode.com/problems/longest-palindromic-substring/editorial/
pub fn longest_palindrome(s: String) -> String {
    fn expand_around_center(s: &[char], left: i32, right: i32) -> i32 {
        let mut l = left;
        let mut r = right;
        while l >= 0 && r < s.len() as i32 && s[l as usize] == s[r as usize] {
            l -= 1;
            r += 1;
        }
        r - l - 1
    }
    let s = s.chars().collect::<Vec<_>>();
    let mut start = 0;
    let mut end = 0;
    for i in 0..s.len() {
        let len1 = expand_around_center(&s, i as i32, i as i32);
        let len2 = expand_around_center(&s, i as i32, i as i32 + 1);
        let len = len1.max(len2);
        if len > (end - start) {
            start = i as i32 - (len - 1) / 2;
            end = i as i32 + len / 2;
        }
    }
    s[(start as usize)..=end as usize].iter().copied().collect()
}

// https://leetcode.com/problems/unique-paths/
pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut dp = vec![vec![1; n as usize]; m as usize];
    for i in 1..dp.len() {
        for j in 1..dp[0].len() {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }
    dp[m as usize - 1][n as usize - 1]
}

// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/description/
pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::HashMap;
    let mut inorder_map = inorder
        .iter()
        .copied()
        .enumerate()
        .map(|(i, x)| (x, i as i32))
        .collect::<HashMap<i32, i32>>();

    fn build_tree(
        preorder: &Vec<i32>,
        inorder_map: &HashMap<i32, i32>,
        preorder_idx: &mut usize,
        left: i32,
        right: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if left > right {
            return None;
        }

        let mut root = TreeNode {
            val: preorder[*preorder_idx],
            left: None,
            right: None,
        };
        *preorder_idx += 1;

        root.left = build_tree(
            preorder,
            inorder_map,
            preorder_idx,
            left,
            inorder_map[&root.val] - 1,
        );

        root.right = build_tree(
            preorder,
            inorder_map,
            preorder_idx,
            inorder_map[&root.val] + 1,
            right,
        );

        Some(Rc::new(RefCell::new(root)))
    }

    build_tree(
        &preorder,
        &inorder_map,
        &mut 0,
        0,
        preorder.len() as i32 - 1,
    )
}

// https://leetcode.com/problems/container-with-most-water/description/
// https://leetcode.com/problems/container-with-most-water/editorial/
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut i = 0;
    let mut j = height.len() - 1;
    let mut height_left = 0;
    let mut height_right = 0;
    while i < j {
        height_left = height_left.max(height[i]);
        height_right = height_right.max(height[j]);
        ans = ans.max(height_left.min(height_right) * (j as i32 - i as i32));
        if height_left <= height_right {
            i += 1;
        } else {
            j -= 1;
        }
    }
    ans
}

// https://leetcode.com/problems/letter-combinations-of-a-phone-number/
pub fn letter_combinations(digits: String) -> Vec<String> {
    const MAPPING: [&str; 11] = [
        "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz", " ",
    ];
    fn rec(digits: &Vec<char>, cur_digit: usize, cur: &mut String, result: &mut Vec<String>) {
        if cur_digit == digits.len() {
            if !cur.is_empty() {
                result.push(cur.clone());
            }
            return;
        }
        for ch in MAPPING[digits[cur_digit] as usize - 48].chars() {
            cur.push(ch);
            rec(digits, cur_digit + 1, cur, result);
            cur.pop();
        }
    }
    let digits = digits.chars().collect::<Vec<_>>();
    let mut result = vec![];
    rec(&digits, 0, &mut String::new(), &mut result);
    result
}

// https://leetcode.com/problems/word-search/
pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    const DIR: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    fn dfs(
        board: &[Vec<char>],
        word: &[char],
        word_idx: usize,
        coord: (i32, i32),
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        if word_idx == word.len() {
            return true;
        }

        if coord.0 < 0
            || coord.0 >= board.len() as i32
            || coord.1 < 0
            || coord.1 >= board[0].len() as i32
            || visited[coord.0 as usize][coord.1 as usize]
        {
            return false;
        }

        let i = coord.0 as usize;
        let j = coord.1 as usize;
        let mut found = false;

        if board[i][j] == word[word_idx] {
            for d in DIR {
                visited[i][j] = true;
                found |= dfs(
                    board,
                    word,
                    word_idx + 1,
                    (coord.0 + d.0, coord.1 + d.1),
                    visited,
                );
                visited[i][j] = false;
            }
        }
        found
    }
    let mut found = false;
    let word = word.chars().collect::<Vec<_>>();
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if board[i][j] == word[0] {
                let mut visited = vec![vec![false; board[0].len()]; board.len()];
                found |= dfs(&board, &word, 0, (i as i32, j as i32), &mut visited);
            }
        }
    }
    found
}

// https://leetcode.com/problems/find-all-anagrams-in-a-string/
pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut result = vec![];
    let s = s.chars().collect::<Vec<_>>();
    let p = p.chars().collect::<Vec<_>>();

    let mut p_map = vec![0; 26];
    for &ch in &p {
        p_map[ch as usize - 'a' as usize] += 1;
    }

    let mut s_map = vec![0; 26];
    let mut start = 0;
    for end in 0..s.len() {
        s_map[s[end] as usize - 'a' as usize] += 1;
        if end - start + 1 == p.len() {
            if s_map == p_map {
                result.push(start as i32);
            }
            s_map[s[start] as usize - 'a' as usize] -= 1;
            start += 1;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search() {
        println!("{}", search(vec![4, 5, 6, 7, 0, 1, 2], 0)); // 4
        println!("{}", search(vec![1, 3], 1)); // 0
        println!("{}", search(vec![3, 1], 1)); // 1
        println!("{}", search(vec![3, 1], 3)); // 0

        println!("{}", search(vec![3, 5, 1], 1)); // 2
        println!("{}", search(vec![3, 5, 1], 3)); // 0
        println!("{}", search(vec![5, 1, 3], 1)); // 1
        println!("{}", search(vec![5, 1, 3], 5)); // 0
        println!("{}", search(vec![5, 1, 3], 3)); // 2

        println!("{}", search_ii(vec![2, 2, 2, 3, 2, 2, 2], 3)); // true
        println!("{}", search_ii(vec![2, 5, 6, 0, 0, 1, 2], 0)); // true
        println!(
            "{}",
            search_ii(
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1],
                2
            )
        ); // true
    }

    #[test]
    fn test_combination_sum() {
        println!("{:?}", combination_sum(vec![2, 3, 6, 7], 7)); // [[2,2,3],[7]]
        println!("{:?}", combination_sum(vec![2, 3, 5], 8)); // [[2,2,2,2],[2,3,3],[3,5]]
        println!("{:?}", combination_sum(vec![2], 1)); // [[2,2,2,2],[2,3,3],[3,5]]

        println!("{:?}", combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)); // [[1,1,6],[1,2,5],[1,7],[2,6]]
        println!("{:?}", combination_sum2(vec![2, 5, 2, 1, 2], 5)); // [[1,2,2],[5]]
    }

    #[test]
    fn test_permute() {
        println!("{:?}", permute(vec![1, 2, 3]));
    }

    #[test]
    fn test_merge() {
        println!(
            "{:?}",
            merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]])
        ); // [[1,6],[8,10],[15,18]]
    }

    #[test]
    fn test_find_poisoned_duration() {
        println!("{}", find_poisoned_duration(vec![1, 4], 2)); // 4
        println!("{}", find_poisoned_duration(vec![1, 2], 2)); // 3
    }

    #[test]
    fn test_predict_party_victory() {
        println!("{}", predict_party_victory("RD".to_string())); // Radiant
        println!("{}", predict_party_victory("DR".to_string())); // Dire
        println!("{}", predict_party_victory("RDD".to_string())); // Dire
        println!("{}", predict_party_victory("DDRRR".to_string())); // Dire
    }

    #[test]
    fn test_min_steps() {
        println!("{}", min_steps(9));

        println!("{}", min_steps(3));
        println!("{}", min_steps(10));
        println!("{}", min_steps(17));
    }

    #[test]
    fn test_lowest_common_ancestor() {
        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: None,
            right: None,
        })));
        let q = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: None,
            right: None,
        })));
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: p.clone(),
                val: 2,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: q.clone(),
                val: 2,
            }))),
        })));

        println!("{:?}", lowest_common_ancestor(root, p, q)); // 1
    }

    #[test]
    fn test_time_map() {
        let mut time_map = TimeMap::new();
        time_map.set("foo".to_string(), "bar".to_string(), 1);
        println!("{}", time_map.get("foo".to_string(), 1));
        println!("{}", time_map.get("foo".to_string(), 3));
        time_map.set("foo".to_string(), "bar2".to_string(), 4);
        println!("{}", time_map.get("foo".to_string(), 4));
        println!("{}", time_map.get("foo".to_string(), 5));
    }

    #[test]
    fn test_accounts_merge() {
        println!(
            "{:?}",
            accounts_merge(vec![
                vec![
                    "John".to_string(),
                    "johnsmith@mail.com".to_string(),
                    "john_newyork@mail.com".to_string(),
                ],
                vec![
                    "John".to_string(),
                    "johnsmith@mail.com".to_string(),
                    "john00@mail.com".to_string(),
                ],
                vec!["Mary".to_string(), "mary@mail.com".to_string()],
                vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
            ])
        ); // [["John","john00@mail.com","john_newyork@mail.com","johnsmith@mail.com"],
           // ["Mary","mary@mail.com"],["John","johnnybravo@mail.com"]]

        println!(
            "{:?}",
            accounts_merge(vec![
                vec![
                    "Gabe".to_string(),
                    "Gabe0@m.co".to_string(),
                    "Gabe3@m.co".to_string(),
                    "Gabe1@m.co".to_string()
                ],
                vec![
                    "Kevin".to_string(),
                    "Kevin3@m.co".to_string(),
                    "Kevin5@m.co".to_string(),
                    "Kevin0@m.co".to_string()
                ],
                vec![
                    "Ethan".to_string(),
                    "Ethan5@m.co".to_string(),
                    "Ethan4@m.co".to_string(),
                    "Ethan0@m.co".to_string()
                ],
                vec![
                    "Hanzo".to_string(),
                    "Hanzo3@m.co".to_string(),
                    "Hanzo1@m.co".to_string(),
                    "Hanzo0@m.co".to_string()
                ],
                vec![
                    "Fern".to_string(),
                    "Fern5@m.co".to_string(),
                    "Fern1@m.co".to_string(),
                    "Fern0@m.co".to_string()
                ],
            ])
        ); // [["Ethan","Ethan0@m.co","Ethan4@m.co","Ethan5@m.co"],
           // ["Gabe","Gabe0@m.co","Gabe1@m.co","Gabe3@m.co"],
           // ["Hanzo","Hanzo0@m.co","Hanzo1@m.co","Hanzo3@m.co"],
           // ["Kevin","Kevin0@m.co","Kevin3@m.co","Kevin5@m.co"],
           // ["Fern","Fern0@m.co","Fern1@m.co","Fern5@m.co"]]
    }

    #[test]
    fn test_sort_colors() {
        let mut v = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut v);
        println!("{:?}", v);
    }

    #[test]
    fn test_word_break() {
        println!(
            "{}",
            word_break(
                "leetcode".to_string(),
                vec!["leet".to_string(), "code".to_string()]
            )
        ); // true
        println!(
            "{}",
            word_break(
                "applepenapple".to_string(),
                vec!["apple".to_string(), "pen".to_string()]
            )
        ); // true
    }

    #[test]
    fn test_can_partition() {
        println!("{}", can_partition(vec![1, 5, 11, 5])); // true
        println!("{}", can_partition(vec![1, 2, 3, 5])); // false
        println!("{}", can_partition(vec![1, 5, 3])); // false
    }

    #[test]
    fn test_my_atoi() {
        println!("{}", my_atoi("-2147483647".to_string())); // -2147483647
        println!("{}", my_atoi(i32::MIN.to_string())); // -2147483648
        println!("{}", my_atoi(i32::MAX.to_string())); // 2147483647

        println!("{}", my_atoi("   -42".to_string())); // -42
        println!("{}", my_atoi("4193 with words".to_string())); // 4193
    }

    #[test]
    fn test_spiral_order() {
        println!(
            "{:?}",
            spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        ); // [1,2,3,6,9,8,7,4,5]

        println!("{:?}", spiral_order(vec![vec![1, 2, 3]])); // [1,2,3]
        println!("{:?}", spiral_order(vec![vec![1], vec![2], vec![3]])); // [1,2,3]
    }

    #[test]
    fn test_subsets() {
        println!("{:?}", subsets(vec![1, 2, 3])); // [[1, 2, 3], [1, 2], [1, 3], [1], [2, 3], [2], [3], []]
    }

    #[test]
    fn test_right_side_view() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        println!("{:?}", right_side_view(root));
    }

    #[test]
    fn test_longest_palindrome() {
        println!("{}", longest_palindrome("babad".to_string())); // aba or bad
        println!("{}", longest_palindrome("cbbd".to_string())); // bb
    }

    #[test]
    fn test_unique_paths() {
        println!("{}", unique_paths(3, 7)); // 28
        println!("{}", unique_paths(3, 2)); // 3
    }

    #[test]
    fn test_build_tree() {
        println!(
            "{:?}",
            build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7])
        ); // [3,9,20,null,null,15,7]
    }

    #[test]
    fn test_max_area() {
        println!("{}", max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7])); // 49
        println!("{}", max_area(vec![1, 1])); // 1
    }

    #[test]
    fn test_letter_combinations() {
        println!("{:?}", letter_combinations("23".to_string())); // ["ad","ae","af","bd","be","bf","cd","ce","cf"]
        println!("{:?}", letter_combinations("".to_string())); // []
        println!("{:?}", letter_combinations("2".to_string())); // ["a", "b", "c"]
    }

    #[test]
    fn test_exists() {
        println!(
            "{}",
            exist(vec![vec!['a', 'b'], vec!['c', 'd']], "acdb".to_string())
        ); // true

        println!(
            "{}",
            exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "SEE".to_string()
            )
        ); // true

        println!(
            "{}",
            exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCCED".to_string()
            )
        ); // true

        println!(
            "{}",
            exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCB".to_string()
            )
        ); // false
    }

    #[test]
    fn test_find_anagrams() {
        println!(
            "{:?}",
            find_anagrams("cbaebabacd".to_string(), "abc".to_string())
        ); // [0,6]
        println!("{:?}", find_anagrams("abab".to_string(), "ab".to_string())); // [0,1,2]
    }
}
