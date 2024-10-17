// https://leetcode.com/problems/largest-component-size-by-common-factor/description/
struct DSU {
    repr: Vec<usize>,
    sz: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        let mut repr = vec![0; n + 1];
        for i in 1..=n {
            repr[i] = i;
        }
        Self {
            repr,
            sz: vec![1; n + 1],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.repr[x] != x {
            self.repr[x] = self.find(self.repr[x]);
        }
        self.repr[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let (mut x, mut y) = (self.find(x), self.find(y));
        if x == y {
            return false;
        }

        if self.sz[x] > self.sz[y] {
            std::mem::swap(&mut x, &mut y);
        }

        self.repr[x] = y;
        self.sz[y] += self.sz[x];
        true
    }
}

fn prime_decompose(mut num: usize) -> Vec<usize> {
    let mut primes = vec![];
    let mut factor = 2;
    while num >= factor * factor {
        if num % factor == 0 {
            primes.push(factor);
            num /= factor;
        } else {
            factor += 1;
        }
    }
    primes.push(num);
    primes
}

pub fn largest_component_size(nums: Vec<i32>) -> i32 {
    use std::collections::{HashMap, HashSet};
    let nums = nums.into_iter().map(|x| x as usize).collect::<Vec<_>>();
    let max_val = *nums.iter().max().unwrap();
    let mut dsu = DSU::new(max_val);
    let mut num_factor_map = HashMap::new();

    for &num in &nums {
        let mut prime_factors = prime_decompose(num)
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        prime_factors.sort();
        num_factor_map.insert(num, prime_factors[0]);
        // println!("primes for {:?} is {:?}", num, prime_factors);
        for i in 0..prime_factors.len() - 1 {
            // println!("union {} and {}", prime_factors[i], prime_factors[i + 1]);
            let _res = dsu.union(prime_factors[i], prime_factors[i + 1]);
            // println!("union res {}", _res);
        }
    }

    let mut max_group_size = 0;
    let mut group_count = HashMap::new();
    for num in nums {
        let group_id = dsu.find(num_factor_map[&num]);
        let mut cnt = group_count.entry(group_id).or_insert(0);
        *cnt += 1;
        max_group_size = max_group_size.max(*cnt);
    }
    max_group_size as i32
}
