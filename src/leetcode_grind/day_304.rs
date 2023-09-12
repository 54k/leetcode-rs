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

// https://leetcode.com/problems/removing-minimum-and-maximum-from-array/description/
pub fn minimum_deletions_i(nums: Vec<i32>) -> i32 {
    let mut min_idx = 0;
    let mut max_idx = 0;

    for i in 0..nums.len() {
        if nums[i] < nums[min_idx] {
            min_idx = i;
        }
        if nums[i] > nums[max_idx] {
            max_idx = i;
        }
    }

    if min_idx > max_idx {
        std::mem::swap(&mut min_idx, &mut max_idx);
    }

    let mut ans = 0;

    let min_left = min_idx as i32 + 1;
    let mut min_right = nums.len() as i32 - min_idx as i32;

    let mut max_left = max_idx as i32 + 1;
    let max_right = nums.len() as i32 - max_idx as i32;

    ans += (min_left).min(max_right);

    if ans == min_left {
        max_left -= ans;
        ans += (max_left).min(max_right);
    } else {
        min_right -= ans;
        ans += (min_left).min(min_right);
    }

    ans
}

pub fn minimum_deletions_ii(nums: Vec<i32>) -> i32 {
    let mut idx_max = 0;
    let mut idx_min = 0;
    for i in 0..nums.len() {
        if nums[i] < nums[idx_max] {
            idx_min = i;
        }
        if nums[i] > nums[idx_max] {
            idx_max = i;
        }
    }

    let front_deletions = (idx_min + 1).max(idx_max + 1);
    let back_deletions = (nums.len() - idx_min).max(nums.len() - idx_max);
    let both_sides_deletions =
        (idx_min + 1).min(idx_max + 1) + (nums.len() - idx_min).min(nums.len() - idx_max);

    both_sides_deletions.min(front_deletions.min(back_deletions)) as i32
}

#[test]
fn test_min_dels() {
    let res = minimum_deletions_i(vec![
        -1, -53, 93, -42, 37, 94, 97, 82, 46, 42, -99, 56, -76, -66, -67, -13, 10, 66, 85, -28,
    ]);
    println!("{res}"); // 11
}

// https://leetcode.com/problems/minimum-deletions-to-make-array-beautiful/description/
pub fn min_deletion_i(nums: Vec<i32>) -> i32 {
    let mut stack = vec![];
    for &n in &nums {
        while stack.len() > 0 && (stack.len() % 2 == 1 && stack[stack.len() - 1] == n) {
            stack.pop();
        }
        stack.push(n);
    }
    if stack.len() % 2 == 1 {
        stack.pop();
    }
    nums.len() as i32 - stack.len() as i32
}

pub fn min_deletion_ii(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dels = 0;
    let mut i = 0;
    while i < n - 1 {
        if nums[i] == nums[i + 1] {
            dels += 1;
            i += 1;
        } else {
            i += 2;
        }
    }
    if (n - dels) % 2 == 1 {
        dels += 1;
    }
    dels as i32
}
