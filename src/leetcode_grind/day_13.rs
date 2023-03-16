#[allow(dead_code)]
pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    use std::collections::VecDeque;
    const MOD: i32 = 1000000007;

    fn sum_subarray_iter(arr: &Vec<i32>) -> i32 {
        let mut stack = VecDeque::new();
        let mut sum_of_minimums = 0;

        for i in 0..=arr.len() {
            while !stack.is_empty()
                && (i as usize == arr.len() || arr[*stack.back().unwrap() as usize] >= arr[i])
            {
                let mid = stack.pop_back().unwrap();
                let left_boundary = if stack.is_empty() {
                    -1i32
                } else {
                    *stack.back().unwrap()
                };
                let right_boundary = i as i32;

                let count = (mid - left_boundary) * (right_boundary - mid) % MOD;

                sum_of_minimums += (count * arr[mid as usize]) % MOD;
                sum_of_minimums %= MOD;
            }
            stack.push_back(i as i32);
        }

        sum_of_minimums
    }

    fn sum_subarray_dp(arr: &Vec<i32>) -> i32 {
        let mut dp = vec![0; arr.len()];
        let mut stack = VecDeque::new();

        for i in 0..arr.len() {
            while !stack.is_empty() && arr[*stack.back().unwrap() as usize] >= arr[i] {
                stack.pop_back();
            }

            if stack.is_empty() {
                dp[i] = (i + 1) as i32 * arr[i];
            } else {
                let j = *stack.back().unwrap() as i32;
                dp[i] = dp[j as usize] + (i as i32 - j) * arr[i];
            }

            stack.push_back(i as i32);
        }

        dp.iter().fold(0, |acc, v| (acc + v) % MOD) % MOD
    }
    println!("{}", sum_subarray_iter(&arr));
    sum_subarray_dp(&arr)
}

#[allow(dead_code)]
pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    fn pivot_array_no_vec_deq(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        use std::cmp::Ordering;
        let mut before_pivot = vec![];
        let mut after_pivot = vec![];

        let mut pivot_pos = 0;

        for num in nums {
            match num.cmp(&pivot) {
                Ordering::Less => before_pivot.push(num),
                Ordering::Equal => {
                    after_pivot.push(0);
                    let mut prev = after_pivot[pivot_pos];
                    for i in pivot_pos..after_pivot.len() - 1 {
                        std::mem::swap(&mut after_pivot[i + 1], &mut prev);
                    }
                    after_pivot[pivot_pos] = pivot;
                    pivot_pos += 1;
                }
                Ordering::Greater => after_pivot.push(num),
            }
        }

        before_pivot.extend(after_pivot);
        before_pivot
    }

    fn pivot_array_vec_deq(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        use std::cmp::Ordering;
        use std::collections::VecDeque;
        let mut before_pivot = vec![];
        let mut after_pivot = VecDeque::new();

        for num in nums {
            match num.cmp(&pivot) {
                Ordering::Less => before_pivot.push(num),
                Ordering::Equal => {
                    after_pivot.push_front(num);
                }
                Ordering::Greater => after_pivot.push_back(num),
            }
        }

        before_pivot.extend(after_pivot);
        before_pivot
    }

    fn pivot_array_count_pivots(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        use std::cmp::Ordering;
        let mut result = vec![];
        let mut after_pivot = vec![];
        let mut pivot_num = 0;
        for num in nums {
            match num.cmp(&pivot) {
                Ordering::Less => result.push(num),
                Ordering::Equal => pivot_num += 1,
                Ordering::Greater => after_pivot.push(num),
            }
        }
        for _ in 0..pivot_num {
            result.push(pivot);
        }

        result.extend(after_pivot);
        result
    }

    pivot_array_count_pivots(nums, pivot)
}

#[allow(dead_code)]
pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    //noinspection ALL
    fn find_substring_map_scan(s: String, words: Vec<String>) -> Vec<i32> {
        use std::collections::hash_map::Entry;
        use std::collections::HashMap;

        let word_size = words[0].len();
        let substring_size = words[0].len() * words.len();

        let words = words.iter().fold(HashMap::new(), |mut acc, v| {
            if let Entry::Vacant(e) = acc.entry(v.as_str()) {
                e.insert(1);
            } else {
                *acc.get_mut(v.as_str()).unwrap() += 1;
            }
            acc
        });

        fn check(
            s: &str,
            mut words: HashMap<&str, i32>,
            i: usize,
            word_size: usize,
            substring_size: usize,
        ) -> bool {
            for j in (i..substring_size + i).step_by(word_size) {
                let w = &s[j..j + word_size];
                if words.contains_key(w) {
                    let val = words.get_mut(w).unwrap();
                    *val -= 1;
                    if *val < 0 {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            true
        }

        let mut res = vec![];
        for i in 0..s.len() - substring_size + 1 {
            if check(&s, words.clone(), i, word_size, substring_size) {
                res.push(i as i32)
            }
        }

        res
    }

    //noinspection ALL
    fn find_substring_sliding_window(s: String, words: Vec<String>) -> Vec<i32> {
        use std::collections::hash_map::Entry;
        use std::collections::HashMap;

        let word_length = words[0].len();
        let substring_size = words[0].len() * words.len();

        if word_length > s.len() {
            return vec![];
        }

        let words_count = words.iter().fold(HashMap::new(), |mut acc, v| {
            if let Entry::Vacant(e) = acc.entry(v.as_str()) {
                e.insert(1);
            } else {
                *acc.get_mut(v.as_str()).unwrap() += 1;
            }
            acc
        });

        fn sliding_window(
            mut left: usize,
            s: &str,
            substring_size: usize,
            words_count: &HashMap<&str, i32>,
            word_length: usize,
            words_length: usize,
            res: &mut Vec<i32>,
        ) {
            let mut words_found = HashMap::new();
            let mut words_used = 0;
            let mut excess_word = false;

            for right in (left..=s.len() - word_length).step_by(word_length) {
                let sub = &s[right..right + word_length];

                if !words_count.contains_key(sub) {
                    words_found.clear();
                    words_used = 0;
                    excess_word = false;
                    left = right + word_length;
                } else {
                    // If we reached max window size or have an excess word
                    while right - left == substring_size || excess_word {
                        let leftmost_word = &s[left..left + word_length];
                        left += word_length;

                        let wf = words_found.get_mut(leftmost_word).unwrap();
                        *wf -= 1;

                        if *wf >= *words_count.get(leftmost_word).unwrap_or(&0) {
                            // This word was an excess word
                            excess_word = false;
                        } else {
                            // Otherwise we actually needed it
                            words_used -= 1;
                        }
                    }

                    // Keep track of how many times this word occurs in the window
                    if let Entry::Vacant(e) = words_found.entry(sub) {
                        e.insert(1);
                    } else {
                        *words_found.get_mut(sub).unwrap() += 1;
                    }

                    if words_found.get(sub) <= words_count.get(sub) {
                        words_used += 1;
                    } else {
                        // Found too many instances already
                        excess_word = true;
                    }

                    if words_used == words_length && !excess_word {
                        res.push(left as i32);
                    }
                }
            }
        }

        let mut res = vec![];
        for left in 0..word_length {
            sliding_window(
                left,
                &s,
                substring_size,
                &words_count,
                word_length,
                words.len(),
                &mut res,
            );
        }
        res
    }

    find_substring_sliding_window(s, words)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test75() {
        println!("{:?}", sum_subarray_mins(vec![2, 2, 2]));
    }

    #[test]
    fn test76() {
        println!("{:?}", pivot_array(vec![9, 12, 5, 10, 14, 3, 10], 10)); //[9,5,3,10,10,12,14]
        println!("{:?}", pivot_array(vec![-3, 4, 3, 2], 2)) //[-3,2,4,3]
    }

    #[test]
    fn test77() {
        println!(
            "{:?}",
            find_substring(
                "barfoothefoobarman".to_owned(),
                vec!["foo".to_owned(), "bar".to_owned()],
            )
        ); //[0,9]

        println!(
            "{:?}",
            find_substring(
                "wordgoodgoodgoodbestword".to_owned(),
                vec![
                    "word".to_owned(),
                    "good".to_owned(),
                    "best".to_owned(),
                    "word".to_owned(),
                ],
            )
        ); //[]

        println!(
            "{:?}",
            find_substring(
                "wordgoodgoodgoodbestword".to_owned(),
                vec![
                    "word".to_owned(),
                    "good".to_owned(),
                    "best".to_owned(),
                    "good".to_owned(),
                ],
            )
        ); //[8]

        println!(
            "{:?}",
            find_substring("mississippi".to_owned(), vec!["mississippis".to_owned()])
        ); //[]
    }
}
