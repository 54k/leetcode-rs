// https://leetcode.com/problems/alien-dictionary/
pub fn alien_order(words: Vec<String>) -> String {
    use std::collections::{HashMap, VecDeque};
    let mut adj = HashMap::new();
    let mut count = HashMap::new();

    for word in &words {
        for ch in word.chars() {
            count.entry(ch).or_insert(0);
            adj.entry(ch).or_insert(vec![]);
        }
    }

    for i in 0..words.len() - 1 {
        let word1 = words[i].clone().chars().collect::<Vec<_>>();
        let word2 = words[i + 1].clone().chars().collect::<Vec<_>>();

        if word1.len() > word2.len() && word1.starts_with(&word2) {
            return "".to_string();
        }

        for j in 0..word1.len().min(word2.len()) {
            if word1[j] != word2[j] {
                adj.entry(word1[j]).or_insert(vec![]).push(word2[j]);
                *count.entry(word2[j]).or_insert(0) += 1;
                break;
            }
        }
    }

    let mut queue = VecDeque::new();
    for (k, &v) in &count {
        if v == 0 {
            queue.push_back(*k);
        }
    }
    let mut ans = vec![];
    while let Some(ch) = queue.pop_front() {
        ans.push(ch);
        for &next in &adj[&ch] {
            *count.entry(next).or_insert(0) -= 1;
            if count[&next] == 0 {
                queue.push_back(next);
            }
        }
    }
    if ans.len() != count.len() {
        return "".to_string();
    }
    ans.into_iter().collect()
}

// https://leetcode.com/problems/alien-dictionary/
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    pub fn find_kth_largest_quickselect(nums: Vec<i32>, k: i32) -> i32 {
        extern "C" {
            fn random() -> i32;
        }
        fn quickselect(nums: &Vec<i32>, k: i32) -> i32 {
            let pivot_idx = unsafe { random() % nums.len() as i32 } as usize;
            let pivot = nums[pivot_idx];

            let mut left = vec![];
            let mut mid = vec![];
            let mut right = vec![];

            for &num in nums {
                if num > pivot {
                    left.push(num);
                } else if num == pivot {
                    mid.push(num);
                } else {
                    right.push(num);
                }
            }

            if k <= left.len() as i32 {
                return quickselect(&left, k);
            }

            if (left.len() as i32 + mid.len() as i32) < k {
                return quickselect(&right, k - left.len() as i32 - mid.len() as i32);
            }

            pivot
        }
        quickselect(&nums, k)
    }

    pub fn find_kth_largest_heap(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::BinaryHeap;
        let mut heap = BinaryHeap::new();
        for i in 0..nums.len() {
            heap.push(-nums[i]);
            if (heap.len() as i32) > k {
                heap.pop();
            }
        }
        -heap.pop().unwrap()
    }

    find_kth_largest_quickselect(nums, k)
}
