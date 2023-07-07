// https://leetcode.com/problems/maximize-the-confusion-of-an-exam/description/
pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
    pub fn max_consecutive_answers_binsearch_sliding_window(answer_key: String, k: i32) -> i32 {
        fn is_valid(answer_key: &Vec<char>, size: usize, k: i32) -> bool {
            use std::collections::HashMap;
            let n = answer_key.len();
            let mut counter = HashMap::new();

            for i in 0..size {
                let c = answer_key[i];
                *counter.entry(c).or_insert(0) += 1;
            }

            if (*counter.get(&'T').unwrap_or(&0)).min(*counter.get(&'F').unwrap_or(&0)) <= k {
                return true;
            }

            for i in size..n {
                let c1 = answer_key[i];
                *counter.entry(c1).or_insert(0) += 1;
                let c2 = answer_key[i - size];
                *counter.entry(c2).or_insert(0) -= 1;

                if (*counter.get(&'T').unwrap_or(&0)).min(*counter.get(&'F').unwrap_or(&0)) <= k {
                    return true;
                }
            }

            false
        }

        let answer_key = answer_key.chars().collect::<Vec<_>>();
        let n = answer_key.len();
        let (mut left, mut right) = (k, n as i32);
        while left < right {
            let mid = (left + right + 1) / 2;

            if is_valid(&answer_key, mid as usize, k) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left
    }

    pub fn max_consecutive_answers_sliding_window(answer_key: String, k: i32) -> i32 {
        use std::collections::HashMap;

        let mut max_size = k;
        let mut counter = HashMap::new();
        let answer_key = answer_key.chars().collect::<Vec<_>>();

        for i in 0..k as usize {
            *counter.entry(answer_key[i]).or_insert(0) += 1;
        }

        let mut left = 0;
        for right in k as usize..answer_key.len() {
            *counter.entry(answer_key[right]).or_insert(0) += 1;

            while (*counter.get(&'T').unwrap_or(&0)).min(*counter.get(&'F').unwrap_or(&0)) > k {
                *counter.entry(answer_key[left]).or_insert(0) -= 1;
                left += 1;
            }

            max_size = max_size.max((right - left + 1) as i32);
        }

        max_size
    }

    pub fn max_consecutive_answers_adv_sliding_window(answer_key: String, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut max_size = 0;
        let mut count = HashMap::new();

        let answer_key = answer_key.chars().collect::<Vec<_>>();

        for right in 0..answer_key.len() {
            *count.entry(answer_key[right]).or_insert(0) += 1;
            let minor = (*count.get(&'T').unwrap_or(&0)).min(*count.get(&'F').unwrap_or(&0));

            if minor <= k {
                max_size += 1;
            } else {
                count.insert(
                    answer_key[right - max_size],
                    *count.get(&answer_key[right - max_size]).unwrap_or(&0) - 1,
                );
            }
        }

        max_size as i32
    }

    max_consecutive_answers_adv_sliding_window(answer_key, k)
}

// https://leetcode.com/problems/count-vowel-substrings-of-a-string/description/
pub fn count_vowel_substrings(word: String) -> i32 {
    pub fn count_vowel_substrings_brute(word: String) -> i32 {
        let mut count = vec![0; 2];
        const vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

        let word = word.chars().collect::<Vec<_>>();
        let mut ans = 0;

        fn is_ok(w: &[char]) -> bool {
            for &v in &vowels {
                if !w.contains(&v) {
                    return false;
                }
            }

            for ch in w {
                if !vowels.contains(&ch) {
                    return false;
                }
            }

            true
        }

        for i in 0..word.len() {
            for j in i + 1..word.len() {
                if is_ok(&word[i..=j]) {
                    ans += 1;
                }
            }
        }

        ans
    }

    pub fn count_vowel_substrings_hash_map(word: String) -> i32 {
        use std::collections::HashMap;
        // iterate over string
        // find consonant index, we will call it c
        // after we find first vowel, lock c
        // then we start building up the vowel dictionary where each vowel maps to the most recent index that it has been seen
        // if all vowels have recent indexes, then the total combinations including this set of vowels is the smallest index - c
        let mut c = -1;
        let default_values = vec![('a', -1), ('e', -1), ('i', -1), ('o', -1), ('u', -1)];
        let mut vowels_count: HashMap<_, _> = default_values.clone().into_iter().collect();
        let mut total = 0;

        for (i, letter) in word.chars().enumerate() {
            if !vowels_count.contains_key(&letter) {
                c = i as i32;
                vowels_count = default_values.clone().into_iter().collect();
            } else {
                *vowels_count.get_mut(&letter).unwrap() = i as i32;
                let min_idx = vowels_count.values().copied().min().unwrap_or(-1);
                if min_idx > -1 {
                    total += min_idx - c;
                }
            }
        }
        total
    }

    count_vowel_substrings_hash_map(word)
}

// https://leetcode.com/problems/vowels-of-all-substrings/description/
pub fn count_vowels(word: String) -> i64 {
    // Number of substring whose the ith character would be part of = Total substrings of length n - Number of substring whose ith character won't be part of.

    // Total substrings of length n = n * (n +1)/2
    // Number of substring whose ith character won't be part of = Number of substrings b/w indices [0 to i-1] + Number of substrings b/w indices [i+1 to n-1].

    // Note : Number of substrings are not driven by index but are driven by length of string. Hence,

    // For 0 to i-1 (Having length = i) => i*(i+1)/2
    // (i+1, n-1) (Having length = n-i-1) => (n-i-1)*(n-i)/2

    // Restating,
    // Number of substring whose the ith character would be part of = Total substrings of length n - Number of substring whose ith character won't be part of.
    // => n*(n+1) / 2 - i(i+1) /2 - (n-i-1)(n-i)/2

    // On expanding and simplification, it becomes

    // => n - i^2 - i + ni
    // =>. (n-i) * (i + 1)
    let mut res = 0;
    let n = word.len();
    for (i, ch) in word.chars().enumerate() {
        if ['a', 'e', 'i', 'o', 'u'].contains(&ch) {
            res += (i as i64 + 1) * (n as i64 - i as i64);
        }
    }
    res
}

// https://leetcode.com/problems/optimize-water-distribution-in-a-village/description/

pub fn min_cost_to_supply_water(n: i32, wells: Vec<i32>, pipes: Vec<Vec<i32>>) -> i32 {
    pub fn min_cost_to_supply_water_prim(n: i32, wells: Vec<i32>, pipes: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, HashSet};

        let mut adj = vec![vec![]; n as usize + 1];
        let mut heap = BinaryHeap::new();

        for i in 0..wells.len() {
            let virtual_edge = (wells[i], i + 1);
            adj[0].push(virtual_edge);
            heap.push(Reverse(virtual_edge));
        }

        for pipe in &pipes {
            let (from, to, cost) = (pipe[0] as usize, pipe[1] as usize, pipe[2]);
            adj[from].push((cost, to));
            adj[to].push((cost, from));
        }

        let mut set = HashSet::new();
        set.insert(0);
        let mut total_cost = 0;

        while set.len() < n as usize + 1 {
            let Reverse((cost, next_house)) = heap.pop().unwrap();
            if set.contains(&next_house) {
                continue;
            }

            set.insert(next_house);
            total_cost += cost;

            for &neighbor in &adj[next_house] {
                if !set.contains(&neighbor.1) {
                    heap.push(Reverse(neighbor));
                }
            }
        }
        total_cost
    }

    pub fn min_cost_to_supply_water_kruskal(n: i32, wells: Vec<i32>, pipes: Vec<Vec<i32>>) -> i32 {
        struct UF {
            repr: Vec<usize>,
            sz: Vec<i32>,
        }

        impl UF {
            fn new(n: usize) -> Self {
                let mut r = vec![];
                for i in 0..=n {
                    r.push(i);
                }

                Self {
                    repr: r,
                    sz: vec![1; n + 1],
                }
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

                if self.sz[x] < self.sz[y] {
                    std::mem::swap(&mut x, &mut y);
                }

                self.repr[y] = x;
                self.sz[x] += self.sz[y];
            }

            fn same(&mut self, x: usize, y: usize) -> bool {
                self.find(x) == self.find(y)
            }
        }

        let mut edges = Vec::with_capacity(n as usize + 1 + wells.len());
        for i in 0..wells.len() {
            edges.push((0, i + 1, wells[i]));
        }

        for i in 0..pipes.len() {
            edges.push((pipes[i][0] as usize, pipes[i][1] as usize, pipes[i][2]));
        }

        edges.sort_by_key(|x| x.2);

        let mut uf = UF::new(n as usize);

        let mut total_cost = 0;
        for edge in edges {
            let (h1, h2, cost) = (edge.0, edge.1, edge.2);
            if !uf.same(h1, h2) {
                uf.union(h1, h2);
                total_cost += cost;
            }
        }
        total_cost
    }

    min_cost_to_supply_water_kruskal(n, wells, pipes)
}
