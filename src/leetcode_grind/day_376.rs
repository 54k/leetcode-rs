// https://leetcode.com/problems/critical-connections-in-a-network/
pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut tin = vec![0; n as usize];
    let mut low = vec![0; n as usize];
    let mut vis = vec![false; n as usize];
    let mut adj = vec![vec![]; n as usize];

    for c in connections {
        adj[c[0] as usize].push(c[1] as usize);
        adj[c[1] as usize].push(c[0] as usize);
    }

    fn dfs(
        v: usize,
        p: usize,
        tin: &mut Vec<i32>,
        low: &mut Vec<i32>,
        adj: &Vec<Vec<usize>>,
        vis: &mut Vec<bool>,
        res: &mut Vec<Vec<i32>>,
    ) {
        tin[v] = if p == v { 0 } else { tin[p] + 1 };
        low[v] = tin[v];
        vis[v] = true;

        for &u in &adj[v] {
            if u == p {
                continue;
            }

            if vis[u] {
                low[v] = low[v].min(tin[u]);
            } else {
                dfs(u, v, tin, low, adj, vis, res);
                low[v] = low[v].min(low[u]);

                if tin[v] < low[u] {
                    res.push(vec![v as i32, u as i32]);
                }
            }
        }
    }

    dfs(0, 0, &mut tin, &mut low, &adj, &mut vis, &mut res);
    res
}

// https://leetcode.com/problems/arithmetic-subarrays/description/
pub fn check_arithmetic_subarrays_1(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
    let mut ans = vec![];
    for i in 0..l.len() {
        let mut seq = nums[l[i] as usize..=r[i] as usize]
            .iter()
            .copied()
            .collect::<Vec<_>>();
        seq.sort();

        println!("{:?}", seq);
        let mut is = true;
        let mut diff = -1;
        for j in 1..seq.len() {
            let d = seq[j] - seq[j - 1];
            if diff == -1 {
                diff = d;
                continue;
            }
            if diff != d {
                is = false;
                break;
            }
        }
        ans.push(is);
    }
    ans
}

pub fn check_arithmetic_subarrays_2(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
    let mut ans = vec![];
    'outer: for i in 0..l.len() {
        let mut seq = nums[l[i] as usize..=r[i] as usize]
            .iter()
            .copied()
            .collect::<Vec<_>>();
        let mut min = i32::MAX;
        let mut max = i32::MIN;

        use std::collections::HashSet;
        let mut set = HashSet::new();
        for j in 0..seq.len() {
            min = min.min(seq[j]);
            max = max.max(seq[j]);
            set.insert(seq[j]);
        }

        let n = seq.len() as i32 - 1;

        if (max - min) % n != 0 {
            ans.push(false);
            continue 'outer;
        }

        let mut diff = (max - min) / n;
        let mut curr = min + diff;

        while curr < max {
            if !set.contains(&curr) {
                ans.push(false);
                continue 'outer;
            }
            curr += diff;
        }

        ans.push(true);
    }
    ans
}

// https://leetcode.com/problems/arithmetic-slices/description/
pub fn number_of_arithmetic_slices_1(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    let n = nums.len();
    if nums.len() < 3 {
        return 0;
    }
    for s in 0..n - 2 {
        let d = nums[s + 1] - nums[s];

        for e in s + 2..n {
            let mut i = s + 1;
            while i <= e {
                if d != nums[i] - nums[i - 1] {
                    break;
                }
                i += 1;
            }
            count += if i > e { 1 } else { 0 };
        }
    }

    count
}

pub fn number_of_arithmetic_slices_2(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    if (nums.len() < 3) {
        return 0;
    }
    for s in 0..nums.len() - 2 {
        let d = nums[s + 1] - nums[s];
        for e in s + 2..nums.len() {
            if nums[e] - nums[e - 1] == d {
                count += 1;
            } else {
                break;
            }
        }
    }
    count
}
