// https://leetcode.com/problems/maximal-network-rank/description/
// max number of connected cities
pub fn maximal_network_rank_wrong(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    if roads.is_empty() {
        return 0;
    }
    struct DSU {
        repr: Vec<usize>,
        rank: Vec<usize>,
        max_rank: usize,
    }
    impl DSU {
        fn new(sz: usize) -> Self {
            let mut repr = Vec::with_capacity(sz);
            for i in 0..sz {
                repr.push(i);
            }
            Self {
                repr,
                rank: vec![1; sz],
                max_rank: 1,
            }
        }
        fn same(&mut self, x: usize, y: usize) -> bool {
            self.find(x) == self.find(y)
        }
        fn find(&mut self, x: usize) -> usize {
            if self.repr[x] != x {
                self.repr[x] = self.find(self.repr[x]);
            }
            self.repr[x]
        }
        fn union(&mut self, x: usize, y: usize) {
            let (mut x, mut y) = (self.find(x), self.find(y));
            if x == y {
                return;
            }
            if self.rank[x] < self.rank[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.repr[x] = y;
            self.rank[y] += self.rank[x];
            self.max_rank = self.max_rank.max(self.rank[y]);
        }
    }

    let mut dsu = DSU::new(n as usize);
    for road in roads {
        let (x, y) = (road[0] as usize, road[1] as usize);
        if !dsu.same(x, y) {
            dsu.union(x, y);
        }
    }

    dsu.max_rank as i32
}

pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let mut adj = vec![vec![false; n as usize]; n as usize];
    let mut degrees = vec![0; n as usize];
    for road in roads.iter() {
        let (a, b) = (road[0] as usize, road[1] as usize);
        degrees[a] += 1;
        degrees[b] += 1;
        adj[a][b] = true;
        adj[b][a] = true;
    }
    let mut ans = 0;
    println!("{:?}", adj);
    println!("{:?}", degrees);
    for x in 0..n as usize {
        for y in 0..n as usize {
            if x == y {
                continue;
            }
            let add = if adj[x][y] { -1 } else { 0 };
            ans = ans.max(degrees[x] + degrees[y] + add);
        }
    }
    ans
}

// https://leetcode.com/problems/largest-rectangle-in-histogram/description/
// https://leetcode.com/problems/largest-rectangle-in-histogram/solutions/28941/segment-tree-solution-just-another-idea-onlogn-solution/
pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    fn divide_and_conquer(heights: Vec<i32>) -> i32 {
        fn calc_area(heights: &Vec<i32>, start: i32, end: i32) -> i32 {
            if start > end {
                return 0;
            }
            let mut min_idx = start;
            for i in start..=end {
                if heights[min_idx as usize] >= heights[i as usize] {
                    min_idx = i;
                }
            }

            (heights[min_idx as usize] * (end - start + 1)).max(
                calc_area(heights, start, min_idx - 1).max(calc_area(heights, min_idx + 1, end)),
            )
        }

        calc_area(&heights, 0, heights.len() as i32 - 1)
    }

    fn stack_approach(heights: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut max_area = 0;
        for right in 0..=heights.len() {
            while !stack.is_empty()
                && (right == heights.len() || heights[*stack.last().unwrap()] >= heights[right])
            {
                let mid = stack.pop().unwrap();
                let h = heights[mid];
                let left = stack.last().map(|x| *x as i32).unwrap_or(-1);
                max_area = max_area.max(h * (right as i32 - left - 1));
            }
            stack.push(right);
        }
        max_area
    }

    stack_approach(heights)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test504() {
        println!("{}", maximal_network_rank(2, vec![vec![1, 0]]));
    }
}
