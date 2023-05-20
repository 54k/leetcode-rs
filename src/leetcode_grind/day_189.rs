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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test504() {
        println!("{}", maximal_network_rank(2, vec![vec![1, 0]]));
    }
}
