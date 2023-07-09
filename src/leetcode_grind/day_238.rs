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

    pub fn length_of_longest_substring_k_distinct_sliding_window1(s: String, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut counter = HashMap::new();
        let n = s.len();
        let s = s.chars().collect::<Vec<_>>();
        let mut max_size = 0;
        let mut left = 0;
        for right in 0..n {
            *counter.entry(s[right]).or_insert(0) += 1;

            while counter.len() > k as usize {
                let c = s[left];
                *counter.get_mut(&c).unwrap() -= 1;
                if counter[&c] == 0 {
                    counter.remove(&c);
                }
                left += 1;
            }
            max_size = max_size.max(right - left + 1);
        }

        max_size as i32
    }

    pub fn length_of_longest_substring_k_distinct_sliding_window2(s: String, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut counter = HashMap::new();
        let n = s.len();
        let s = s.chars().collect::<Vec<_>>();
        let mut max_size = 0;

        for right in 0..n {
            *counter.entry(s[right]).or_insert(0) += 1;
            if counter.len() <= k as usize {
                max_size += 1;
            } else {
                *counter.entry(s[right - max_size]).or_insert(0) -= 1;
                if counter[&s[right - max_size]] == 0 {
                    counter.remove(&s[right - max_size]);
                }
            }
        }

        max_size as i32
    }

    length_of_longest_substring_k_distinct_sliding_window2(s, k)
}

// https://leetcode.com/problems/flip-string-to-monotone-increasing/description/
pub fn min_flips_mono_incr(s: String) -> i32 {
    pub fn min_flips_mono_incr_iterative(s: String) -> i32 {
        let mut m = 0;
        for ch in s.chars() {
            if ch == '0' {
                m += 1;
            }
        }

        let mut ans = m;

        for ch in s.chars() {
            if ch == '1' {
                m += 1;
            } else {
                m -= 1;
            }
            ans = ans.min(m);
        }

        ans
    }
    pub fn min_flips_mono_incr_dp(s: String) -> i32 {
        let mut ans = 0;
        let s = s.chars().collect::<Vec<_>>();
        let mut num = 0;
        for i in 0..s.len() {
            if s[i] == '0' {
                ans = num.min(ans + 1);
            } else {
                num += 1;
            }
        }
        ans
    }
    min_flips_mono_incr_dp(s)
}

// https://leetcode.com/problems/sort-the-jumbled-numbers/description/
pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;
    let mapping: HashMap<_, _> = mapping
        .iter()
        .copied()
        .enumerate()
        .map(|(i, x)| (i as i32, x))
        .collect();

    fn transform(n: i32, mapping: &HashMap<i32, i32>) -> i32 {
        let nn: Vec<i32> = n
            .to_string()
            .chars()
            .map(|x| format!("{}", x).parse::<i32>().unwrap())
            .collect();

        let mut ans = 0;

        for i in nn {
            ans = ans * 10 + mapping[&i];
        }

        // println!("given {} then {}", n, ans);
        ans
    }

    let mut nums: Vec<_> = nums
        .into_iter()
        .enumerate()
        .map(|(i, x)| (transform(x, &mapping), i, x))
        .collect();

    nums.sort();
    nums.into_iter().map(|(_, _, x)| x).collect()
}
