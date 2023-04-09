// https://leetcode.com/problems/largest-color-value-in-a-directed-graph/description/
// https://leetcode.com/problems/largest-color-value-in-a-directed-graph/editorial/
pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
    fn using_dfs_topo_sort(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        fn dfs(
            v: usize,
            adj: &[Vec<usize>],
            visited: &mut [i32],
            count: &mut [Vec<i32>],
            colors: &[char],
        ) -> i32 {
            if visited[v] == 2 {
                return count[v][colors[v] as usize - 'a' as usize];
            }
            if visited[v] == 1 {
                return i32::MAX;
            }
            visited[v] = 1;
            for &u in &adj[v] {
                if dfs(u, adj, visited, count, colors) == i32::MAX {
                    return i32::MAX;
                }
                for i in 0..26 {
                    count[v][i] = count[v][i].max(count[u][i]);
                }
            }
            count[v][colors[v] as usize - 'a' as usize] += 1;
            visited[v] = 2;
            count[v][colors[v] as usize - 'a' as usize]
        }
        let mut adj = vec![vec![]; colors.len()];
        let colors = colors.chars().collect::<Vec<_>>();
        for e in edges {
            adj[e[0] as usize].push(e[1] as usize);
        }
        let mut count = vec![vec![0; 26]; colors.len()];
        let mut visited = vec![0; colors.len()];
        let mut ans = 0;
        for i in 0..colors.len() {
            ans = ans.max(dfs(i, &adj, &mut visited, &mut count, &colors));
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
    fn using_khan_topo_sort(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;
        let mut adj = vec![vec![]; colors.len()];
        let mut in_degree = vec![0; colors.len()];
        let colors = colors.chars().collect::<Vec<_>>();
        for e in edges {
            adj[e[0] as usize].push(e[1] as usize);
            in_degree[e[1] as usize] += 1;
        }
        let mut count = vec![vec![0; 26]; colors.len()];
        let mut visited = 0;
        let mut ans = 0;
        let mut queue = VecDeque::new();
        for node in 0..in_degree.len() {
            if in_degree[node] == 0 {
                queue.push_back(node);
            }
        }
        while let Some(v) = queue.pop_front() {
            visited += 1;
            count[v][colors[v] as usize - 'a' as usize] += 1;
            ans = ans.max(count[v][colors[v] as usize - 'a' as usize]);
            for &u in &adj[v] {
                for i in 0..26usize {
                    count[u][i] = count[u][i].max(count[v][i]);
                }
                in_degree[u] -= 1;
                if in_degree[u] == 0 {
                    queue.push_back(u);
                }
            }
        }
        if visited == colors.len() {
            ans
        } else {
            -1
        }
    }
    using_dfs_topo_sort(colors, edges)
}

// https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/description/
pub fn remove_duplicates(s: String) -> String {
    let mut ans = String::new();
    for ch in s.chars() {
        if !ans.is_empty() && ans.chars().last().unwrap() == ch {
            ans.pop();
        } else {
            ans.push(ch);
        }
    }
    ans
}

// https://leetcode.com/problems/simplify-path/description/
// https://leetcode.com/problems/simplify-path/editorial/
pub fn simplify_path(path: String) -> String {
    let mut stack = vec![];
    let dir_elems = path.split('/').collect::<Vec<_>>();
    for dir in dir_elems {
        if dir.is_empty() || dir == "." {
            continue;
        } else if dir == ".." {
            if !stack.is_empty() {
                stack.pop();
            }
        } else {
            stack.push(dir)
        }
    }
    let ans = stack.into_iter().fold(String::new(), |mut acc, v| {
        acc.push('/');
        acc.push_str(v);
        acc
    });
    if ans.is_empty() {
        "/".to_string()
    } else {
        ans
    }
}

// https://leetcode.com/problems/make-the-string-great/description/
// https://leetcode.com/problems/make-the-string-great/editorial/
pub fn make_good(s: String) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    let mut end = 0;
    for cur in 0..s.len() {
        if end > 0 && (s[cur] as i32 - s[end - 1] as i32).abs() == 32 {
            end -= 1;
        } else {
            s[end] = s[cur];
            end += 1;
        }
    }
    s[0..end].into_iter().collect()
}

// https://leetcode.com/problems/number-of-recent-calls/
use std::collections::VecDeque;

struct RecentCounter {
    queue: VecDeque<i32>,
}
impl RecentCounter {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
    fn ping(&mut self, t: i32) -> i32 {
        while !self.queue.is_empty() && *self.queue.front().unwrap() < t - 3000 {
            self.queue.pop_front();
        }
        self.queue.push_back(t);
        self.queue.len() as i32
    }
}

// https://leetcode.com/problems/moving-average-from-data-stream/description/
// https://leetcode.com/problems/moving-average-from-data-stream/editorial/
struct MovingAverage {
    queue: VecDeque<i32>,
    sum: i32,
    size: usize,
}
impl MovingAverage {
    fn new(size: i32) -> Self {
        Self {
            queue: VecDeque::new(),
            size: size as usize,
            sum: 0,
        }
    }
    fn next(&mut self, val: i32) -> f64 {
        if self.queue.len() >= self.size {
            self.sum -= self.queue.pop_front().unwrap();
        }
        self.queue.push_back(val);
        self.sum += val;
        (self.sum as f64) / self.queue.len() as f64
    }
}

// https://leetcode.com/problems/longest-continuous-subarray-with-absolute-diff-less-than-or-equal-to-limit/
pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    use std::collections::VecDeque;
    let mut increasing = VecDeque::new();
    let mut decreasing = VecDeque::new();
    let mut ans = 0;
    let mut start = 0;
    for end in 0..nums.len() {
        while !increasing.is_empty() && *increasing.back().unwrap() > nums[end] {
            increasing.pop_back();
        }
        while !decreasing.is_empty() && *decreasing.back().unwrap() < nums[end] {
            decreasing.pop_back();
        }
        increasing.push_back(nums[end]);
        decreasing.push_back(nums[end]);
        while *decreasing.front().unwrap() - *increasing.front().unwrap() > limit {
            if *decreasing.front().unwrap() == nums[start] {
                decreasing.pop_front();
            }
            if *increasing.front().unwrap() == nums[start] {
                increasing.pop_front();
            }
            start += 1;
        }
        ans = ans.max(end - start + 1);
    }
    ans as i32
}

// https://leetcode.com/problems/online-stock-span/description/
// https://leetcode.com/problems/online-stock-span/editorial/
struct StockSpanner {
    stack: Vec<(i32, i32)>,
}
impl StockSpanner {
    fn new() -> Self {
        Self { stack: vec![] }
    }
    fn next(&mut self, price: i32) -> i32 {
        let mut ans = 1;
        while !self.stack.is_empty() && self.stack.last().unwrap().0 <= price {
            ans += self.stack.pop().unwrap().1;
        }
        self.stack.push((price, ans));
        ans
    }
}

// https://leetcode.com/problems/removing-stars-from-a-string/
pub fn remove_stars(s: String) -> String {
    fn stacks_approach(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();
        let mut res = vec![];
        for i in 0..s.len() {
            if s[i] == '*' {
                if !res.is_empty() {
                    res.pop();
                }
            } else {
                res.push(s[i]);
            }
        }
        res.into_iter().collect()
    }
    fn two_pointers_approach(s: String) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        let mut j = 0;
        for i in 0..s.len() {
            if s[i] == '*' {
                j -= 1;
            } else {
                s[j] = s[i];
                j += 1;
            }
        }
        s[0..j].iter().collect()
    }
    two_pointers_approach(s)
}

// https://leetcode.com/problems/minimum-amount-of-time-to-collect-garbage/description/
pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
    // For each type of garbage, find the house with the highest index that has at least 1 unit of this type of garbage.
    let mut paper_max = 0;
    let mut glass_max = 0;
    let mut metal_max = 0;
    for i in 0..garbage.len() {
        if garbage[i].contains('P') {
            paper_max = i;
        }
        if garbage[i].contains('G') {
            glass_max = i;
        }
        if garbage[i].contains('M') {
            metal_max = i;
        }
    }
    let mut ans = 0;
    for i in 0..garbage.len() {
        if i > 0 {
            if glass_max >= i {
                ans += travel[i - 1];
            }
            if paper_max >= i {
                ans += travel[i - 1];
            }
            if metal_max >= i {
                ans += travel[i - 1];
            }
        }
        ans += garbage[i].len() as i32;
    }
    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test418() {
        println!(
            "{}",
            largest_path_value(
                "abaca".to_string(),
                vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]]
            )
        ); // 3
        println!("{}", largest_path_value("a".to_string(), vec![vec![0, 0]])); // -1
    }

    #[test]
    fn test419() {
        println!("{}", remove_duplicates("abbaca".to_string())); // ca
    }

    #[test]
    fn test420() {
        println!("{}", simplify_path("/../".to_string())); // /
        println!("{}", simplify_path("/home//foo/".to_string())); // /home/foo
    }

    #[test]
    fn test421() {
        println!("{}", make_good("leEeetcode".to_string())); // leetcode
        println!("{}", make_good("abBAcC".to_string())); // ""
    }

    #[test]
    fn test422() {
        println!("{}", longest_subarray(vec![8, 2, 4, 7], 4)); // 2
        println!("{}", longest_subarray(vec![10, 1, 2, 4, 7, 2], 5)); // 4
        println!(
            "{}",
            longest_subarray(vec![2, 2, 2, 4, 4, 2, 5, 5, 5, 5, 5, 2], 2)
        ); // 4
    }

    #[test]
    fn test423() {
        println!("{}", remove_stars("leet**cod*e".to_string())); // leecoe
    }

    #[test]
    fn test424() {
        println!(
            "{}",
            garbage_collection(
                vec![
                    "G".to_string(),
                    "P".to_string(),
                    "GP".to_string(),
                    "GG".to_string(),
                ],
                vec![2, 4, 3]
            )
        ); // 21

        println!(
            "{}",
            garbage_collection(
                vec!["MMM".to_string(), "PGM".to_string(), "GP".to_string(),],
                vec![3, 10]
            )
        ); // 37
    }
}
