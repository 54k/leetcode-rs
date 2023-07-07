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
    let mut res = 0;
    let n = word.len();
    for (i, ch) in word.chars().enumerate() {
        if ['a', 'e', 'i', 'o', 'u'].contains(&ch) {
            res += (i as i64 + 1) * (n as i64 - i as i64);
        }
    }
    res
}
