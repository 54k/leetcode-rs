// https://leetcode.com/problems/minimum-deletions-to-make-character-frequencies-unique/description/
pub fn min_deletions_i(s: String) -> i32 {
    use std::collections::HashSet;
    let mut freq = vec![0; 26];
    for ch in s.chars() {
        freq[ch as usize - 'a' as usize] += 1;
    }

    let mut min_deletions = 0;
    let mut seen_freq = HashSet::new();
    for i in 0..26 {
        while freq[i] > 0 && seen_freq.contains(&freq[i]) {
            freq[i] -= 1;
            min_deletions += 1;
        }
        seen_freq.insert(freq[i]);
    }

    min_deletions
}

pub fn min_deletions_ii(s: String) -> i32 {
    let mut freq = vec![0; 26];
    for ch in s.chars() {
        freq[ch as usize - 'a' as usize] += 1;
    }

    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::new();
    for i in 0..26 {
        if freq[i] > 0 {
            heap.push(freq[i]);
        }
    }

    let mut delete_count = 0;
    while heap.len() > 0 {
        let top = heap.pop().unwrap();
        if top == *heap.peek().unwrap_or(&0) {
            if top - 1 > 0 {
                heap.push(top - 1);
            }
            delete_count += 1;
        }
    }

    delete_count
}

pub fn min_deletions_iii(s: String) -> i32 {
    let mut freq = vec![0; 26];
    for ch in s.chars() {
        freq[ch as usize - 'a' as usize] += 1;
    }
    freq.sort();
    let mut max_freq_allowed = s.len() as i32;

    let mut deletions_count = 0;
    for i in (0..26).rev() {
        if freq[i] > max_freq_allowed {
            deletions_count += freq[i] - max_freq_allowed;
            freq[i] = max_freq_allowed;
        }
        max_freq_allowed = 0.max(freq[i] - 1);
    }
    deletions_count
}

// https://leetcode.com/problems/remove-letter-to-equalize-frequency/description/
pub fn equal_frequency(word: String) -> bool {
    let word = word.chars().collect::<Vec<_>>();
    let mut all_eq = true;
    for i in 0..word.len() {
        all_eq = true;
        let mut freq = vec![0; 26];
        for j in 0..word.len() {
            if i == j {
                continue;
            }
            freq[word[j] as usize - 'a' as usize] += 1;
        }

        let mut prev = -1;
        for k in 0..26 {
            if freq[k] == 0 {
                continue;
            }
            if prev == -1 {
                prev = freq[k];
            } else if prev != freq[k] {
                all_eq = false;
            }
        }
        if all_eq {
            return true;
        }
    }
    all_eq
}
