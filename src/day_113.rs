// https://leetcode.com/problems/jump-game-iv/description/
// https://leetcode.com/problems/jump-game-iv/editorial/
pub fn min_jumps(arr: Vec<i32>) -> i32 {
    use std::collections::*;
    let mut adj = HashMap::new();
    for i in 0..arr.len() {
        adj.entry(arr[i]).or_insert(vec![]).push(i as i32);
    }

    let mut visited = HashSet::new();
    let mut steps = 0;
    let mut curs = vec![0]; // current layer
    let n = arr.len() as i32;

    while !curs.is_empty() {
        let mut next = vec![]; // next layer
        for &v in &curs {
            if v == n - 1 {
                return steps;
            }

            let map_key = arr[v as usize];
            for &u in &adj[&map_key] {
                if !visited.contains(&u) {
                    visited.insert(u);
                    next.push(u);
                }
            }
            adj.get_mut(&map_key).unwrap().clear();

            if (v - 1) >= 0 && !visited.contains(&(v - 1)) {
                visited.insert(v - 1);
                next.push(v - 1);
            }

            if v + 1 < n && !visited.contains(&(v + 1)) {
                visited.insert(v + 1);
                next.push(v + 1);
            }
        }
        curs = next;
        steps += 1;
    }
    -1
}

// https://leetcode.com/problems/increasing-triplet-subsequence/description/
// https://leetcode.com/problems/increasing-triplet-subsequence/solutions/78994/generalized-solution-to-increasing-subsequence-of-any-size-k/
pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    fn leetcode(nums: Vec<i32>) -> bool {
        let k = 3usize;
        let mut mins = vec![i32::MAX; k - 1];
        for n in nums {
            let mut i = 0;
            while i < mins.len() {
                if n <= mins[i] {
                    mins[i] = n;
                    break;
                }
                i += 1;
            }
            if i == mins.len() {
                return true;
            }
        }
        false
    }
    // tle
    fn lis(nums: Vec<i32>) -> bool {
        let mut d = vec![1; nums.len()];
        let mut max = 1;
        for i in 0..nums.len() {
            for j in 0..i {
                if nums[j] < nums[i] && d[j] + 1 > d[i] {
                    d[i] = d[j] + 1;
                    max = max.max(d[i]);
                }
            }
        }
        max >= 3
    }
    leetcode(nums)
}

// https://leetcode.com/problems/shortest-unsorted-continuous-subarray/description/
// https://leetcode.com/problems/shortest-unsorted-continuous-subarray/editorial/
pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    // tle
    fn brute_force(nums: Vec<i32>) -> i32 {
        let mut ans = nums.len() as i32;
        for i in 0..nums.len() {
            for j in i..=nums.len() {
                let mut min = i32::MAX;
                let mut max = i32::MIN;
                let mut prev = i32::MIN; // to check if [0..i] and [j..len] is sorted
                for k in i..j {
                    min = min.min(nums[k]);
                    max = max.max(nums[k]);
                }
                if (i > 0 && nums[i - 1] > min) || (j < nums.len() && nums[j] < max) {
                    continue;
                }

                let mut k = 0;
                // check [0..i] is sorted
                while k < i && prev <= nums[k] {
                    prev = nums[k];
                    k += 1;
                }
                if k != i {
                    continue; // not sorted
                }
                k = j;
                // check [j..k] is sorted
                while k < nums.len() && prev <= nums[k] {
                    prev = nums[k];
                    k += 1;
                }
                if k == nums.len() {
                    ans = ans.min((j - i) as i32);
                }
            }
        }
        ans
    }

    fn insertion_sort(nums: Vec<i32>) -> i32 {
        let mut l = nums.len() as i32;
        let mut r = 0 as i32;
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                if nums[i] > nums[j] {
                    l = l.min(i as i32);
                    r = r.max(j as i32);
                }
            }
        }
        if r - l < 0 {
            0
        } else {
            r - l + 1
        }
    }

    fn using_sorting(nums: Vec<i32>) -> i32 {
        let mut sorted = nums.clone();
        sorted.sort();
        let mut r = 0 as i32;
        let mut l = nums.len() as i32;
        for i in 0..nums.len() {
            if sorted[i] != nums[i] {
                l = l.min(i as i32);
                r = r.max(i as i32);
            }
        }
        if r - l < 0 {
            0
        } else {
            r - l + 1
        }
    }

    fn using_stack(nums: Vec<i32>) -> i32 {
        let mut l = nums.len() as i32;
        let mut r = 0 as i32;
        let mut stack = vec![];
        for i in 0..nums.len() {
            while !stack.is_empty() && nums[stack[stack.len() - 1]] > nums[i] {
                let x = stack.pop().unwrap(); // insert position
                l = l.min(x as i32);
            }
            stack.push(i);
        }
        stack.clear();
        for i in (0..nums.len()).rev() {
            while !stack.is_empty() && nums[stack[stack.len() - 1]] < nums[i] {
                let x = stack.pop().unwrap();
                r = r.max(x as i32);
            }
            stack.push(i);
        }
        if r - l < 0 {
            0
        } else {
            r - l + 1
        }
    }

    fn no_extra_space(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut flag = false;
        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                flag = true;
            }
            if flag {
                min = min.min(nums[i]);
            }
        }
        flag = false;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] > nums[i + 1] {
                flag = true;
            }
            if flag {
                max = max.max(nums[i]);
            }
        }
        let mut l = 0i32;
        while l < nums.len() as i32 && min >= nums[l as usize] {
            l += 1;
        }
        let mut r = nums.len() as i32 - 1;
        while r >= 0 && max <= nums[r as usize] {
            r -= 1;
        }
        if r - l < 0 {
            0
        } else {
            r - l + 1
        }
    }

    no_extra_space(nums)
}

// https://leetcode.com/problems/min-stack/
struct ListNode {
    val: i32,
    min: i32,
    next: Option<Box<ListNode>>,
}
struct MinStack {
    top: Option<Box<ListNode>>,
}
impl MinStack {
    fn new() -> Self {
        Self { top: None }
    }

    fn push(&mut self, val: i32) {
        let mut new_top = ListNode {
            val,
            min: val,
            next: None,
        };
        if let Some(h) = self.top.take() {
            if h.min < new_top.min {
                new_top.min = h.min;
            }
            new_top.next = Some(h);
        }
        self.top = Some(Box::new(new_top));
    }

    fn pop(&mut self) {
        if let Some(h) = self.top.take() {
            self.top = h.next;
        }
    }

    fn top(&self) -> i32 {
        self.top.as_ref().map(|t| t.val).unwrap()
    }

    fn get_min(&self) -> i32 {
        self.top.as_ref().map(|t| t.min).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test318() {
        println!(
            "{}",
            min_jumps(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404])
        );
    }

    #[test]
    fn test319() {
        println!(
            "{}",
            increasing_triplet(vec![100, -23, -23, 404, 100, 23, 23, 23, 3, 404])
        ); // true
        println!("{}", increasing_triplet(vec![5, 4, 3, 2, 1])); // false
        println!("{}", increasing_triplet(vec![20, 100, 10, 12, 5, 13])); // true
    }

    #[test]
    fn test320() {
        println!("{}", find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15])); // 5
        println!("{}", find_unsorted_subarray(vec![1, 2, 3, 4])); // 0
        println!("{}", find_unsorted_subarray(vec![1])); // 0
        println!("{}", find_unsorted_subarray(vec![2, 1])); // 2
        println!("{}", find_unsorted_subarray(vec![1, 3, 2, 3, 3])); // 2
    }

    #[test]
    fn test321() {
        let mut min_stack = MinStack::new();
        min_stack.push(0);
        min_stack.push(1);
        min_stack.push(0);
        println!("{}", min_stack.get_min());
        min_stack.pop();
        println!("{}", min_stack.get_min());
    }
}
