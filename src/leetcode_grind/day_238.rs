// https://leetcode.com/problems/put-marbles-in-bags/description/
pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
    let n = weights.len();
    let mut pair_weights = vec![];
    for i in 0..n - 1 {
        pair_weights.push(weights[i] + weights[i + 1]);
    }
    pair_weights.sort();
    let mut ans = 0;
    for i in 0..k as usize - 1 {
        ans += pair_weights[n - 2 - i] as i64 - pair_weights[i] as i64;
    }
    ans
}

// https://leetcode.com/problems/longest-substring-with-at-most-two-distinct-characters/description/
pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
    use std::collections::HashMap;

    let n = s.len();
    if n < 3 {
        return n as i32;
    }

    let s = s.chars().collect::<Vec<_>>();

    let (mut left, mut right) = (0, 0);
    let mut hashmap = HashMap::new();

    let mut max_len = 2;

    while right < n {
        hashmap.insert(s[right], right);
        right += 1;

        if hashmap.len() == 3 {
            let min_idx = hashmap.values().copied().min().unwrap();
            hashmap.remove(&s[min_idx]);
            left = min_idx + 1;
        }

        max_len = max_len.max(right - left);
    }

    max_len as i32
}

// https://leetcode.com/problems/longest-substring-with-at-most-k-distinct-characters/description/
pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
    pub fn length_of_longest_substring_bin_search(s: String, k: i32) -> i32 {
        fn is_valid(s: &Vec<char>, size: i32, k: i32) -> bool {
            use std::collections::HashMap;
            let n = s.len();
            let mut counter = HashMap::new();

            for i in 0..size as usize {
                let c = s[i];
                *counter.entry(c).or_insert(0) += 1;
            }

            if counter.len() <= k as usize {
                return true;
            }

            for i in size as usize..n {
                let c1 = s[i];
                *counter.entry(c1).or_insert(0) += 1;

                let c2 = s[i - size as usize];
                *counter.entry(c2).or_insert(0) -= 1;

                if counter[&c2] == 0 {
                    counter.remove(&c2);
                }

                if counter.len() <= k as usize {
                    return true;
                }
            }

            false
        }

        let s = s.chars().collect::<Vec<_>>();
        let n = s.len();

        if k as usize >= n {
            return n as i32;
        }

        let (mut left, mut right) = (k, n as i32);

        while left < right {
            let mid = (left + right + 1) / 2;

            if is_valid(&s, mid, k) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        left
    }
    todo!()
}
