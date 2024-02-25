// https://leetcode.com/problems/greatest-common-divisor-traversal/description
pub fn can_traverse_all_pairs(nums: Vec<i32>) -> bool {
    if nums.len() == 1 {
        return true;
    }
    struct Dsu {
        dsu: Vec<usize>,
        rank: Vec<i32>,
    }
    impl Dsu {
        fn new(n: usize) -> Self {
            let mut dsu = vec![0; n];
            for i in 0..n {
                dsu[i] = i;
            }
            let rank = vec![1; n];
            Dsu { dsu, rank }
        }

        fn find(&mut self, x: usize) -> usize {
            if self.dsu[x] != x {
                self.dsu[x] = self.find(self.dsu[x]);
            }
            self.dsu[x]
        }

        fn union(&mut self, x: usize, y: usize) {
            let (mut px, mut py) = (self.find(x), self.find(y));
            if px == py {
                return;
            }
            if (self.rank[px] < self.rank[py]) {
                std::mem::swap(&mut px, &mut py);
            }
            self.dsu[py] = px;
            self.rank[px] += self.rank[py];
        }
    }

    const MAX: usize = 100000;
    let mut sieve = vec![0; MAX + 1];
    for d in 2..=MAX {
        if sieve[d] == 0 {
            let mut v = d;
            while v <= MAX {
                sieve[v] = d;
                v += d;
            }
        }
    }

    let mut dsu = Dsu::new(MAX * 2 + 1);
    for &x in &nums {
        let mut v = x as usize;
        while v > 1 {
            let prime = sieve[v];
            let root = MAX + prime;

            if (dsu.find(x as usize) != dsu.find(root)) {
                dsu.union(x as usize, root);
                // println!("union {} {} {} {}", x, root, dsu.find(root), dsu.find(x as usize));
            }

            while v % prime == 0 {
                v /= prime;
            }
        }
    }

    let mut cnt = 0;
    for x in nums.into_iter().collect::<std::collections::HashSet<_>>() {
        if x == 1 {
            return false;
        }
        // println!("x par {}={}", x, dsu.find(x as usize));
        if dsu.find(x as usize) == x as usize {
            cnt += 1;
        }
    }
    // println!("{cnt}");
    cnt == 1
}
