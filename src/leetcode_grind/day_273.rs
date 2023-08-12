// https://leetcode.com/problems/unique-paths-ii/description/
pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
    let mut dp = vec![0; n];
    dp[0] = 1;
    for row in 0..m {
        let mut next = vec![0; n];
        for col in 0..n {
            if obstacle_grid[row][col] == 0 {
                next[col] += dp[col];
                if col > 0 && obstacle_grid[row][col - 1] == 0 {
                    next[col] += next[col - 1];
                }
            }
        }
        dp = next;
    }
    dp[n - 1]
}

// https://leetcode.com/problems/describe-the-painting/description/
pub fn split_painting(segments: Vec<Vec<i32>>) -> Vec<Vec<i64>> {
    let mut mapping = vec![0; 100002];
    let mut ends = vec![false; 100002];
    for seg in segments {
        let (s, e, v) = (seg[0] as usize, seg[1] as usize, seg[2] as i64);
        mapping[s] += v;
        mapping[e] -= v;
        ends[s] = true;
        ends[e] = true;
    }

    let mut res = vec![];
    let mut prev = 0;
    let mut sum = 0;

    for i in 0..mapping.len() {
        if ends[i] && sum > 0 {
            res.push(vec![prev as i64, i as i64, sum]);
        }
        sum += mapping[i];
        if ends[i] {
            prev = i;
        }
    }
    res
}

// https://leetcode.com/problems/data-stream-as-disjoint-intervals/description/
mod sum_ranges_sorted_set {
    use std::collections::BTreeSet;
    struct SummaryRanges {
        set: BTreeSet<i32>,
    }

    impl SummaryRanges {
        fn new() -> Self {
            Self {
                set: BTreeSet::new(),
            }
        }

        fn add_num(&mut self, value: i32) {
            self.set.insert(value);
        }

        fn get_intervals(&self) -> Vec<Vec<i32>> {
            if self.set.is_empty() {
                return vec![];
            }
            let mut ans = vec![];
            let (mut left, mut right) = (-1, -1);
            for &val in &self.set {
                if left < 0 {
                    left = val;
                    right = val;
                } else if (val == right + 1) {
                    right = val;
                } else {
                    ans.push(vec![left, right]);
                    left = val;
                    right = val;
                }
            }
            ans.push(vec![left, right]);
            ans
        }
    }
}

// https://leetcode.com/problems/the-skyline-problem/
pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::BTreeSet;
    use std::collections::HashMap;

    let mut set = BTreeSet::new();
    for b in &buildings {
        set.insert(b[0]);
        set.insert(b[1]);
    }

    let edges = set.into_iter().collect::<Vec<_>>();
    let edge_idx_map = edges
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<HashMap<i32, usize>>();

    let mut heights = vec![0; edges.len()];
    for building in buildings {
        let left = edge_idx_map[&building[0]];
        let right = edge_idx_map[&building[1]];
        let height = building[2];

        for idx in left..right {
            heights[idx] = heights[idx].max(height);
        }
    }

    let mut ans: Vec<Vec<i32>> = vec![];
    for i in 0..heights.len() {
        let curr_height = heights[i];
        let curr_pos = edges[i];

        if ans.is_empty() || (ans[ans.len() - 1][1] != curr_height) {
            ans.push(vec![curr_pos, curr_height]);
        }
    }
    ans
}

// https://leetcode.com/problems/amount-of-new-area-painted-each-day/description/
pub fn amount_painted(paint: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::BTreeMap;
    let mut records = vec![];
    let mut max_pos = 0;
    for i in 0..paint.len() {
        let (start, end) = (paint[i][0], paint[i][1]);
        records.push((start as usize, i, 1)); // use 1 and -1 to record the type
        records.push((end as usize, i, -1));
        max_pos = max_pos.max(end as usize);
    }
    records.sort();

    let mut ans = vec![0; paint.len()];
    let mut indexes = BTreeMap::new();
    let mut i = 0;

    for pos in 0..=max_pos {
        while i < records.len() && records[i].0 == pos {
            let (index, t) = (records[i].1, records[i].2);
            if t == 1 {
                *indexes.entry(index).or_insert(0) += 1;
            } else if t == -1 {
                *indexes.entry(index).or_insert(0) -= 1;
                if indexes[&index] <= 0 {
                    indexes.remove(&index);
                }
            }
            i += 1;
        }
        if indexes.len() > 0 {
            ans[*indexes.keys().take(1).find(|_| true).unwrap()] += 1;
        }
    }
    ans
}
