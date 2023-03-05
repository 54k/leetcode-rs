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
pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
    todo!()
}

// https://leetcode.com/problems/min-stack/
struct MinStack {}
impl MinStack {
    fn new() -> Self {
        Self {}
    }

    fn push(&self, val: i32) {}

    fn pop(&self) {}

    fn top(&self) -> i32 {
        todo!()
    }

    fn get_min(&self) -> i32 {
        todo!()
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
}
