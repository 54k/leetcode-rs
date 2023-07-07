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
