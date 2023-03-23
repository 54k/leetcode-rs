use crate::grind_169::week_5::odd_even_list;

// https://leetcode.com/problems/number-of-operations-to-make-network-connected/description/
pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    fn make_adj(n: i32, connections: &Vec<Vec<i32>>) -> (Vec<Vec<usize>>, Vec<bool>) {
        let mut ans = vec![vec![]; n as usize];
        for c in connections {
            ans[c[0] as usize].push(c[1] as usize);
            ans[c[1] as usize].push(c[0] as usize);
        }
        (ans, vec![false; n as usize])
    }
    fn using_dfs(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        fn dfs(v: usize, visited: &mut Vec<bool>, adj: &Vec<Vec<usize>>) {
            visited[v] = true;
            for &u in &adj[v] {
                if !visited[u] {
                    dfs(u, visited, adj)
                }
            }
        }
        if connections.len() < n as usize - 1 {
            return -1;
        }
        let (adj, mut visited) = make_adj(n, &connections);
        let mut cmp_count = 0;
        for v in 0..n as usize {
            if !visited[v] {
                cmp_count += 1;
                dfs(v, &mut visited, &adj);
            }
        }
        cmp_count - 1
    }
    fn using_bfs(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        fn bfs(v: usize, visited: &mut Vec<bool>, adj: &Vec<Vec<usize>>) {
            use std::collections::VecDeque;
            let mut queue = VecDeque::new();
            queue.push_back(v);
            visited[v] = true;
            while let Some(n) = queue.pop_front() {
                for &u in &adj[n] {
                    if !visited[u] {
                        visited[u] = true;
                        queue.push_back(u);
                    }
                }
            }
        }
        if connections.len() < n as usize - 1 {
            return -1;
        }
        let (adj, mut visited) = make_adj(n, &connections);
        let mut cmp_count = 0;
        for v in 0..n as usize {
            if !visited[v] {
                cmp_count += 1;
                bfs(v, &mut visited, &adj);
            }
        }
        cmp_count - 1
    }
    fn using_union_find(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        struct UnionFind {
            links: Vec<usize>,
            size: Vec<i32>,
        }
        impl UnionFind {
            fn new(n: usize) -> Self {
                let links = (0..n).collect::<Vec<_>>();
                Self {
                    links,
                    size: vec![1; n],
                }
            }
            fn find(&mut self, x: usize) -> usize {
                if self.links[x] != x {
                    self.links[x] = self.find(self.links[x]);
                }
                self.links[x]
            }
            fn union(&mut self, x: usize, y: usize) {
                let mut x = self.find(x);
                let mut y = self.find(y);
                if self.size[x] > self.size[y] {
                    std::mem::swap(&mut x, &mut y);
                }
                self.links[x] = y;
                self.size[y] += self.size[x];
            }
        }
        if connections.len() < n as usize - 1 {
            return -1;
        }
        let mut union_find = UnionFind::new(n as usize);
        let mut num_of_components = n;
        for connection in connections {
            let from = connection[0] as usize;
            let to = connection[1] as usize;
            if union_find.find(from) != union_find.find(to) {
                num_of_components -= 1;
                union_find.union(from, to);
            }
        }
        num_of_components - 1
    }
    using_union_find(n, connections)
}

// https://leetcode.com/problems/reverse-nodes-in-k-group/
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    fn rec(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut next = &mut head;
        for _ in 0..k {
            if let Some(n) = next {
                next = &mut n.next;
            } else {
                return head;
            }
        }
        let mut ret = rec(next.take(), k);
        while let Some(n) = head.take() {
            ret = Some(Box::new(ListNode {
                val: n.val,
                next: ret,
            }));
            head = n.next;
        }
        ret
    }
    rec(head, k)
}

// https://leetcode.com/problems/random-pick-index/description/
// https://leetcode.com/problems/random-pick-index/editorial/
mod rpi_brute_force {
    use rand::rngs::ThreadRng;
    use rand::Rng;

    pub struct Solution {
        nums: Vec<i32>,
        rnd: ThreadRng,
    }

    impl Solution {
        pub fn new(nums: Vec<i32>) -> Self {
            Self {
                nums,
                rnd: rand::thread_rng(),
            }
        }
        pub fn pick(&mut self, target: i32) -> i32 {
            let mut indicies = vec![];
            for (idx, &num) in self.nums.iter().enumerate() {
                if num == target {
                    indicies.push(idx as i32)
                }
            }
            indicies[self.rnd.gen::<usize>() % indicies.len()]
        }
    }
}

mod rpi_hashmap_caching {
    use rand::rngs::ThreadRng;
    use rand::Rng;
    use std::collections::HashMap;

    pub struct Solution {
        cache: HashMap<i32, Vec<i32>>,
        rnd: ThreadRng,
    }

    impl Solution {
        pub fn new(nums: Vec<i32>) -> Self {
            let cache = nums
                .into_iter()
                .enumerate()
                .fold(HashMap::new(), |mut acc, (i, num)| {
                    acc.entry(num).or_insert(vec![]).push(i as i32);
                    acc
                });
            Self {
                cache,
                rnd: rand::thread_rng(),
            }
        }
        pub fn pick(&mut self, target: i32) -> i32 {
            self.cache[&target][self.rnd.gen::<usize>() % self.cache[&target].len()]
        }
    }
}

mod rpi_reservoir_sampling {
    use rand::rngs::ThreadRng;
    use rand::Rng;

    pub struct Solution {
        nums: Vec<i32>,
        rnd: ThreadRng,
    }

    impl Solution {
        pub fn new(nums: Vec<i32>) -> Self {
            Self {
                nums,
                rnd: rand::thread_rng(),
            }
        }
        pub fn pick(&mut self, target: i32) -> i32 {
            let mut idx = 0;
            let mut count = 0;
            for i in 0..self.nums.len() {
                if self.nums[i] == target {
                    count += 1;
                    if self.rnd.gen::<i32>() % count == 0 {
                        idx = i as i32;
                    }
                }
            }
            idx
        }
    }
}

// https://leetcode.com/problems/data-stream-as-disjoint-intervals/description/
// https://leetcode.com/problems/data-stream-as-disjoint-intervals/editorial/
mod btreeset_approach {
    use std::collections::BTreeSet;

    pub struct SummaryRanges {
        nums: BTreeSet<i32>,
    }

    impl SummaryRanges {
        pub fn new() -> Self {
            Self {
                nums: BTreeSet::new(),
            }
        }

        pub fn add_num(&mut self, value: i32) {
            self.nums.insert(value);
        }

        pub fn get_intervals(&self) -> Vec<Vec<i32>> {
            let mut ans = vec![];
            if self.nums.is_empty() {
                return ans;
            }
            let mut left = *self.nums.iter().take(1).next().unwrap();
            let mut right = left;
            for &num in self.nums.iter().skip(1) {
                if num - right == 1 {
                    right = num;
                } else {
                    ans.push(vec![left, right]);
                    left = num;
                    right = num;
                }
            }
            ans.push(vec![left, right]);
            ans
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test365() {
        println!(
            "{}",
            make_connected(4, vec![vec![0, 1], vec![0, 2], vec![1, 2]])
        ); // 1
    }

    #[test]
    fn test366() {
        let list = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
        println!("{:?}", reverse_k_group(list, 2));
    }

    #[test]
    fn test367() {
        let mut rpi = rpi_brute_force::Solution::new(vec![1, 2, 3, 3, 3]);
        println!("{}", rpi.pick(3));
        println!("{}", rpi.pick(1));
        println!("{}", rpi.pick(3));

        let mut rpi = rpi_hashmap_caching::Solution::new(vec![1, 2, 3, 3, 3]);
        println!("{}", rpi.pick(3));
        println!("{}", rpi.pick(1));
        println!("{}", rpi.pick(3));

        let mut rpi = rpi_reservoir_sampling::Solution::new(vec![1, 2, 3, 3, 3]);
        println!("{}", rpi.pick(3));
        println!("{}", rpi.pick(1));
        println!("{}", rpi.pick(3));
    }

    #[test]
    fn test368() {
        let mut sr = btreeset_approach::SummaryRanges::new();
        sr.add_num(1);
        println!("{:?}", sr.get_intervals()); // [1, 1]
        sr.add_num(3);
        println!("{:?}", sr.get_intervals()); // [1, 1], [3, 3]
        sr.add_num(7);
        println!("{:?}", sr.get_intervals()); // [[1, 1], [3, 3], [7, 7]]
        sr.add_num(2);
        println!("{:?}", sr.get_intervals()); // [[1, 3], [7, 7]]
        sr.add_num(6);
        println!("{:?}", sr.get_intervals()); // [[1, 3], [6, 7]]
    }
}
