use std::{cell::RefCell, rc::Rc};

// https://leetcode.com/problems/number-of-music-playlists/description/
pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
    const MOD: i64 = 1000000007;
    let mut dp = vec![vec![0i64; n as usize + 1]; goal as usize + 1];
    dp[0][0] = 1;
    for i in 1..=goal as usize {
        for j in 1..=(i as usize).min(n as usize) {
            dp[i][j] = (dp[i][j] + dp[i - 1][j - 1] * (n as i64 - j as i64 + 1)) % MOD;
            if j > k as usize {
                dp[i][j] = (dp[i][j] + dp[i - 1][j] * (j as i64 - k as i64)) % MOD;
            }
        }
    }
    dp[goal as usize][n as usize] as i32
}

// https://leetcode.com/problems/faulty-keyboard/
pub fn final_string(s: String) -> String {
    let s = s.chars().collect::<Vec<_>>();
    let mut ans = vec![];
    for i in 0..s.len() {
        if s[i] == 'i' {
            ans.reverse();
        } else {
            ans.push(s[i]);
        }
    }
    ans.into_iter().collect()
}

// https://leetcode.com/problems/check-if-it-is-possible-to-split-array/
pub fn can_split_array(nums: Vec<i32>, m: i32) -> bool {
    for i in 0..nums.len() {
        if nums[i] + nums[i + 1] >= m {
            return true;
        }
    }
    nums.len() <= 2
}

// https://leetcode.com/problems/find-the-safest-path-in-a-grid/description/
pub fn maximum_safeness_factor_tle(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashSet;
    const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let (m, n) = (grid.len(), grid[0].len());

    let mut thieves = vec![];
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                thieves.push((i as i32, j as i32));
            }
        }
    }

    let mut ans = 0;

    fn dfs(
        (x, y): (i32, i32),
        grid: &Vec<Vec<i32>>,
        m: usize,
        n: usize,
        thieves: &Vec<(i32, i32)>,
        visited: &mut HashSet<(i32, i32)>,
        safeness: i32,
        ans: &mut i32,
    ) {
        if 0 > x || 0 > y || m as i32 <= x || m as i32 <= y {
            return;
        }

        if x + 1 == m as i32 && y + 1 == n as i32 {
            *ans = (*ans).max(safeness);
        }

        for (i, j) in &DIRS {
            let (nx, ny) = (x + *i, y + *j);
            if !visited.contains(&(nx, ny)) {
                visited.insert((nx, ny));
                let mut safe = i32::MAX;
                for &t in thieves {
                    safe = safe.min((nx - t.0).abs() + (ny - t.1).abs());
                }
                dfs(
                    (nx, ny),
                    grid,
                    m,
                    n,
                    thieves,
                    visited,
                    safeness.min(safe),
                    ans,
                );
            }
        }
    }

    let mut safeness = i32::MAX;
    for &t in &thieves {
        safeness = safeness.min((0 - t.0).abs() + (0 - t.1).abs());
    }

    dfs(
        (0, 0),
        &grid,
        m,
        n,
        &thieves,
        &mut HashSet::new(),
        safeness,
        &mut ans,
    );

    ans
}

// https://leetcode.com/problems/find-the-safest-path-in-a-grid/description/
pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::VecDeque;
    const DIR: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    fn is_valid((x, y): (i32, i32), n: i32, visited: &Vec<Vec<bool>>) -> bool {
        if 0 <= x && x < n && 0 <= y && y < n && !visited[x as usize][y as usize] {
            return true;
        }
        false
    }

    fn is_safe(dist_to_thief: &Vec<Vec<i32>>, safe_dist: i32) -> bool {
        if dist_to_thief[0][0] < safe_dist {
            return false;
        }

        let n = dist_to_thief.len();
        let mut visited = vec![vec![false; n]; n];
        visited[0][0] = true;

        let mut q = VecDeque::new();
        q.push_back((0i32, 0i32));

        while let Some((x, y)) = q.pop_front() {
            if x == n as i32 - 1 && y == n as i32 - 1 {
                return true;
            }

            for &d in &DIR {
                let (nx, ny) = (x + d.0, y + d.1);
                if !is_valid((nx, ny), n as i32, &visited) {
                    continue;
                }
                if dist_to_thief[nx as usize][ny as usize] < safe_dist {
                    continue;
                }
                visited[nx as usize][ny as usize] = true;
                q.push_back((nx, ny));
            }
        }

        false
    }

    let n = grid.len();

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    let mut q = VecDeque::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 1 {
                visited[r][c] = true;
                q.push_back((r as i32, c as i32));
            }
        }
    }

    let mut dist_to_thief = vec![vec![0; grid.len()]; grid[0].len()];

    let mut dist = 0;
    while q.len() > 0 {
        let mut sz = q.len();
        while sz > 0 {
            let (x, y) = q.pop_front().unwrap();
            dist_to_thief[x as usize][y as usize] = dist;
            sz -= 1;

            for &d in &DIR {
                let (nx, ny) = (x + d.0, y + d.1);
                if !is_valid((nx, ny), n as i32, &visited) {
                    continue;
                }
                visited[nx as usize][ny as usize] = true;
                q.push_back((nx, ny));
            }
        }
        dist += 1;
    }

    let (mut left, mut right) = (0, i32::MAX);
    let mut ans = 0;
    while left <= right {
        let mid = left + (right - left) / 2;
        if is_safe(&dist_to_thief, mid) {
            ans = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    ans
}

// https://leetcode.com/problems/maximum-elegance-of-a-k-length-subsequence/
pub fn find_maximum_elegance(mut items: Vec<Vec<i32>>, k: i32) -> i64 {
    use std::collections::HashSet;
    items.sort_by(|a, b| b[0].cmp(&a[0]));
    let mut res = 0;
    let mut cur = 0;
    let mut dup = vec![];
    let mut seen = HashSet::new();

    for i in 0..items.len() {
        if (i as i32) < k {
            if seen.contains(&items[i][1]) {
                dup.push(items[i][0] as i64);
            }
            cur += items[i][0] as i64;
        } else if !seen.contains(&items[i][1]) {
            if dup.is_empty() {
                break;
            }
            cur += items[i][0] as i64 - dup.pop().unwrap();
        }
        seen.insert(items[i][1]);
        res = res.max(cur + (seen.len() as i64) * (seen.len() as i64));
    }
    res
}

// https://leetcode.com/problems/distribute-coins-in-binary-tree/description/
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    type Node = Option<Rc<RefCell<TreeNode>>>;
    fn dfs(root: Node, ans: &mut i32) -> i32 {
        if let Some(n) = root {
            let n = n.borrow();
            let left = n.left.clone();
            let right = n.right.clone();
            let l = dfs(left.clone(), ans);
            let r = dfs(right.clone(), ans);
            *ans += l.abs() + r.abs();
            return n.val + l + r - 1;
        }
        0
    }
    let mut ans = 0;
    dfs(root, &mut ans);
    ans
}

#[test]
fn max_safe_factor() {
    let ans = maximum_safeness_factor(vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
        vec![1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
        vec![1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
    ]);

    println!("{}", ans);
    let ans = maximum_safeness_factor(vec![
        vec![0, 0, 0, 1],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![1, 0, 0, 0],
    ]);
    println!("{}", ans);
}
